//! Tauri commands: the only door between the UI and the outside. Every NBS call, token
//! and password stays on this (Rust) side, and so does the one other request: the update check.

mod creds;
mod nbs;
mod shifts;
mod summary;

use std::time::Duration;

use chrono::{Datelike, NaiveDate};
use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_updater::{Update, UpdaterExt};

use nbs::{AppError, FileOutcome, FileRequest, Me, Nbs, Profile, Res};
use summary::PeriodView;

#[tauri::command]
async fn server_reachable(nbs: State<'_, Nbs>) -> Res<bool> {
    Ok(nbs.reachable().await)
}

/// Signed in already, or silently with the remembered login; `None` means show sign-in.
#[tauri::command]
async fn restore_session(nbs: State<'_, Nbs>) -> Res<Option<Me>> {
    if let Some(me) = nbs.me().await {
        return Ok(Some(me));
    }
    let Some((username, password)) = creds::load() else {
        return Ok(None);
    };
    match nbs.login(&username, &password).await {
        Ok(me) => Ok(Some(me)),
        Err(e) if e.kind == "auth" => {
            creds::clear(); // password changed in NBS: forget it, ask again
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

#[tauri::command]
async fn login(nbs: State<'_, Nbs>, username: String, password: String, remember: bool) -> Res<Me> {
    let username = username.trim();
    if username.is_empty() || password.is_empty() {
        return Err(AppError::input("Enter your NBS username and password."));
    }
    let me = nbs.login(username, &password).await?;
    if remember {
        creds::save(username, &password);
    } else {
        creds::clear();
    }
    Ok(me)
}

#[tauri::command]
async fn logout(nbs: State<'_, Nbs>) -> Res<()> {
    nbs.logout().await;
    creds::clear();
    Ok(())
}

#[tauri::command]
async fn profile(nbs: State<'_, Nbs>) -> Res<Profile> {
    nbs.profile().await
}

#[tauri::command]
async fn load_period(nbs: State<'_, Nbs>, year: i32, month: u32, half: u32) -> Res<PeriodView> {
    let p = summary::make_period(year, month, half).ok_or_else(|| AppError::input("That payroll period doesn't exist."))?;
    let (from, to) = (p.start.format("%Y-%m-%d").to_string(), p.end.format("%Y-%m-%d").to_string());
    let obs = nbs.ob_filings(&from, &to).await?;
    let css = nbs.cs_filings(&from, &to).await?;
    let leaves = nbs.leave_filings(&from, &to).await?;
    Ok(summary::period_view(&p, &obs, &css, &leaves, chrono::Local::now().date_naive()))
}

#[derive(Serialize)]
struct ShiftOption {
    id: i64,
    name: String,
    /// "HH:MM" the shift implies; empty when its name has no readable times.
    time_in: String,
    time_out: String,
}

#[tauri::command]
async fn shifts_for(nbs: State<'_, Nbs>, date: String) -> Res<Vec<ShiftOption>> {
    let d = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|_| AppError::input("Pick a date."))?;
    let all = nbs.shifts().await?;
    Ok(shifts::filter_shifts(&all, d.weekday().num_days_from_monday())
        .into_iter()
        .map(|s| {
            let p = shifts::parse_shift_times(&s.name);
            ShiftOption {
                id: s.id,
                time_in: p.map(|p| shifts::hhmm(p.time_in)).unwrap_or_default(),
                time_out: p.map(|p| shifts::hhmm(p.time_out)).unwrap_or_default(),
                name: s.name,
            }
        })
        .collect())
}

/// Customer names for the Settings suggestions.
#[tauri::command]
async fn customers(nbs: State<'_, Nbs>) -> Res<Vec<String>> {
    Ok(nbs.customers().await?.into_iter().map(|c| c.customer_name.trim().to_string()).collect())
}

#[tauri::command]
async fn file_day(nbs: State<'_, Nbs>, request: FileRequest) -> Res<FileOutcome> {
    nbs.file_day(&request).await
}

#[tauri::command]
async fn delete_filings(nbs: State<'_, Nbs>, date: String, kinds: Vec<String>) -> Res<Vec<String>> {
    nbs.delete_day(&date, &kinds).await
}

/// Reads latest.json from the newest GitHub release. The only request NBSBuddy makes that
/// isn't to NBS, and it sends nothing about the user.
async fn find_update(app: &AppHandle) -> Res<Option<Update>> {
    let failed = |e| AppError::update(format!("Couldn't check for updates: {}.", said(e)));
    app.updater_builder().timeout(Duration::from_secs(8)).build().map_err(failed)?.check().await.map_err(failed)
}

/// The newer NBSBuddy version on GitHub, or `None` when this copy is up to date.
#[tauri::command]
async fn check_update(app: AppHandle) -> Res<Option<String>> {
    Ok(find_update(&app).await?.map(|u| u.version))
}

/// Downloads the update, checks its signature against the public key in tauri.conf.json,
/// and runs the installer. NBSBuddy closes and opens again on the new version.
#[tauri::command]
async fn install_update(app: AppHandle) -> Res<()> {
    let update = find_update(&app).await?.ok_or_else(|| AppError::update("NBSBuddy is already up to date."))?;
    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|e| AppError::update(format!("Couldn't install the update: {}. Nothing was changed.", said(e))))?;
    app.restart()
}

/// The updater's error as one sentence without its own full stop, so ours can follow it.
fn said(e: tauri_plugin_updater::Error) -> String {
    e.to_string().trim_end_matches('.').to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(Nbs::new())
        .invoke_handler(tauri::generate_handler![
            server_reachable,
            restore_session,
            login,
            logout,
            profile,
            load_period,
            shifts_for,
            customers,
            file_day,
            delete_filings,
            check_update,
            install_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running NBSBuddy");
}

//! The NBS API client: login + token refresh, filings, create, void (delete), profile.
//! Leave filings are only read, never filed or voided.
//!
//! Endpoints were taken from NBS's own web app. NBS is plain HTTP and only reachable
//! over the company ZeroTier network.

use std::time::Duration;

use base64::Engine;
use serde::de::{DeserializeOwned, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::Mutex;

use crate::creds;
use crate::shifts::{Shift, hhmm, parse_shift_times};

pub const API_BASE: &str = "http://172.16.50.4:13641";
pub const OB: &str = "official_business_filing";
pub const CS: &str = "change_shift_filing";
pub const LEAVE: &str = "leave_filing";
const CUSTOMER: &str = "315 CORPORATE OFFICE";
pub const PENDING: &str = "For Approval";

// ---- errors ----

/// What the UI gets on failure. `kind`: "auth" (show sign-in), "offline" (ZeroTier),
/// "nbs" (NBS refused), "input" (fix the form), "update" (GitHub update check or install).
#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub kind: &'static str,
    pub message: String,
}

impl AppError {
    fn new(kind: &'static str, message: impl Into<String>) -> Self {
        Self { kind, message: message.into() }
    }
    pub fn auth(message: impl Into<String>) -> Self {
        Self::new("auth", message)
    }
    pub fn input(message: impl Into<String>) -> Self {
        Self::new("input", message)
    }
    pub fn nbs(message: impl Into<String>) -> Self {
        Self::new("nbs", message)
    }
    pub fn update(message: impl Into<String>) -> Self {
        Self::new("update", message)
    }
    fn signed_out() -> Self {
        Self::auth("Your NBS session ended. Sign in again.")
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_connect() || e.is_timeout() {
            Self::new("offline", "Can't reach NBS. Check that ZeroTier is connected, then try again.")
        } else {
            Self::nbs(format!("NBS request failed: {e}"))
        }
    }
}

pub type Res<T> = Result<T, AppError>;

// ---- lenient JSON: NBS sends null, numbers-as-strings, and strings-as-numbers ----

fn text<'de, D: Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Null => String::new(),
        Value::String(s) => s,
        v => v.to_string(),
    })
}

fn int<'de, D: Deserializer<'de>>(d: D) -> Result<i64, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|f| f as i64)).unwrap_or(0),
        Value::String(s) => s.trim().parse().unwrap_or(0),
        Value::Bool(b) => b as i64,
        _ => 0,
    })
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ObFiling {
    #[serde(default, deserialize_with = "int")]
    pub id: i64,
    #[serde(default, deserialize_with = "text")]
    pub date: String,
    #[serde(default, deserialize_with = "text")]
    pub customer_name: String,
    #[serde(default, deserialize_with = "text")]
    pub time_in: String,
    #[serde(default, deserialize_with = "text")]
    pub time_out: String,
    #[serde(default, deserialize_with = "text")]
    pub hours_filed: String,
    #[serde(default, deserialize_with = "text")]
    pub reason: String,
    #[serde(default, deserialize_with = "text")]
    pub status: String,
    #[serde(default, deserialize_with = "int")]
    pub is_voided: i64,
    #[serde(default, deserialize_with = "int")]
    pub is_posted: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CsFiling {
    #[serde(default, deserialize_with = "int")]
    pub id: i64,
    #[serde(default, deserialize_with = "text")]
    pub date: String,
    #[serde(default, deserialize_with = "text")]
    pub new_shift_schedule: String,
    #[serde(default, deserialize_with = "text")]
    pub reason: String,
    #[serde(default, deserialize_with = "text")]
    pub status: String,
    #[serde(default, deserialize_with = "int")]
    pub is_voided: i64,
    #[serde(default, deserialize_with = "int")]
    pub is_posted: i64,
}

/// One row per date. NBS sends more (balance, approver, ...); only these are read.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LeaveFiling {
    #[serde(default, deserialize_with = "text")]
    pub date: String,
    #[serde(default, deserialize_with = "text")]
    pub leave_type: String,
    /// "1.000000" for a whole day, "0.500000" for a half day.
    #[serde(default, deserialize_with = "text")]
    pub days: String,
    #[serde(default, deserialize_with = "text")]
    pub reason: String,
    #[serde(default, deserialize_with = "text")]
    pub status: String,
    #[serde(default, deserialize_with = "int")]
    pub is_voided: i64,
}

/// Only "For Approval" filings that payroll hasn't posted may be deleted (voided).
pub fn is_pending(status: &str, is_posted: i64) -> bool {
    status.trim().eq_ignore_ascii_case(PENDING) && is_posted == 0
}

/// The work-facing slice of `GET /my_profile`. NBS also returns salary, bank and
/// government IDs; they are deliberately not deserialized, so they never reach the UI.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Profile {
    #[serde(deserialize_with = "text")]
    pub employee_name: String,
    #[serde(deserialize_with = "text")]
    pub first_name: String,
    #[serde(deserialize_with = "text")]
    pub last_name: String,
    #[serde(deserialize_with = "text")]
    pub nickname: String,
    #[serde(deserialize_with = "text")]
    pub employee_code: String,
    #[serde(deserialize_with = "text")]
    pub position: String,
    #[serde(deserialize_with = "text")]
    pub department: String,
    #[serde(deserialize_with = "text")]
    pub division: String,
    #[serde(deserialize_with = "text")]
    pub team: String,
    #[serde(deserialize_with = "text")]
    pub branch: String,
    #[serde(deserialize_with = "text")]
    pub work_location: String,
    #[serde(deserialize_with = "text")]
    pub pay_group: String,
    #[serde(deserialize_with = "text")]
    pub employment_type: String,
    #[serde(deserialize_with = "text")]
    pub date_hired: String,
    #[serde(deserialize_with = "text")]
    pub employment_tenure: String,
    #[serde(deserialize_with = "text")]
    pub immediate_superior_employee: String,
    #[serde(deserialize_with = "text")]
    pub work_email_address: String,
}

// ---- identity (from the access-token JWT) ----

#[derive(Debug, Clone, Serialize)]
pub struct Me {
    pub employee_id: i64,
    pub company_id: i64,
    /// The name NBS expects inside filing payloads.
    pub employee_name: String,
    pub first_name: String,
    pub display_name: String,
    pub employee_code: String,
    pub position: String,
    pub department: String,
}

fn claims(token: &str) -> Option<Value> {
    let payload = token.split('.').nth(1)?;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload.trim_end_matches('='))
        .ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn expiring(token: &str, margin_secs: i64) -> bool {
    let exp = claims(token).and_then(|c| c["exp"].as_i64()).unwrap_or(0);
    exp - chrono::Utc::now().timestamp() <= margin_secs
}

fn me_from_token(token: &str) -> Option<Me> {
    let c = claims(token)?;
    let s = |k: &str| c[k].as_str().unwrap_or("").trim().to_string();
    let n = |k: &str| c[k].as_i64().or_else(|| c[k].as_str()?.parse().ok());
    let (first, last) = (s("first_name"), s("last_name"));
    let full = format!("{first} {last}").trim().to_string();
    let employee_name = [s("user_name"), s("employee_name"), full.clone()]
        .into_iter()
        .find(|v| !v.is_empty())
        .unwrap_or_default();
    Some(Me {
        employee_id: n("employee_id")?,
        company_id: n("company_id")?,
        display_name: if full.is_empty() { employee_name.clone() } else { full },
        employee_name,
        first_name: first,
        employee_code: s("employee_code"),
        position: s("position"),
        department: s("department"),
    })
}

// ---- client ----

struct Session {
    access: String,
    refresh: String,
    me: Me,
}

pub struct Nbs {
    http: reqwest::Client,
    session: Mutex<Option<Session>>,
    shifts: Mutex<Option<Vec<Shift>>>, // ~1100 schedules; fetched once per sign-in
}

/// One kind's (OB or CS) result of a filing attempt.
#[derive(Debug, Clone, Serialize)]
pub struct KindOutcome {
    /// "created", "skipped" (already filed) or "failed".
    pub state: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileOutcome {
    pub ob: KindOutcome,
    pub cs: KindOutcome,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileRequest {
    pub date: String,
    pub shift_id: i64,
    pub shift_name: String,
    pub time_in: String,
    pub time_out: String,
    pub ob_reason: String,
    pub cs_reason: String,
    /// NBS customer the OB is filed against; blank means the built-in one.
    #[serde(default)]
    pub customer: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Customer {
    #[serde(deserialize_with = "int")]
    pub id: i64,
    #[serde(deserialize_with = "text")]
    pub customer_name: String,
}

/// The id of the customer named `name` (any case), or of the built-in one when blank.
fn find_customer(all: &[Customer], name: &str) -> Res<i64> {
    let name = Some(name.trim()).filter(|n| !n.is_empty()).unwrap_or(CUSTOMER);
    all.iter()
        .find(|c| c.customer_name.trim().eq_ignore_ascii_case(name))
        .map(|c| c.id)
        .ok_or_else(|| AppError::nbs(format!("NBS has no customer named {name}. Pick another one in Settings.")))
}

fn filings_query(from: &str, to: &str) -> String {
    format!(
        "id=null&approval_status_id=0&is_approver=0&employee_id=null&from_date={from}&to_date={to}&date_range_id=9"
    )
}

async fn nbs_message(r: reqwest::Response) -> String {
    let status = r.status();
    let body = r.text().await.unwrap_or_default();
    let said = serde_json::from_str::<Value>(&body)
        .ok()
        .and_then(|v| v["message"].as_str().map(str::to_string))
        .unwrap_or(body);
    let said: String = said.trim().chars().take(300).collect();
    if said.is_empty() { format!("NBS answered HTTP {}.", status.as_u16()) } else { said }
}

impl Nbs {
    pub fn new() -> Self {
        let http = reqwest::Client::builder()
            .no_proxy() // NBS is on ZeroTier; the updater's system-proxy feature must not route it to a proxy
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(20))
            .build()
            .expect("HTTP client");
        Self { http, session: Mutex::new(None), shifts: Mutex::new(None) }
    }

    pub async fn reachable(&self) -> bool {
        self.http.get(format!("{API_BASE}/")).timeout(Duration::from_secs(4)).send().await.is_ok()
    }

    async fn sign_in(&self, username: &str, password: &str) -> Res<Session> {
        let r = self
            .http
            .post(format!("{API_BASE}/login"))
            .json(&json!({"username": username, "password": password,
                          "is_mobile": "0", "is_keep_me_logged_in": "0"}))
            .send()
            .await?;
        if !r.status().is_success() {
            return Err(AppError::auth(nbs_message(r).await));
        }
        let header = |name: &str| r.headers().get(name).and_then(|v| v.to_str().ok()).map(str::to_string);
        let access = header("x-auth-token").ok_or_else(|| AppError::nbs("NBS signed you in but sent no token."))?;
        let refresh = header("x-auth-refresh-token").unwrap_or_default();
        let me = me_from_token(&access).ok_or_else(|| AppError::nbs("NBS sent a token NBSBuddy can't read."))?;
        Ok(Session { access, refresh, me })
    }

    pub async fn login(&self, username: &str, password: &str) -> Res<Me> {
        let session = self.sign_in(username, password).await?;
        let me = session.me.clone();
        *self.session.lock().await = Some(session);
        *self.shifts.lock().await = None;
        Ok(me)
    }

    pub async fn logout(&self) {
        *self.session.lock().await = None;
        *self.shifts.lock().await = None;
    }

    pub async fn me(&self) -> Option<Me> {
        self.session.lock().await.as_ref().map(|s| s.me.clone())
    }

    /// A fresh access token: refreshed when about to expire, and if the refresh token
    /// is dead too, re-signed-in with the remembered login (if any).
    async fn token(&self) -> Res<String> {
        let mut guard = self.session.lock().await;
        let session = guard.as_mut().ok_or_else(AppError::signed_out)?;
        if !expiring(&session.access, 60) {
            return Ok(session.access.clone());
        }
        let refreshed = self
            .http
            .post(format!("{API_BASE}/token"))
            .json(&json!({"grant_type": "refresh_token", "refresh_token": session.refresh, "is_mobile": 0}))
            .send()
            .await?;
        if refreshed.status().is_success() {
            let body: Value = refreshed.json().await.unwrap_or(Value::Null);
            if let Some(access) = body["access_token"].as_str() {
                session.access = access.to_string();
                return Ok(session.access.clone());
            }
        }
        match creds::load() {
            Some((username, password)) => {
                *session = self.sign_in(&username, &password).await?;
                Ok(session.access.clone())
            }
            None => {
                *guard = None;
                Err(AppError::signed_out())
            }
        }
    }

    async fn checked(r: reqwest::Response) -> Res<reqwest::Response> {
        match r.status().as_u16() {
            200..=299 => Ok(r),
            401 => Err(AppError::signed_out()),
            _ => Err(AppError::nbs(nbs_message(r).await)),
        }
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Res<T> {
        let token = self.token().await?;
        let r = self.http.get(format!("{API_BASE}/{path}")).header("x-auth-token", token).send().await?;
        Ok(Self::checked(r).await?.json().await?)
    }

    /// POST that reports NBS's refusal as `Ok(Err(message))` so one kind failing
    /// doesn't hide the other's result; network failures are still `Err`.
    async fn post(&self, path: &str, body: &Value) -> Res<Result<(), String>> {
        let token = self.token().await?;
        let r = self.http.post(format!("{API_BASE}/{path}")).header("x-auth-token", token).json(body).send().await?;
        if r.status().as_u16() == 401 {
            return Err(AppError::signed_out());
        }
        Ok(if r.status().is_success() { Ok(()) } else { Err(nbs_message(r).await) })
    }

    pub async fn ob_filings(&self, from: &str, to: &str) -> Res<Vec<ObFiling>> {
        self.get(&format!("{OB}/get_filings?{}", filings_query(from, to))).await
    }

    pub async fn cs_filings(&self, from: &str, to: &str) -> Res<Vec<CsFiling>> {
        self.get(&format!("{CS}/get_filings?{}", filings_query(from, to))).await
    }

    pub async fn leave_filings(&self, from: &str, to: &str) -> Res<Vec<LeaveFiling>> {
        self.get(&format!("{LEAVE}/get_filings?{}", filings_query(from, to))).await
    }

    pub async fn shifts(&self) -> Res<Vec<Shift>> {
        if let Some(cached) = self.shifts.lock().await.clone() {
            return Ok(cached);
        }
        let fresh: Vec<Shift> = self.get("shift_schedules").await?;
        *self.shifts.lock().await = Some(fresh.clone());
        Ok(fresh)
    }

    pub async fn profile(&self) -> Res<Profile> {
        let mut list: Vec<Profile> = self.get("my_profile?employee_id=").await?;
        if list.is_empty() {
            return Err(AppError::nbs("NBS returned no profile for this account."));
        }
        Ok(list.swap_remove(0))
    }

    pub async fn customers(&self) -> Res<Vec<Customer>> {
        self.get("customers").await
    }

    /// File OB + CS for one date, skipping whichever kind NBS already has.
    pub async fn file_day(&self, req: &FileRequest) -> Res<FileOutcome> {
        let me = self.me().await.ok_or_else(AppError::signed_out)?;
        let (time_in, time_out, hours) = validate(req)?;
        let date = req.date.as_str();
        let has_ob = self.ob_filings(date, date).await?.iter().any(|f| f.date.starts_with(date) && f.is_voided == 0);
        let has_cs = self.cs_filings(date, date).await?.iter().any(|f| f.date.starts_with(date) && f.is_voided == 0);

        let outcome = |r: Result<(), String>| match r {
            Ok(()) => KindOutcome { state: "created", message: String::new() },
            Err(m) => KindOutcome { state: "failed", message: m },
        };
        let skipped = || KindOutcome { state: "skipped", message: "Already filed.".into() };

        let ob = if has_ob {
            skipped()
        } else {
            let customer_id = find_customer(&self.customers().await?, &req.customer)?;
            let payload = ob_payload(&me, date, customer_id, &time_in, &time_out, hours, &req.ob_reason);
            outcome(self.post(OB, &payload).await?)
        };
        let cs = if has_cs {
            skipped()
        } else {
            outcome(self.post(CS, &cs_payload(&me, date, req.shift_id, &req.shift_name, &req.cs_reason)).await?)
        };
        Ok(FileOutcome { ob, cs })
    }

    /// Void (NBS's "delete") the date's pending OB and/or CS. Checks every requested kind
    /// before voiding any, so a locked filing never leaves the day half-deleted.
    pub async fn delete_day(&self, date: &str, kinds: &[String]) -> Res<Vec<String>> {
        let want = |k: &str| kinds.iter().any(|x| x == k);
        let mut targets: Vec<(&str, i64)> = Vec::new();
        if want("OB") {
            let obs = self.ob_filings(date, date).await?;
            let f = obs.iter().find(|f| f.date.starts_with(date) && f.is_voided == 0);
            targets.push(("OB", deletable("OB", f.map(|f| (f.id, f.status.as_str(), f.is_posted)))?));
        }
        if want("CS") {
            let css = self.cs_filings(date, date).await?;
            let f = css.iter().find(|f| f.date.starts_with(date) && f.is_voided == 0);
            targets.push(("CS", deletable("CS", f.map(|f| (f.id, f.status.as_str(), f.is_posted)))?));
        }
        if targets.is_empty() {
            return Err(AppError::input("Pick OB, CS or both to delete."));
        }
        let mut deleted = Vec::new();
        for (kind, id) in targets {
            let path = format!("{}/void", if kind == "OB" { OB } else { CS });
            if let Err(m) = self.post(&path, &json!([{ "id": id }])).await? {
                let done = if deleted.is_empty() { String::new() } else { format!(" ({} was deleted.)", deleted.join(" + ")) };
                return Err(AppError::nbs(format!("NBS didn't delete the {kind}: {m}{done}")));
            }
            deleted.push(kind.to_string());
        }
        Ok(deleted)
    }
}

fn deletable(kind: &str, filing: Option<(i64, &str, i64)>) -> Res<i64> {
    match filing {
        None => Err(AppError::input(format!("There's no {kind} on this date to delete."))),
        Some((id, status, posted)) if is_pending(status, posted) => Ok(id),
        Some((_, status, _)) => Err(AppError::input(format!(
            "The {kind} is {status}, so it can't be deleted. Only filings still {PENDING} can be."
        ))),
    }
}

/// "7:30" / "07:30" / "07:30:00" -> seconds after midnight.
fn parse_clock(text: &str) -> Option<u32> {
    let parts: Vec<&str> = text.trim().split(':').collect();
    if !(2..=3).contains(&parts.len()) || parts.iter().any(|p| p.is_empty() || p.len() > 2) {
        return None;
    }
    let n: Vec<u32> = parts.iter().map(|p| p.parse().ok()).collect::<Option<_>>()?;
    let (h, m, s) = (n[0], n[1], n.get(2).copied().unwrap_or(0));
    (h < 24 && m < 60 && s < 60).then_some(h * 3600 + m * 60 + s)
}

/// Guards the write path: (time_in "HH:MM:SS", time_out, hours) or the first problem.
pub fn validate(req: &FileRequest) -> Res<(String, String, f64)> {
    if chrono::NaiveDate::parse_from_str(&req.date, "%Y-%m-%d").is_err() {
        return Err(AppError::input("Pick a date to file."));
    }
    if req.shift_id <= 0 || req.shift_name.trim().is_empty() {
        return Err(AppError::input("NBS has no shift for this date, so there's nothing to pick."));
    }
    let tin = parse_clock(&req.time_in).ok_or_else(|| AppError::input("Time in must be a time like 07:30."))?;
    let tout = parse_clock(&req.time_out).ok_or_else(|| AppError::input("Time out must be a time like 17:30."))?;
    let overnight = parse_shift_times(&req.shift_name).is_some_and(|p| p.time_out <= p.time_in);
    let mut span = tout as i64 - tin as i64;
    if span <= 0 && overnight {
        span += 86_400;
    }
    if span <= 0 {
        return Err(AppError::input("Time out must be later than time in."));
    }
    let hours = (span as f64 / 3600.0 * 100.0).round() / 100.0;
    let full = |s: u32| format!("{}:{:02}", hhmm(s), s % 60);
    Ok((full(tin), full(tout), hours))
}

fn hours_json(hours: f64) -> Value {
    if hours.fract() == 0.0 { json!(hours as i64) } else { json!(hours) }
}

pub fn ob_payload(me: &Me, date: &str, customer_id: i64, time_in: &str, time_out: &str, hours: f64, reason: &str) -> Value {
    json!({
        "id": null, "company_id": me.company_id, "employee_id": me.employee_id,
        "employee_name": me.employee_name,
        "customer_id": customer_id, "is_other_customer": 0, "other_customer": null,
        "date": null, "date_start": date, "date_end": date,
        "time_in": time_in, "time_out": time_out,
        "reason": reason.trim(), "hours_filed": hours_json(hours), "hours_approved": 0,
        "approval_status_id": null, "other_name": null,
    })
}

pub fn cs_payload(me: &Me, date: &str, shift_id: i64, shift_name: &str, reason: &str) -> Value {
    json!({
        "id": null, "company_id": me.company_id, "employee_id": me.employee_id,
        "employee_name": me.employee_name,
        "date": date, "to_date": date,
        "new_shift_schedule_id": shift_id, "new_shift_schedule": shift_name,
        "reason": reason.trim(), "approval_status_id": null,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn me() -> Me {
        Me {
            employee_id: 1001,
            company_id: 1,
            employee_name: "JUAN DELA CRUZ".into(),
            first_name: "JUAN".into(),
            display_name: "JUAN DELA CRUZ".into(),
            employee_code: String::new(),
            position: String::new(),
            department: String::new(),
        }
    }

    fn req(time_in: &str, time_out: &str, shift: &str) -> FileRequest {
        FileRequest {
            date: "2026-07-07".into(),
            shift_id: 9,
            shift_name: shift.into(),
            time_in: time_in.into(),
            time_out: time_out.into(),
            ob_reason: "Work from home".into(),
            cs_reason: "WFH".into(),
            customer: String::new(),
        }
    }

    #[test]
    fn files_against_the_chosen_customer_or_the_built_in_one() {
        let c = |id, name: &str| Customer { id, customer_name: name.into() };
        let all = [c(5, "315 CORPORATE OFFICE"), c(8, " Acme Site ")];
        assert_eq!(find_customer(&all, "").unwrap(), 5);
        assert_eq!(find_customer(&all, "  ").unwrap(), 5);
        assert_eq!(find_customer(&all, "acme site").unwrap(), 8);
        assert_eq!(find_customer(&all, "Nope").unwrap_err().kind, "nbs");
        // An older UI that sends no customer still files against the built-in one.
        let old: FileRequest = serde_json::from_value(json!({"date": "2026-07-07", "shift_id": 9, "shift_name": "s",
            "time_in": "07:30", "time_out": "17:30", "ob_reason": "a", "cs_reason": "b"})).unwrap();
        assert_eq!(old.customer, "");
    }

    #[test]
    fn validates_and_normalises_times() {
        let day = "CW 9.5H 730AM-530PM SP 30MIN BRK";
        let (tin, tout, hours) = validate(&req("7:30", "17:30", day)).unwrap();
        assert_eq!((tin.as_str(), tout.as_str(), hours), ("07:30:00", "17:30:00", 10.0));
        assert_eq!(validate(&req("07:30", "18:00:00", day)).unwrap().2, 10.5);
        assert_eq!(validate(&req("17:30", "07:30", day)).unwrap_err().kind, "input");
        assert_eq!(validate(&req("7.30", "17:30", day)).unwrap_err().kind, "input");
        assert_eq!(validate(&req("25:00", "17:30", day)).unwrap_err().kind, "input");
        let night = validate(&req("22:00", "06:00", "CW 10H 10PM-6AM SP 30MIN BRK")).unwrap();
        assert_eq!(night.2, 8.0);
        let mut no_shift = req("7:30", "17:30", day);
        no_shift.shift_id = 0;
        assert!(validate(&no_shift).is_err());
    }

    #[test]
    fn payloads_match_what_nbs_accepts() {
        let ob = ob_payload(&me(), "2026-07-07", 5, "07:30:00", "17:30:00", 10.0, " WFH ");
        assert_eq!(ob["hours_filed"], json!(10));
        assert_eq!(ob["date_start"], "2026-07-07");
        assert_eq!(ob["reason"], "WFH");
        assert_eq!(ob["customer_id"], 5);
        assert_eq!(ob_payload(&me(), "d", 5, "a", "b", 10.5, "")["hours_filed"], json!(10.5));
        let cs = cs_payload(&me(), "2026-07-07", 77, "CW 9.5H 8AM-6PM SP 30MIN BRK", "WFH");
        assert_eq!((cs["to_date"].as_str(), cs["new_shift_schedule_id"].as_i64()), (Some("2026-07-07"), Some(77)));
        assert_eq!(cs["employee_name"], "JUAN DELA CRUZ");
    }

    #[test]
    fn only_unposted_for_approval_filings_are_deletable() {
        assert!(is_pending("For Approval", 0));
        assert!(is_pending("for approval ", 0));
        assert!(!is_pending("For Approval", 1));
        assert!(!is_pending("Approved", 0));
        assert!(!is_pending("Disapproved", 0));
        assert_eq!(deletable("OB", Some((3, "For Approval", 0))).unwrap(), 3);
        assert_eq!(deletable("OB", Some((3, "Approved", 1))).unwrap_err().kind, "input");
        assert!(deletable("CS", None).is_err());
    }

    /// Read-only check against the real NBS (needs ZeroTier); never files or deletes:
    /// `NBS_USER=... NBS_PASS=... cargo test live -- --ignored --nocapture`
    /// (without NBS_USER/NBS_PASS it uses the app's remembered login).
    #[test]
    #[ignore]
    fn live_read_only() {
        let (user, pass) = match (std::env::var("NBS_USER"), std::env::var("NBS_PASS")) {
            (Ok(u), Ok(p)) => (u, p),
            _ => creds::load().expect("set NBS_USER and NBS_PASS, or sign in once with Remember me"),
        };
        tauri::async_runtime::block_on(async {
            let nbs = Nbs::new();
            assert!(nbs.reachable().await, "NBS unreachable: ZeroTier?");
            let me = nbs.login(&user, &pass).await.unwrap();
            assert!(me.employee_id > 0 && !me.employee_name.is_empty());
            let profile = nbs.profile().await.unwrap();
            assert!(!profile.employee_code.is_empty());
            let today = chrono::Local::now().date_naive().format("%Y-%m-%d").to_string();
            let month_start = format!("{}-01", &today[..7]);
            let obs = nbs.ob_filings(&month_start, &today).await.unwrap();
            let css = nbs.cs_filings(&month_start, &today).await.unwrap();
            let leaves = nbs.leave_filings(&month_start, &today).await.unwrap();
            let shifts = nbs.shifts().await.unwrap();
            assert!(!shifts.is_empty());
            find_customer(&nbs.customers().await.unwrap(), "").unwrap();
            println!("ok: {} OB, {} CS, {} leave this month, {} shifts", obs.len(), css.len(), leaves.len(), shifts.len());
        });
    }

    #[test]
    fn reads_identity_from_jwt_and_lenient_json() {
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(
            br#"{"id":1,"employee_id":"1001","company_id":1,"user_name":"JUAN DELA CRUZ","first_name":"JUAN","last_name":"DELA CRUZ","exp":1}"#,
        );
        let me = me_from_token(&format!("h.{payload}.s")).unwrap();
        assert_eq!((me.employee_id, me.employee_name.as_str(), me.display_name.as_str()), (1001, "JUAN DELA CRUZ", "JUAN DELA CRUZ"));
        assert!(expiring(&format!("h.{payload}.s"), 60));
        assert!(me_from_token("not-a-jwt").is_none());

        let f: ObFiling = serde_json::from_str(r#"{"id":"12","date":"2026-07-07","hours_filed":null,"status":"For Approval","is_voided":"0"}"#).unwrap();
        assert_eq!((f.id, f.hours_filed.as_str(), f.is_voided, f.is_posted), (12, "", 0, 0));

        // The fields we read from a real leave_filing/get_filings row.
        let l: LeaveFiling = serde_json::from_str(
            r#"{"id":31442,"date":"2026-06-11","leave_type":"Vacation Leave","days":"0.500000","reason":null,"status":"For Approval","is_voided":0}"#,
        )
        .unwrap();
        assert_eq!(
            (l.date.as_str(), l.leave_type.as_str(), l.days.as_str(), l.reason.as_str(), l.status.as_str(), l.is_voided),
            ("2026-06-11", "Vacation Leave", "0.500000", "", "For Approval", 0)
        );
    }
}

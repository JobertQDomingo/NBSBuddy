//! Per-day verdicts and the period summary, all relative to `today`: payroll cutoff is the
//! period's last day, so a future workday with nothing filed is "upcoming", not missing.
//! One source of truth for the table, the status strip and the day rail.

use std::collections::BTreeSet;

use chrono::{Datelike, NaiveDate};
use serde::Serialize;

use crate::nbs::{CsFiling, LeaveFiling, ObFiling, is_pending};
use crate::shifts::{TimeStatus, late_undertime_minutes, time_status};

pub const DONE: &str = "Done";
pub const WAITING: &str = "Waiting";
pub const DUE: &str = "Due today";
pub const ACTION: &str = "Needs action";
const APPROVED: &str = "Approved";
const DISAPPROVED: &str = "Disapproved";

// ---- periods: 1st half = 1-15, 2nd half = 16-end of month ----

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

pub fn make_period(year: i32, month: u32, half: u32) -> Option<Period> {
    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let last = first.checked_add_months(chrono::Months::new(1))?.pred_opt()?;
    match half {
        1 => Some(Period { start: first, end: first.with_day(15)? }),
        2 => Some(Period { start: first.with_day(16)?, end: last }),
        _ => None,
    }
}

pub fn period_label(p: &Period) -> String {
    format!("{} {}–{}", p.start.format("%b"), p.start.day(), p.end.day())
}

fn is_workday(d: NaiveDate) -> bool {
    d.weekday().num_days_from_monday() < 5
}

pub fn fmt_day(d: NaiveDate) -> String {
    d.format("%b %-d").to_string()
}

fn fmt_weekday(d: NaiveDate) -> String {
    d.format("%a %b %-d").to_string()
}

pub fn parse_hours(value: &str) -> f64 {
    value.trim().parse::<f64>().ok().filter(|h| h.is_finite()).unwrap_or(0.0)
}

pub fn fmt_hours(hours: f64) -> String {
    let s = format!("{hours:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

// ---- day rows ----

#[derive(Debug, Clone)]
pub struct DayRow {
    pub date: NaiveDate,
    pub workday: bool,
    pub ob: Option<ObFiling>,
    pub cs: Option<CsFiling>,
    pub leave: Option<LeaveFiling>,
}

/// A whole-day leave stands in for OB + CS; a half-day one ("0.500000") needs both beside it.
fn whole_day(f: &LeaveFiling) -> bool {
    parse_hours(&f.days) >= 1.0
}

/// "Vacation Leave", or "Vacation Leave (half day)".
fn leave_name(f: &LeaveFiling) -> String {
    let name = Some(f.leave_type.trim()).filter(|t| !t.is_empty()).unwrap_or("Leave");
    if whole_day(f) { name.to_string() } else { format!("{name} (half day)") }
}

impl DayRow {
    fn ob_missing(&self) -> bool {
        self.workday && self.ob.is_none() && !self.leave.as_ref().is_some_and(whole_day)
    }
    fn cs_missing(&self) -> bool {
        self.workday && self.cs.is_none() && !self.leave.as_ref().is_some_and(whole_day)
    }
    fn missing(&self) -> Vec<&'static str> {
        [("OB", self.ob_missing()), ("CS", self.cs_missing())].into_iter().filter(|x| x.1).map(|x| x.0).collect()
    }
    /// (kind, status) of each filing present.
    fn filed(&self) -> Vec<(&'static str, &str)> {
        let mut out = Vec::new();
        if let Some(f) = &self.ob {
            out.push(("OB", f.status.as_str()));
        }
        if let Some(f) = &self.cs {
            out.push(("CS", f.status.as_str()));
        }
        if let Some(f) = &self.leave {
            out.push(("Leave", f.status.as_str()));
        }
        out
    }
    fn disapproved(&self) -> Vec<&'static str> {
        self.filed().into_iter().filter(|(_, s)| *s == DISAPPROVED).map(|(k, _)| k).collect()
    }
    fn ob_time(&self) -> String {
        self.ob.as_ref().map(|f| format!("{}-{}", clip5(&f.time_in), clip5(&f.time_out))).unwrap_or_default()
    }
    fn new_shift(&self) -> &str {
        self.cs.as_ref().map(|f| f.new_shift_schedule.as_str()).unwrap_or("")
    }
    fn time_status(&self) -> TimeStatus {
        // ponytail: a leave day's OB is short by design, so the check is skipped outright;
        // compare against the part of the shift the leave doesn't cover if that ever matters.
        if self.leave.is_some() {
            return TimeStatus::Ok;
        }
        time_status(&self.ob_time(), self.new_shift())
    }
    fn late_or_undertime(&self) -> bool {
        !matches!(self.time_status(), TimeStatus::Ok | TimeStatus::Mismatch)
    }
}

fn clip5(s: &str) -> &str {
    s.get(..5).unwrap_or(s)
}

pub fn build_day_rows(p: &Period, obs: &[ObFiling], css: &[CsFiling], leaves: &[LeaveFiling]) -> Vec<DayRow> {
    p.start
        .iter_days()
        .take_while(|d| *d <= p.end)
        .map(|date| {
            let iso = date.format("%Y-%m-%d").to_string();
            DayRow {
                date,
                workday: is_workday(date),
                ob: obs.iter().find(|f| f.date.starts_with(&iso) && f.is_voided == 0).cloned(),
                cs: css.iter().find(|f| f.date.starts_with(&iso) && f.is_voided == 0).cloned(),
                // A refile beats the disapproved leave it replaces.
                leave: leaves
                    .iter()
                    .filter(|f| f.date.starts_with(&iso) && f.is_voided == 0)
                    .min_by_key(|f| f.status == DISAPPROVED)
                    .cloned(),
            }
        })
        .collect()
}

pub fn day_verdict(row: &DayRow, today: NaiveDate) -> &'static str {
    if !row.disapproved().is_empty() || row.time_status() == TimeStatus::Mismatch {
        return ACTION;
    }
    if !row.missing().is_empty() {
        return if row.date < today {
            ACTION
        } else if row.date == today {
            DUE
        } else {
            "" // upcoming, not missing
        };
    }
    let filed = row.filed();
    if filed.is_empty() {
        ""
    } else if filed.iter().all(|(_, s)| *s == APPROVED) {
        DONE
    } else {
        WAITING
    }
}

fn fmt_minutes(m: i32) -> String {
    match (m / 60, m % 60) {
        (0, m) => format!("{m}m"),
        (h, 0) => format!("{h}h"),
        (h, m) => format!("{h}h {m}m"),
    }
}

/// Why a row looks the way it does, in words (colour is never the only signal).
pub fn day_note(row: &DayRow, today: NaiveDate) -> String {
    let mut parts = Vec::new();
    if let Some(f) = &row.leave {
        parts.push(leave_name(f));
    }
    let missing = row.missing();
    if !missing.is_empty() {
        if row.date > today {
            parts.push(if missing.len() == 2 { "Upcoming".into() } else { format!("{} not filed yet", missing[0]) });
        } else {
            let text = if missing.len() == 2 { "Not filed".to_string() } else { format!("{} not filed", missing[0]) };
            parts.push(if row.date == today { format!("{text} yet") } else { text });
        }
    }
    let bad = row.disapproved();
    if !bad.is_empty() {
        parts.push(format!("{} disapproved, refile in NBS", bad.join(" + ")));
    }
    match row.time_status() {
        TimeStatus::Mismatch => parts.push("Time mismatch".into()),
        TimeStatus::Ok => {}
        _ => {
            let (late, under) = late_undertime_minutes(&row.ob_time(), row.new_shift());
            if late > 0 {
                parts.push(format!("Late {}", fmt_minutes(late)));
            }
            if under > 0 {
                parts.push(format!("Undertime {}", fmt_minutes(under)));
            }
        }
    }
    if day_verdict(row, today) == WAITING {
        let waiting: Vec<&str> = row.filed().into_iter().filter(|(_, s)| *s != APPROVED).map(|(k, _)| k).collect();
        parts.push(format!("{} awaiting approval", waiting.join(" + ")));
    }
    parts.join(" · ")
}

/// Row wash: danger > attention (due today, late/undertime) > muted (weekend, upcoming).
pub fn row_tone(row: &DayRow, today: NaiveDate) -> &'static str {
    let verdict = day_verdict(row, today);
    if verdict == ACTION {
        "danger"
    } else if verdict == DUE || row.late_or_undertime() {
        "attention"
    } else if !row.workday || (row.date > today && row.filed().is_empty()) {
        "muted"
    } else {
        ""
    }
}

// ---- period summary ----

#[derive(Debug, Clone, PartialEq)]
pub struct PeriodSummary {
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub today: NaiveDate,
    pub to_file: Vec<NaiveDate>,
    pub to_refile: Vec<NaiveDate>,
    pub mismatches: Vec<NaiveDate>,
    pub late_undertime: Vec<NaiveDate>,
    pub due_today: bool,
    pub waiting: usize,
    pub upcoming: usize,
    pub workdays_left: usize,
    pub days_filed: usize,
    pub days_expected: usize,
    /// Workdays with a leave that isn't disapproved (whole or half day).
    pub on_leave: usize,
    pub total_hours: f64,
}

impl PeriodSummary {
    pub fn needs_action(&self) -> usize {
        self.to_file.iter().chain(&self.to_refile).chain(&self.mismatches).collect::<BTreeSet<_>>().len()
    }
}

pub fn compute_summary(rows: &[DayRow], today: NaiveDate) -> PeriodSummary {
    let dates = |pred: &dyn Fn(&DayRow) -> bool| rows.iter().filter(|r| pred(r)).map(|r| r.date).collect::<Vec<_>>();
    let work: Vec<&DayRow> = rows.iter().filter(|r| r.workday).collect();
    PeriodSummary {
        start: rows.first().map_or(today, |r| r.date),
        end: rows.last().map_or(today, |r| r.date),
        today,
        to_file: dates(&|r| !r.missing().is_empty() && r.date < today),
        to_refile: dates(&|r| !r.disapproved().is_empty()),
        mismatches: dates(&|r| r.time_status() == TimeStatus::Mismatch),
        late_undertime: dates(&|r| r.late_or_undertime()),
        due_today: rows.iter().any(|r| !r.missing().is_empty() && r.date == today),
        waiting: rows.iter().filter(|r| day_verdict(r, today) == WAITING).count(),
        upcoming: work.iter().filter(|r| r.date > today && r.filed().is_empty()).count(),
        workdays_left: work.iter().filter(|r| r.date >= today).count(),
        days_filed: work.iter().filter(|r| r.missing().is_empty() && r.disapproved().is_empty()).count(),
        days_expected: work.len(),
        on_leave: work.iter().filter(|r| r.leave.as_ref().is_some_and(|f| f.status != DISAPPROVED)).count(),
        total_hours: rows
            .iter()
            .filter_map(|r| r.ob.as_ref())
            .filter(|f| f.status != DISAPPROVED)
            .map(|f| parse_hours(&f.hours_filed))
            .sum(),
    }
}

fn plural(n: usize, word: &str) -> String {
    format!("{n} {word}{}", if n == 1 { "" } else { "s" })
}

fn days_list(ds: &[NaiveDate]) -> String {
    let shown: Vec<String> = ds.iter().take(4).map(|d| fmt_day(*d)).collect();
    let more = if ds.len() > 4 { format!(" +{} more", ds.len() - 4) } else { String::new() };
    shown.join(", ") + &more
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Strip {
    pub tone: &'static str,
    pub headline: String,
    pub detail: String,
    pub cutoff: String,
    pub progress: String,
}

pub fn strip_text(s: &PeriodSummary) -> Strip {
    let n = s.needs_action();
    let mut items = Vec::new();
    if s.due_today && n > 0 {
        items.push("Today not filed yet".to_string()); // otherwise the headline says it
    }
    for (label, ds) in [
        ("To file", &s.to_file),
        ("Disapproved, refile in NBS", &s.to_refile),
        ("Time mismatch", &s.mismatches),
        ("Late/undertime", &s.late_undertime),
    ] {
        if !ds.is_empty() {
            items.push(format!("{label}: {}", days_list(ds)));
        }
    }
    if s.waiting > 0 {
        items.push(format!("Awaiting approval: {}", plural(s.waiting, "day")));
    }
    let (tone, headline) = if n > 0 {
        ("danger", format!("{} need{} action", plural(n, "day"), if n == 1 { "s" } else { "" }))
    } else if s.due_today {
        ("attention", "Today isn't filed yet".to_string())
    } else if s.today < s.start {
        ("muted", "Nothing due yet".to_string())
    } else if s.upcoming > 0 {
        ("success", "Every workday so far is filed".to_string())
    } else {
        ("success", "Every workday is filed".to_string())
    };
    let mut cutoff = format!("Cutoff {}, {}", s.end.format("%a"), fmt_day(s.end));
    cutoff += &if s.today > s.end { " (passed)".to_string() } else { format!(" · {} left", plural(s.workdays_left, "workday")) };
    let leave = if s.on_leave > 0 { format!(" ({} on leave)", s.on_leave) } else { String::new() };
    Strip {
        tone,
        headline,
        detail: if items.is_empty() { "No missing, disapproved or mismatched days.".into() } else { items.join(" · ") },
        cutoff,
        progress: format!("{} of {} workdays filed{leave} · {} h", s.days_filed, s.days_expected, fmt_hours(s.total_hours)),
    }
}

// ---- what the UI renders ----

#[derive(Debug, Clone, Serialize)]
pub struct RailDay {
    pub date: String,
    pub day: u32,
    /// done / wait / due / act / up (upcoming: nothing to judge yet)
    pub kind: &'static str,
    pub today: bool,
    /// A Disapproved filing: the sticker stays, struck through.
    pub struck: bool,
    pub cutoff: bool,
    pub tip: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FilingView {
    pub status: String,
    pub deletable: bool,
    pub time_in: String,
    pub time_out: String,
    pub hours: String,
    pub reason: String,
    pub shift: String,
    pub customer: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RowView {
    pub date: String,
    pub label: String,
    pub today: bool,
    pub workday: bool,
    pub verdict: &'static str,
    pub tone: &'static str,
    pub ob_time: String,
    pub shift: String,
    pub hours: String,
    pub note: String,
    pub reasons: String,
    pub reasons_tip: String,
    pub status_tip: String,
    pub ob: Option<FilingView>,
    pub cs: Option<FilingView>,
    /// "Vacation Leave (half day) · For Approval"; empty when there's no leave.
    pub leave: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PeriodView {
    pub label: String,
    pub today: String,
    pub rows: Vec<RowView>,
    pub strip: Strip,
    pub rail: Vec<RailDay>,
}

fn iso(d: NaiveDate) -> String {
    d.format("%Y-%m-%d").to_string()
}

fn first_line(text: &str) -> String {
    let lines: Vec<&str> = text.trim().lines().collect();
    match lines.as_slice() {
        [] => String::new(),
        [one] => one.to_string(),
        [first, ..] => format!("{first} …"),
    }
}

pub fn rail_days(rows: &[DayRow], today: NaiveDate) -> Vec<RailDay> {
    let work: Vec<&DayRow> = rows.iter().filter(|r| r.workday).collect();
    work.iter()
        .enumerate()
        .map(|(i, r)| {
            let note = day_note(r, today);
            RailDay {
                date: iso(r.date),
                day: r.date.day(),
                kind: match day_verdict(r, today) {
                    DONE => "done",
                    WAITING => "wait",
                    DUE => "due",
                    ACTION => "act",
                    _ => "up",
                },
                today: r.date == today,
                struck: !r.disapproved().is_empty(),
                cutoff: i == work.len() - 1,
                tip: if note.is_empty() { fmt_weekday(r.date) } else { format!("{} · {note}", fmt_weekday(r.date)) },
            }
        })
        .collect()
}

fn row_view(row: &DayRow, today: NaiveDate) -> RowView {
    let line = |kind: &str, status: Option<&str>, missing: bool, extra: &str| match status {
        Some(s) if extra.is_empty() => format!("{kind}: {s}"),
        Some(s) => format!("{kind}: {s} · {extra}"),
        None if missing => format!("{kind}: not filed"),
        None => format!("{kind}: —"),
    };
    let reasons: Vec<(&str, &str)> = [
        ("OB", row.ob.as_ref().map(|f| f.reason.as_str())),
        ("CS", row.cs.as_ref().map(|f| f.reason.as_str())),
        ("Leave", row.leave.as_ref().map(|f| f.reason.as_str())),
    ]
    .into_iter()
    .filter_map(|(k, r)| r.filter(|r| !r.trim().is_empty()).map(|r| (k, r)))
    .collect();
    let hours = row.ob.as_ref().map(|f| f.hours_filed.trim()).filter(|h| !h.is_empty()).map(|h| fmt_hours(parse_hours(h))).unwrap_or_default();
    let mut status_tip = [
        line("OB", row.ob.as_ref().map(|f| f.status.as_str()), row.ob_missing(), row.ob.as_ref().map_or("", |f| f.customer_name.as_str())),
        line("CS", row.cs.as_ref().map(|f| f.status.as_str()), row.cs_missing(), ""),
    ]
    .join("\n");
    if let Some(f) = &row.leave {
        status_tip += &format!("\n{}", line("Leave", Some(&f.status), false, &leave_name(f)));
    }
    RowView {
        date: iso(row.date),
        label: fmt_weekday(row.date),
        today: row.date == today,
        workday: row.workday,
        verdict: day_verdict(row, today),
        tone: row_tone(row, today),
        ob_time: row.ob_time().replace('-', "–"),
        shift: row.new_shift().to_string(),
        hours: hours.clone(),
        note: day_note(row, today),
        reasons: reasons.iter().map(|(k, r)| format!("{k}: {}", first_line(r))).collect::<Vec<_>>().join(" · "),
        reasons_tip: reasons.iter().map(|(k, r)| format!("{k} reason:\n{}", r.trim())).collect::<Vec<_>>().join("\n\n"),
        status_tip,
        ob: row.ob.as_ref().map(|f| FilingView {
            status: f.status.clone(),
            deletable: is_pending(&f.status, f.is_posted),
            time_in: clip5(&f.time_in).to_string(),
            time_out: clip5(&f.time_out).to_string(),
            hours,
            reason: f.reason.clone(),
            shift: String::new(),
            customer: f.customer_name.clone(),
        }),
        cs: row.cs.as_ref().map(|f| FilingView {
            status: f.status.clone(),
            deletable: is_pending(&f.status, f.is_posted),
            time_in: String::new(),
            time_out: String::new(),
            hours: String::new(),
            reason: f.reason.clone(),
            shift: f.new_shift_schedule.clone(),
            customer: String::new(),
        }),
        leave: row.leave.as_ref().map(|f| format!("{} · {}", leave_name(f), f.status)).unwrap_or_default(),
    }
}

pub fn period_view(p: &Period, obs: &[ObFiling], css: &[CsFiling], leaves: &[LeaveFiling], today: NaiveDate) -> PeriodView {
    let rows = build_day_rows(p, obs, css, leaves);
    PeriodView {
        label: period_label(p),
        today: iso(today),
        strip: strip_text(&compute_summary(&rows, today)),
        rail: rail_days(&rows, today),
        rows: rows.iter().map(|r| row_view(r, today)).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 7, d).unwrap()
    }
    const TODAY: u32 = 8; // Wednesday, Jul 8 2026

    fn ob(d: u32, status: &str, time_in: &str, hours: &str) -> ObFiling {
        ObFiling {
            id: d as i64,
            date: iso(day(d)),
            customer_name: "315 CORPORATE OFFICE".into(),
            time_in: time_in.into(),
            time_out: "18:00:00".into(),
            hours_filed: hours.into(),
            reason: "WFH".into(),
            status: status.into(),
            ..Default::default()
        }
    }

    fn cs(d: u32, status: &str) -> CsFiling {
        CsFiling {
            id: 100 + d as i64,
            date: iso(day(d)),
            new_shift_schedule: "CW 9.5H 8AM-6PM SP 30MIN BRK".into(),
            reason: "WFH".into(),
            status: status.into(),
            ..Default::default()
        }
    }

    struct R {
        d: u32,
        ob: Option<&'static str>,
        cs: Option<&'static str>,
        time_in: &'static str,
        hours: &'static str,
    }

    fn r(d: u32) -> R {
        R { d, ob: Some("Approved"), cs: Some("Approved"), time_in: "08:00:00", hours: "10.000000" }
    }
    fn none(d: u32) -> R {
        R { ob: None, cs: None, ..r(d) }
    }

    fn row(x: R) -> DayRow {
        let date = day(x.d);
        DayRow {
            date,
            workday: is_workday(date),
            ob: x.ob.map(|s| ob(x.d, s, x.time_in, x.hours)),
            cs: x.cs.map(|s| cs(x.d, s)),
            leave: None,
        }
    }

    fn lv(d: u32, status: &str, days: &str) -> LeaveFiling {
        LeaveFiling {
            date: iso(day(d)),
            leave_type: "Vacation Leave".into(),
            days: days.into(),
            reason: "Family trip".into(),
            status: status.into(),
            is_voided: 0,
        }
    }

    fn on_leave(x: R, status: &str, days: &str) -> DayRow {
        DayRow { leave: Some(lv(x.d, status, days)), ..row(x) }
    }

    fn t() -> NaiveDate {
        day(TODAY)
    }

    #[test]
    fn periods() {
        let p = make_period(2026, 2, 2).unwrap();
        assert_eq!((p.start, p.end), (NaiveDate::from_ymd_opt(2026, 2, 16).unwrap(), NaiveDate::from_ymd_opt(2026, 2, 28).unwrap()));
        assert_eq!(make_period(2026, 12, 2).unwrap().end.day(), 31);
        assert_eq!(period_label(&make_period(2026, 7, 1).unwrap()), "Jul 1–15");
        assert!(make_period(2026, 7, 3).is_none());
        assert_eq!((parse_hours("10.000000"), parse_hours("9.5"), parse_hours(""), parse_hours("n/a")), (10.0, 9.5, 0.0, 0.0));
    }

    #[test]
    fn verdicts_and_notes() {
        let cases = [
            (row(none(13)), "", "Upcoming"),
            (row(none(8)), DUE, "Not filed yet"),
            (row(R { cs: None, ..r(7) }), ACTION, "CS not filed"),
            (row(R { cs: Some("Disapproved"), ..r(7) }), ACTION, "CS disapproved, refile in NBS"),
            (row(R { ob: Some("For Approval"), ..r(7) }), WAITING, "OB awaiting approval"),
            (row(r(7)), DONE, ""),
            (row(R { time_in: "08:30:00", ..r(7) }), DONE, "Late 30m"), // shift 8AM-6PM
            // A whole-day leave stands in for OB + CS; it waits for approval like they do.
            (on_leave(none(7), "Approved", "1.000000"), DONE, "Vacation Leave"),
            (on_leave(none(7), "For Approval", "1.000000"), WAITING, "Vacation Leave · Leave awaiting approval"),
            (on_leave(none(7), "Disapproved", "1.000000"), ACTION, "Vacation Leave · Leave disapproved, refile in NBS"),
            // A half day still needs OB + CS, and its short OB isn't late.
            (on_leave(none(7), "Approved", "0.500000"), ACTION, "Vacation Leave (half day) · Not filed"),
            (on_leave(R { cs: None, ..r(7) }, "Approved", "0.500000"), ACTION, "Vacation Leave (half day) · CS not filed"),
            (on_leave(R { time_in: "13:00:00", ..r(7) }, "Approved", "0.500000"), DONE, "Vacation Leave (half day)"),
            (on_leave(R { ob: Some("For Approval"), ..r(7) }, "Approved", "0.500000"), WAITING, "Vacation Leave (half day) · OB awaiting approval"),
        ];
        for (row, verdict, note) in cases {
            assert_eq!((day_verdict(&row, t()), day_note(&row, t()).as_str()), (verdict, note), "{}", row.date);
        }
    }

    #[test]
    fn row_tones() {
        assert_eq!(row_tone(&row(none(7)), t()), "danger");
        assert_eq!(row_tone(&row(none(8)), t()), "attention");
        assert_eq!(row_tone(&row(none(13)), t()), "muted");
        assert_eq!(row_tone(&row(none(4)), t()), "muted"); // Saturday
        assert_eq!(row_tone(&row(R { time_in: "08:30:00", ..r(7) }), t()), "attention");
        assert_eq!(row_tone(&row(r(7)), t()), "");
        assert_eq!(row_tone(&on_leave(R { time_in: "13:00:00", ..r(7) }, "Approved", "0.500000"), t()), "");
        assert_eq!(row_tone(&on_leave(none(7), "Disapproved", "1.000000"), t()), "danger");
    }

    #[test]
    fn leave_in_the_summary() {
        let mut rows = period();
        rows[1] = on_leave(none(7), "Approved", "1.000000");
        let s = compute_summary(&rows, t());
        assert!(s.to_file.is_empty());
        assert_eq!((s.days_filed, s.on_leave, s.needs_action()), (2, 1, 0));
        assert_eq!(strip_text(&s).progress, "2 of 5 workdays filed (1 on leave) · 10 h");

        rows[1] = on_leave(none(7), "Disapproved", "1.000000");
        let s = compute_summary(&rows, t());
        assert_eq!((s.to_refile.as_slice(), s.to_file.len(), s.on_leave), (&[day(7)][..], 0, 0));
        assert!(strip_text(&s).detail.contains("Disapproved, refile in NBS: Jul 7"));

        // A voided leave is gone, as a voided OB/CS is.
        let p = Period { start: day(6), end: day(10) };
        let voided = LeaveFiling { is_voided: 1, ..lv(7, "Approved", "1.000000") };
        let rows = build_day_rows(&p, &[], &[], &[voided]);
        assert!(rows[1].leave.is_none());
        assert_eq!((day_verdict(&rows[1], t()), day_note(&rows[1], t()).as_str()), (ACTION, "Not filed"));
        assert_eq!(build_day_rows(&p, &[], &[], &[lv(7, "Approved", "1.000000")])[1].leave.as_ref().unwrap().status, "Approved");

        // Refiled after a disapproval: the refile counts, whichever order NBS sends them in.
        let refiled = [lv(7, "Disapproved", "1.000000"), lv(7, "For Approval", "1.000000")];
        assert_eq!(build_day_rows(&p, &[], &[], &refiled)[1].leave.as_ref().unwrap().status, "For Approval");
    }

    fn period() -> Vec<DayRow> {
        vec![
            row(r(6)),                       // Mon: done
            row(R { cs: None, ..r(7) }),     // Tue: CS missing (past)
            row(none(8)),                    // Wed (today): due
            row(none(9)),                    // Thu: upcoming
            row(none(10)),                   // Fri: upcoming
            row(none(11)),                   // Sat
        ]
    }

    #[test]
    fn summary_counts_relative_to_today() {
        let s = compute_summary(&period(), t());
        assert_eq!(s.to_file, [day(7)]);
        assert!(s.due_today);
        assert_eq!((s.upcoming, s.workdays_left, s.days_filed, s.days_expected, s.needs_action()), (2, 3, 1, 5, 1));
        assert_eq!(s.total_hours, 20.0);

        let s = compute_summary(&[row(R { ob: Some("Disapproved"), cs: Some("Disapproved"), hours: "9.5", ..r(6) })], t());
        assert_eq!((s.to_refile.as_slice(), s.days_filed, s.total_hours), (&[day(6)][..], 0, 0.0));
    }

    #[test]
    fn strip_text_reads_like_the_old_app() {
        let s = strip_text(&compute_summary(&period(), t()));
        assert_eq!((s.tone, s.headline.as_str()), ("danger", "1 day needs action"));
        assert!(s.detail.starts_with("Today not filed yet · To file: Jul 7"), "{}", s.detail);
        assert_eq!(s.cutoff, "Cutoff Sat, Jul 11 · 3 workdays left");
        assert_eq!(s.progress, "1 of 5 workdays filed · 20 h");

        let s = strip_text(&compute_summary(&[row(r(7)), row(none(8))], t()));
        assert_eq!((s.tone, s.headline.as_str()), ("attention", "Today isn't filed yet"));
        assert!(!s.detail.contains("Today"));

        let s = strip_text(&compute_summary(&[row(r(6)), row(r(7))], day(20)));
        assert_eq!((s.tone, s.headline.as_str()), ("success", "Every workday is filed"));
        assert!(s.cutoff.ends_with("(passed)"));

        let rows: Vec<DayRow> = [1, 2, 3, 6, 7].into_iter().map(|d| row(none(d))).collect();
        assert!(strip_text(&compute_summary(&rows, t())).detail.contains("To file: Jul 1, Jul 2, Jul 3, Jul 6 +1 more"));
    }

    #[test]
    fn rail_and_view() {
        let mut rows = period();
        rows.push(row(R { cs: Some("Disapproved"), ..r(9) }));
        let days = rail_days(&rows, t());
        assert_eq!(days.iter().map(|d| d.day).collect::<Vec<_>>(), [6, 7, 8, 9, 10, 9]);
        assert_eq!(days.iter().take(5).map(|d| d.kind).collect::<Vec<_>>(), ["done", "act", "due", "up", "up"]);
        assert!(days[2].today && !days[0].today);
        assert!(days[5].struck && days[5].kind == "act" && days[5].cutoff && !days[0].cutoff);
        assert_eq!(days[1].tip, "Tue Jul 7 · CS not filed");

        let p = make_period(2026, 7, 1).unwrap();
        let obs = [ob(6, "For Approval", "08:00:00", "10.000000")];
        let v = period_view(&p, &obs, &[cs(6, "Approved")], &[lv(7, "Approved", "1.000000")], t());
        assert_eq!(v.rows.len(), 15);
        let mon = &v.rows[5];
        assert_eq!((mon.label.as_str(), mon.ob_time.as_str(), mon.hours.as_str()), ("Mon Jul 6", "08:00–18:00", "10"));
        assert!(mon.ob.as_ref().unwrap().deletable && !mon.cs.as_ref().unwrap().deletable);
        assert_eq!(mon.status_tip, "OB: For Approval · 315 CORPORATE OFFICE\nCS: Approved");
        assert_eq!(mon.leave, "");

        let tue = &v.rows[6];
        assert_eq!((tue.verdict, tue.leave.as_str(), tue.note.as_str()), (DONE, "Vacation Leave · Approved", "Vacation Leave"));
        assert_eq!(tue.status_tip, "OB: —\nCS: —\nLeave: Approved · Vacation Leave");
        assert_eq!((tue.reasons.as_str(), tue.reasons_tip.as_str()), ("Leave: Family trip", "Leave reason:\nFamily trip"));
        assert_eq!(v.rail.iter().find(|d| d.day == 7).unwrap().tip, "Tue Jul 7 · Vacation Leave");

        let half = period_view(&p, &[], &[], &[lv(7, "For Approval", "0.500000")], t());
        assert_eq!(half.rows[6].leave, "Vacation Leave (half day) · For Approval");
        assert_eq!(half.rows[6].status_tip, "OB: not filed\nCS: not filed\nLeave: For Approval · Vacation Leave (half day)");
    }
}

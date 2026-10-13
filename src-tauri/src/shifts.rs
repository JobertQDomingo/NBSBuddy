//! NBS shift names -> times, and the OB-vs-shift time check.
//!
//! Shift names are irregular ("730AM-530PM", "8AM-600PM", "1045AM- 915PM"), so times
//! are pulled out with regexes that require AM/PM: "9.5H" and "30MIN" never parse as times.

use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

pub const PREFIX_MON_THU: &str = "CW 9.5H";
pub const PREFIX_FRI: &str = "CW 10H";
pub const SUFFIX: &str = "SP 30MIN BRK";

const TIME: &str = r"(?:\d{1,2}(?::\d{2})?|\d{3,4})\s*(?:AM|PM)";
static TIME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(\d{1,2}(?::\d{2})?|\d{3,4})\s*(AM|PM)").unwrap());
static RANGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"(?i)({TIME})\s*-\s*({TIME})")).unwrap());
static CLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(\d{1,2}:\d{2}|\d{1,4})\s*(AM|PM)?$").unwrap());

#[derive(Debug, Clone, PartialEq, serde::Deserialize, Serialize)]
pub struct Shift {
    pub id: i64,
    pub name: String,
}

/// Times a shift name implies, in seconds after midnight.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParsedShift {
    pub time_in: u32,
    pub time_out: u32,
    pub hours: f64,
}

pub fn hhmm(seconds: u32) -> String {
    let s = seconds % 86_400;
    format!("{:02}:{:02}", s / 3600, s % 3600 / 60)
}

/// "730" + "AM" -> 7:30. Digits may carry a colon ("08:30").
fn clock(digits: &str, meridiem: Option<&str>) -> (u32, u32) {
    let d: String = digits.chars().filter(|c| *c != ':').collect();
    let n = |s: &str| s.parse::<u32>().unwrap_or(0);
    let (mut hour, minute) = match d.len() {
        0..=2 => (n(&d), 0),
        3 => (n(&d[..1]), n(&d[1..])),
        _ => (n(&d[..2]), n(&d[2..])),
    };
    match meridiem.map(str::to_ascii_uppercase).as_deref() {
        Some("PM") if hour != 12 => hour += 12,
        Some("AM") if hour == 12 => hour = 0,
        _ => {}
    }
    (hour, minute)
}

pub fn parse_shift_times(name: &str) -> Option<ParsedShift> {
    let mut times = TIME_RE.captures_iter(name).map(|c| {
        let (h, m) = clock(&c[1], Some(&c[2]));
        h * 3600 + m * 60
    });
    let (start, end) = (times.next()?, times.next()?);
    let mut span = end as i64 - start as i64;
    if span <= 0 {
        span += 86_400; // overnight
    }
    let hours = (span as f64 / 3600.0 * 100.0).round() / 100.0;
    Some(ParsedShift { time_in: start, time_out: end, hours })
}

/// The shifts offered for a weekday (0 = Monday): Mon-Thu "CW 9.5H ...", Fri "CW 10H ...",
/// weekends either; all must end "SP 30MIN BRK". Sorted by start time, early birds (< 7 AM) last.
pub fn filter_shifts(shifts: &[Shift], weekday: u32) -> Vec<Shift> {
    let prefixes: &[&str] = match weekday {
        4 => &[PREFIX_FRI],
        5.. => &[PREFIX_MON_THU, PREFIX_FRI],
        _ => &[PREFIX_MON_THU],
    };
    let mut out: Vec<Shift> = shifts
        .iter()
        .filter(|s| s.name.trim_end().ends_with(SUFFIX) && prefixes.iter().any(|p| s.name.starts_with(p)))
        .cloned()
        .collect();
    out.sort_by_cached_key(|s| match parse_shift_times(&s.name) {
        Some(p) => {
            let (h, m) = (p.time_in / 3600, p.time_in % 3600 / 60);
            (if h < 7 { h + 24 } else { h }, m, s.name.clone())
        }
        None => (24, 0, s.name.clone()),
    });
    out
}

// ---- OB (actual) vs CS (scheduled) ----

/// "8AM", "730AM", "13:00", "1130" -> "HH:MM".
pub fn normalise_time(value: &str) -> Option<String> {
    let c = CLOCK_RE.captures(value.trim())?;
    let (h, m) = clock(&c[1], c.get(2).map(|m| m.as_str()));
    Some(format!("{h:02}:{m:02}"))
}

pub fn parse_time_range(value: &str) -> Option<(String, String)> {
    let (a, b) = value.split_once('-')?;
    Some((normalise_time(a)?, normalise_time(b)?))
}

pub fn extract_shift_range(new_shift: &str) -> Option<(String, String)> {
    let c = RANGE_RE.captures(new_shift)?;
    parse_time_range(&format!("{}-{}", &c[1], &c[2]))
}

fn minutes(hhmm: &str) -> i32 {
    let (h, m) = hhmm.split_once(':').unwrap_or(("0", "0"));
    h.parse::<i32>().unwrap_or(0) * 60 + m.parse::<i32>().unwrap_or(0)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeStatus {
    /// Exact, unparseable, or the OB fully covers the shift.
    Ok,
    Late,
    Undertime,
    LateUndertime,
    /// Overnight windows that differ: minute comparison can't reason about them.
    Mismatch,
}

fn ranges(ob_time: &str, new_shift: &str) -> Option<[i32; 4]> {
    let (oi, oo) = parse_time_range(ob_time)?;
    let (ci, co) = extract_shift_range(new_shift)?;
    Some([minutes(&oi), minutes(&oo), minutes(&ci), minutes(&co)])
}

pub fn time_status(ob_time: &str, new_shift: &str) -> TimeStatus {
    let Some([ob_in, ob_out, cs_in, cs_out]) = ranges(ob_time, new_shift) else {
        return TimeStatus::Ok;
    };
    if (ob_in, ob_out) == (cs_in, cs_out) {
        return TimeStatus::Ok;
    }
    if ob_out < ob_in || cs_out < cs_in {
        return TimeStatus::Mismatch;
    }
    match (ob_in > cs_in, ob_out < cs_out) {
        (true, true) => TimeStatus::LateUndertime,
        (true, false) => TimeStatus::Late,
        (false, true) => TimeStatus::Undertime,
        (false, false) => TimeStatus::Ok,
    }
}

/// (minutes late, minutes undertime); (0, 0) when on time or not comparable.
pub fn late_undertime_minutes(ob_time: &str, new_shift: &str) -> (i32, i32) {
    match ranges(ob_time, new_shift) {
        Some([ob_in, ob_out, cs_in, cs_out]) if ob_out >= ob_in && cs_out >= cs_in => {
            ((ob_in - cs_in).max(0), (cs_out - ob_out).max(0))
        }
        _ => (0, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed(name: &str) -> (String, String, f64) {
        let p = parse_shift_times(name).unwrap();
        (hhmm(p.time_in), hhmm(p.time_out), p.hours)
    }

    #[test]
    fn parses_irregular_shift_names() {
        for (name, tin, tout, hours) in [
            ("CW 9.5H 730AM-530PM SP 30MIN BRK", "07:30", "17:30", 10.0),
            ("CW 9.5H 8AM-6PM SP 30MIN BRK", "08:00", "18:00", 10.0),
            ("CW 9.5H 8AM-600PM SP 30MIN BRK", "08:00", "18:00", 10.0),
            ("CW 9.5H 1015AM-0815PM SP 30MIN BRK", "10:15", "20:15", 10.0),
            ("CW 10H 730AM-6PM SP 30MIN BRK", "07:30", "18:00", 10.5),
            ("CW 10H 1045AM- 915PM SP 30MIN BRK", "10:45", "21:15", 10.5),
            ("CW 9.5H 08:30 AM - 07:00 PM", "08:30", "19:00", 10.5),
            ("CW 10H 10PM-6AM SP 30MIN BRK", "22:00", "06:00", 8.0),
        ] {
            assert_eq!(parsed(name), (tin.into(), tout.into(), hours), "{name}");
        }
        assert_eq!(parse_shift_times("CW 9.5H NO TIMES HERE SP 30MIN BRK"), None);
    }

    #[test]
    fn filters_by_weekday_and_sorts_by_start() {
        let s = |id, name: &str| Shift { id, name: name.into() };
        let all = vec![
            s(1, "CW 9.5H 10AM-8PM SP 30MIN BRK"),
            s(2, "CW 9.5H 8AM-6PM SP 30MIN BRK"),
            s(3, "CW 10H 8AM-7PM SP 30MIN BRK"),
            s(4, "CW 9.5H 8AM-6PM"),
            s(5, "REGULAR 8AM-5PM SP 30MIN BRK"),
            s(6, "CW 9.5H 5AM-3PM SP 30MIN BRK"),
        ];
        let ids = |wd| filter_shifts(&all, wd).iter().map(|s| s.id).collect::<Vec<_>>();
        assert_eq!(ids(0), [2, 1, 6]); // Monday; 5 AM sorts after the day shifts
        assert_eq!(ids(4), [3]);
        assert_eq!(ids(5), [3, 2, 1, 6]); // same start: by name, "CW 10H" < "CW 9.5H"
    }

    #[test]
    fn normalises_times() {
        for (raw, want) in [
            ("8", Some("08:00")),
            ("8:15", Some("08:15")),
            ("8AM", Some("08:00")),
            ("6PM", Some("18:00")),
            ("12AM", Some("00:00")),
            ("12PM", Some("12:00")),
            ("13:00", Some("13:00")),
            ("730AM", Some("07:30")),
            ("1130", Some("11:30")),
            ("530PM", Some("17:30")),
            ("", None),
            ("noon", None),
        ] {
            assert_eq!(normalise_time(raw).as_deref(), want, "{raw}");
        }
        assert_eq!(parse_time_range("8AM"), None);
        assert_eq!(extract_shift_range("Work From Home"), None);
        assert_eq!(
            extract_shift_range("CW 10H 1130AM-10PM SP 30MIN BRK"),
            Some(("11:30".into(), "22:00".into()))
        );
    }

    #[test]
    fn classifies_ob_against_shift() {
        let shift = "CW 9.5H 730AM-530PM SP 30MIN BRK"; // 07:30-17:30
        use TimeStatus::*;
        for (ob, want) in [
            ("07:30-17:30", Ok),
            ("07:26-18:00", Ok), // covers the whole shift
            ("07:35-17:30", Late),
            ("08:00-19:00", Late),
            ("07:26-17:00", Undertime),
            ("07:35-17:00", LateUndertime),
        ] {
            assert_eq!(time_status(ob, shift), want, "{ob}");
        }
        assert_eq!(time_status("22:00-05:00", "CW 10H 10PM-6AM SP 30MIN BRK"), Mismatch);
        assert_eq!(time_status("07:30-17:30", "Work From Home"), Ok);
        assert_eq!(time_status("11:30-22:00", "CW 10H 1130AM-10PM SP 30MIN BRK"), Ok);
        assert_eq!(late_undertime_minutes("07:35-17:00", shift), (5, 30));
    }
}

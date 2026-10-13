//! "Remember me": the NBS login kept in Windows Credential Manager (encrypted per
//! Windows user). Nothing is written to disk by the app itself.

use keyring::Entry;

fn entry() -> Option<Entry> {
    Entry::new("NBSBuddy", "nbs-login").ok()
}

pub fn save(username: &str, password: &str) {
    if let Some(e) = entry() {
        let _ = e.set_password(&serde_json::json!({ "u": username, "p": password }).to_string());
    }
}

pub fn load() -> Option<(String, String)> {
    let raw = entry()?.get_password().ok()?;
    let v: serde_json::Value = serde_json::from_str(&raw).ok()?;
    Some((v["u"].as_str()?.to_string(), v["p"].as_str()?.to_string()))
}

pub fn clear() {
    if let Some(e) = entry() {
        let _ = e.delete_credential();
    }
}

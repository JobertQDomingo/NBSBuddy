# NBSBuddy

Your daily NBS sidekick: file your work-from-home **Official Business (OB)** and **Change Shift (CS)** in a couple of clicks, see which days still need action before payroll cutoff, and delete a mistaken filing while it's still *For Approval*.

**Download:** https://jobertqdomingo.github.io/NBSBuddy/ (Windows 10/11). You need ZeroTier connected and your NBS account.

## What it does

- **Board:** every day of the semi-monthly payroll period (1–15, 16–end) with one verdict: *Done*, *Waiting*, *Due today* or *Needs action* (missing, disapproved, or OB times that don't match the shift). A status strip says how many days need action before cutoff.
- **Leave:** your NBS leave filings show on the board (read-only; file leave in NBS). A whole-day leave stands in for OB + CS; a half day still needs both. A disapproved leave needs action like any other filing.
- **File CS/OB** (the lemon + button, or Ctrl+N): pick the shift, and the OB times and hours come from it. It files both, skipping whichever NBS already has. Weekends ask first.
- **Delete:** open a filed day and delete its OB and/or CS while NBS still has them *For Approval*, then file again.
- **Sign-in:** your NBS account. "Remember me" keeps the login in Windows Credential Manager so the app signs in by itself next time. Log out forgets it.
- **Profile:** your work details from NBS.
- **Settings** (the gear, or Ctrl+,): the OB reason, CS reason and NBS customer (`315 CORPORATE OFFICE` out of the box) every new filing starts with. Saved on this PC; the File CS/OB dialog can still change the reasons for one filing.

## Develop

Needs Node 20+, Rust (stable, MSVC) and the Visual Studio C++ build tools.

```sh
npm install
npm run tauri dev      # run the app
npm run check          # type-check the Svelte/TS UI
cargo test --manifest-path src-tauri/Cargo.toml   # verdicts, shift parsing, payloads
```

Read-only check against the real NBS (needs ZeroTier; it never files or deletes anything). Without `NBS_USER`/`NBS_PASS` it uses the app's remembered login:

```sh
NBS_USER=you NBS_PASS=secret cargo test --manifest-path src-tauri/Cargo.toml live -- --ignored --nocapture
```

### Layout

| Path | What |
|---|---|
| `src-tauri/src/nbs.rs` | NBS API client: login + token refresh, filings, create, void (delete), profile |
| `src-tauri/src/summary.rs` | Payroll periods, per-day verdicts, notes, status strip, day rail |
| `src-tauri/src/shifts.rs` | Shift-name parsing and late/undertime/mismatch checks |
| `src-tauri/src/creds.rs` | "Remember me" in Windows Credential Manager |
| `src-tauri/src/lib.rs` | The Tauri commands the UI calls |
| `src/` | Svelte UI: `Onboarding`, `Login`, `Board`, `FileDialog`, `Profile`, `Settings` |
| `docs/` | GitHub Pages download page |
| `DESIGN.md` | The Sticker Board design system |

All NBS traffic, tokens and passwords stay in Rust. The UI only calls the commands in `lib.rs`.

## Release

1. Bump `version` in `package.json` (it's the app's only version number).
2. Commit, then tag and push: `git tag v0.2.0 && git push origin v0.2.0`.
3. The **Release** workflow runs the tests, builds the installer and publishes a GitHub Release with `NBSBuddy-setup.exe`. The download page always points at the latest one.

GitHub Pages serves `docs/` from `main` (Settings → Pages → Deploy from a branch → `main` / `/docs`).

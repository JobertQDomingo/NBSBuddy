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
- **Updates:** when it starts, NBSBuddy asks GitHub for a newer version and installs it before you sign in. Turn that off in Settings to get a lemon **Update to X** button instead. The check sends nothing about you.

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
| `src-tauri/src/lib.rs` | The Tauri commands the UI calls, including the update check and install |
| `src/` | Svelte UI: `Onboarding`, `Login`, `Board`, `FileDialog`, `Profile`, `Settings` |
| `docs/` | GitHub Pages download page |
| `DESIGN.md` | The Sticker Board design system |
| `RELEASING.md` | How to ship a change and a release |
| `.claude/skills/release/` | The `/release` command for Claude Code |

All NBS traffic, tokens and passwords stay in Rust, and so does the update check. The UI only calls the commands in `lib.rs`, plus Tauri's own version lookup.

## Release

The full guide is [RELEASING.md](RELEASING.md). It covers both loops, the `/release` command in Claude Code that automates a release, and what to do when something goes wrong. In short: every change goes through a PR into `main` and releases nothing. To ship what's on `main`:

1. Branch `release/vX.Y.Z`, run `npm version X.Y.Z --no-git-tag-version` (`package.json` holds the app's only version number), and commit `chore: release vX.Y.Z`.
2. Open a PR. When CI is green, squash-merge it.
3. The **Release** workflow tests, builds and signs the installer, then publishes the GitHub Release and its `vX.Y.Z` tag. NBSBuddy 0.2.0 and later update themselves, and the download page always points at the newest one.

Never push a tag by hand. Publishing creates it.

**Signing key.** Updates are signed with `~/.tauri/nbsbuddy.key`, which is backed up off this PC and stored as the `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` repo secrets. The public half is in `tauri.conf.json`. If the key is lost, installed copies can't accept another update. A local `npx tauri build` needs both variables set; `npm run tauri dev` doesn't.

GitHub Pages serves `docs/` from `main` (Settings → Pages → Deploy from a branch → `main` / `/docs`).

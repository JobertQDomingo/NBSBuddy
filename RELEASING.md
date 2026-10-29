# Shipping NBSBuddy

How a change gets from your editor to everyone's NBSBuddy. Read it top to bottom the first few times, then use the headings as a checklist.

## The idea

- `main` is always ready to ship. Changes reach it only through a pull request (PR) with green CI.
- `package.json` holds the app's only version number.
- **Merging a change releases nothing.** Merging a **version bump** releases that version: GitHub builds and signs it, then publishes it. Every NBSBuddy 0.2.0 or later installs it at its next start.
- Never push a tag by hand. Publishing a release creates its tag.

```
a change:   branch → PR → CI ✓ → squash-merge → main                   (nothing released)
a release:  release/vX.Y.Z → PR → CI ✓ → merge → Release workflow → everyone updates
```

## Loop 1: every change (feature, fix, docs, chore)

```powershell
git switch main
git pull
git switch -c feat/short-name        # or fix/…, docs/…, chore/…, ci/…
# work and commit, as many commits as you like
git push -u origin feat/short-name
gh pr create --web
gh pr checks --watch                 # CI: type-check and tests, 5–15 minutes
```

When CI is green, click **Squash and merge**, then **Delete branch**, then run:

```powershell
git switch main
git pull
git branch -D feat/short-name        # capital D: git can't tell a squashed branch was merged
```

- **Your PR title becomes a line in the next release's notes**, for example `feat: show sick leave balance`. Write it for the person reading them.
- **The next release ships everything on `main`**, so only merge what's ready.

## Loop 2: a release

### Pick the version number

| Since the last release | Bump | Example |
|---|---|---|
| Only `fix`, `docs`, `chore`, `ci` | patch | 0.2.0 → 0.2.1 |
| At least one `feat` | minor | 0.2.1 → 0.3.0 |
| A breaking change, or you call it "1.0" | major | 0.x → 1.0.0 |

To see what would ship, run `git log v0.2.0..HEAD --oneline`, replacing `v0.2.0` with the newest tag (`git describe --tags --abbrev=0` prints it).

### Automate it with `/release`

The repo includes a Claude Code skill, [`.claude/skills/release/SKILL.md`](.claude/skills/release/SKILL.md), that runs the whole release for you and **always asks before it publishes**.

**Run it:**

1. Open Claude Code in this repo, either the terminal (`claude`) or the VS Code panel. Start a new session after pulling a new version of the skill, because skills load when a session starts.
2. Type one of:

   | You type | It releases |
   |---|---|
   | `/release` | It suggests patch or minor from the commits, and you confirm |
   | `/release patch` | The next patch, e.g. 0.2.0 → 0.2.1 |
   | `/release minor` | The next minor, e.g. 0.2.1 → 0.3.0 |
   | `/release 0.3.0` | Exactly that version |

   Typing `/` shows the list of commands. `release` is among them as *(project)*.

**What it does, and where it waits for you:**

| # | It… | You |
|---|---|---|
| 1 | Checks your working tree is clean, `main` is up to date, and CI on `main` is green. Then it lists what ships. | Commit or stash anything it says is uncommitted |
| 2 | Proposes the version number | **Confirm**, or give a different one |
| 3 | Makes `release/vX.Y.Z`, bumps the version, commits, pushes and opens the PR | Approve the commands if Claude Code asks |
| 4 | Waits for CI on the PR | Nothing |
| 5 | **Asks: "Merging publishes vX.Y.Z to everyone… Merge now?"** | **Yes** to publish, **no** to stop. The PR stays open, and you can merge it yourself later. |
| 6 | Squash-merges, then watches the Release run (15–25 minutes) | Nothing |
| 7 | Checks the release isn't a draft, all 4 files are there and `latest.json` shows the new version. Then it gives you the link. | Install-check it in Settings if you like |

**Safety:**
- It only runs when you type `/release`. Claude never starts a release on its own (`disable-model-invocation: true`).
- It never merges without a yes to step 5, even if you said "go ahead" earlier.
- It never creates tags.
- If anything fails, it stops and shows you the error. A failed release stays a hidden draft, so nobody gets it.

**Changing how it works:** edit `.claude/skills/release/SKILL.md`. It's plain instructions, not code. Ship the edit like any change (Loop 1), then start a new Claude Code session.

### By hand

The same steps, if you'd rather type them:

1. Check `main` is green with `gh run list --workflow ci.yml --branch main --limit 1`.
2. Branch, bump and open the PR:
   ```powershell
   git switch main
   git pull
   git switch -c release/v0.2.1
   npm version 0.2.1 --no-git-tag-version   # never drop the flag: tags are the workflow's job
   git diff --stat                           # only package.json and package-lock.json
   git commit -am "chore: release v0.2.1"
   git push -u origin release/v0.2.1
   gh pr create --web
   ```
   Title: `chore: release v0.2.1`. Body: `Releases **v0.2.1**`, followed by the PRs that ship.
3. Run `gh pr checks --watch`. When it's green, click **Squash and merge**. **This is the moment it goes to everyone.**
4. Watch it with `gh run list --workflow release.yml --limit 1`, then `gh run watch`. A release takes 15–25 minutes, or follow it at **Actions → Release**.
5. Check it's live:
   ```powershell
   gh release view v0.2.1
   curl.exe -sL https://github.com/JobertQDomingo/NBSBuddy/releases/latest/download/latest.json
   ```
   It shouldn't be a draft, it should have 4 files, and `latest.json` should show the new version. GitHub may serve the old file for a minute.
6. Clean up with `git switch main`, `git pull`, then `git branch -D release/v0.2.1`.

### What the Release workflow does

1. **`version`** (seconds, on Linux): reads `package.json`. If that version's tag already exists, it stops. That's why ordinary merges release nothing.
2. **`release`** (on Windows):
   - Runs the tests.
   - Removes any draft left by a failed run.
   - Builds and signs the installer into a **hidden draft**, then uploads `NBSBuddy-setup.exe` for the download page.
   - Checks that all 4 files are there: `latest.json`, the installer, its `.sig` and `NBSBuddy-setup.exe`.
   - Writes the notes (the install text plus the PR titles) and publishes. Publishing creates the tag.

### How people get it

- **Automatic** (the default): at their next start, NBSBuddy shows "Checking for updates…", then "Updating NBSBuddy to X…", then reopens on X.
- **Automatic off** (Settings): a lemon **Update to X** button appears on the board and asks before restarting. If an install fails, the dialog says why and nothing changes.
- **Settings → Updates → Check for updates** says "You're up to date." or names the newer version.
- **Anyone still on 0.1.0** has to install from the download page once, because 0.1.0 has no updater.

## When something goes wrong

| Situation | What to do |
|---|---|
| CI is red on a PR | Open the failing step (PR → Checks, or `gh run view --log-failed`), fix it, commit and push. CI runs again. Never merge on red. |
| The Release run is red | Nobody got anything: it's a hidden draft. Fix it in a normal PR, merge, then run `gh workflow run release.yml` to finish the release. |
| A published version has a bug | Fix it in a PR, then release a patch. Never delete or move a published release or tag, because people only move forward. |
| You merged changes but forgot the bump | Nothing was released. Open a release PR. |
| "Node.js 20 is deprecated" warnings in Actions | Harmless. Move the named action (e.g. `actions/checkout@v4`) to its newest major version in a `chore/` PR. |
| `main` rejects a direct push | The **Protect Main** ruleset is working. Use a branch and a PR. |
| The signing key or its password is lost | No installed copy can accept a new update. Make a new key, put its `.pub` in `src-tauri/tauri.conf.json`, replace both secrets and release. Everyone then reinstalls by hand once. |

## The signing key

- **Private key:** `~/.tauri/nbsbuddy.key`, protected by a password and backed up off this PC. GitHub holds copies in the secrets `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`. `gh secret list` shows their names; GitHub never shows the values.
- **Public key:** `plugins.updater.pubkey` in `src-tauri/tauri.conf.json`. It's safe to share.
- **What the signature protects:** installed apps refuse an update whose signature doesn't match, or one signed for a different version than `latest.json` announces (`requireSignedVersion`).
- **Never** run `tauri signer generate -f` on that path again. It would replace the key and break updates for everyone.
- A local `npx tauri build` needs both variables set. `npm run tauri dev` doesn't.

## Rehearse an update on your PC

You only need this when you change the updater itself: `check_update` or `install_update` in `src-tauri/src/lib.rs`, the `plugins.updater` config, or the plugin version. It builds two versions of a side-by-side app, "NBSBuddy Rehearsal", that updates from a server on your PC. Nothing is published, and your real NBSBuddy isn't touched.

**Don't press Log out in the rehearsal app.** It shares your remembered login with your real NBSBuddy.

1. **Set up a folder.** Make `$HOME\nbs-rehearsal` and put `0.2.0.json` in it, plus a copy named `0.2.1.json` with `"version": "0.2.1"`. Any two versions work, because it's a separate app.
   ```json
   {
     "version": "0.2.0",
     "productName": "NBSBuddy Rehearsal",
     "mainBinaryName": "nbsbuddy-rehearsal",
     "identifier": "io.github.jobertqdomingo.nbsbuddy.rehearsal",
     "plugins": {
       "updater": {
         "endpoints": ["http://localhost:8765/latest.json"],
         "dangerousInsecureTransportProtocol": true
       }
     }
   }
   ```
2. **Build both versions, signed.** Run this from the repo folder. The password prompt hides what you type.
   ```powershell
   $env:TAURI_SIGNING_PRIVATE_KEY = (Get-Content "$HOME\.tauri\nbsbuddy.key" -Raw).Trim()
   $p = Read-Host "Signing key password" -AsSecureString
   $env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = [Net.NetworkCredential]::new("", $p).Password
   $out = "src-tauri\target\release\bundle\nsis"
   foreach ($v in "0.2.0", "0.2.1") {
     npx tauri build --config "$HOME\nbs-rehearsal\$v.json"
     Copy-Item "$out\NBSBuddy Rehearsal_${v}_x64-setup.exe" "$HOME\nbs-rehearsal\setup-$v.exe"
     Copy-Item "$out\NBSBuddy Rehearsal_${v}_x64-setup.exe.sig" "$HOME\nbs-rehearsal\setup-$v.exe.sig"
   }
   Remove-Item Env:TAURI_SIGNING_PRIVATE_KEY, Env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD
   ```
3. **Write `latest.json`.** Keep `ascii`: PowerShell 5.1's `utf8` adds a hidden mark the updater can't read.
   ```powershell
   $sig = (Get-Content "$HOME\nbs-rehearsal\setup-0.2.1.exe.sig" -Raw).Trim()
   @{ version = "0.2.1"; platforms = @{ "windows-x86_64" = @{ signature = $sig; url = "http://localhost:8765/setup-0.2.1.exe" } } } |
     ConvertTo-Json -Depth 4 | Set-Content "$HOME\nbs-rehearsal\latest.json" -Encoding ascii
   ```
4. **Serve it** (start and stop it as the tests say):
   ```powershell
   python -m http.server 8765 --bind 127.0.0.1 --directory "$HOME\nbs-rehearsal"
   ```
5. **Test in this order:**

   | Test | Do | Expect |
   |---|---|---|
   | A. Offline | Server off. Install `setup-0.2.0.exe` and open it. | It starts normally and shows the board. |
   | C. Automatic | Server on. Close and reopen the app. | It updates to 0.2.1 by itself and reopens. |
   | B. Button | Server off. Reinstall 0.2.0, untick automatic updates in Settings, then close. Server on. Reopen. | The **Update to 0.2.1** button works. |
   | D1. Swapped installer | Rewrite `latest.json` with the signature from `setup-0.2.0.exe.sig`. Reinstall 0.2.0 and click Update. | Refused: "signature verification failed" |
   | D2. Relabelled old release | Rewrite `latest.json`: version 0.2.1, `url` pointing at `setup-0.2.0.exe`, with its own signature. Click Update. | Refused: "signed for version 0.2.0 but … announced 0.2.1" |

6. **Clean up.**
   - Stop the server.
   - Uninstall **NBSBuddy Rehearsal** in Windows Settings → Apps.
   - Delete `%LOCALAPPDATA%\io.github.jobertqdomingo.nbsbuddy.rehearsal` and `$HOME\nbs-rehearsal`.

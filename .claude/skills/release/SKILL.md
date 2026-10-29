---
name: release
description: Use when the user types /release to publish a new NBSBuddy version to GitHub Releases and every installed copy.
disable-model-invocation: true
argument-hint: "[patch | minor | major | X.Y.Z]"
---

# Release NBSBuddy

A release is a PR that changes only the version in `package.json`. Merging it to `main` runs `.github/workflows/release.yml`, which tests, builds and signs the installer, then publishes `vX.Y.Z` and creates its tag. Every NBSBuddy 0.2.0 or later installs it at its next start. Never create or push a tag yourself. `RELEASING.md` describes the same steps for doing them by hand.

Run every command with Bash. At the first check that fails, stop and report it.

## 1. Start from a clean, current, green main
- `git status --porcelain` prints nothing. If it prints anything, stop and list what's uncommitted.
- `git switch main && git pull --ff-only`
- `gh run list --workflow ci.yml --branch main --limit 1` shows `completed` and `success`. If CI on `main` is red or still running, stop and say so.
- `LAST=$(git describe --tags --abbrev=0)`, then `git log $LAST..HEAD --oneline` lists what ships. If the list is empty, stop: there's nothing to release.

## 2. Pick the version
- If `$ARGUMENTS` is a version like `0.3.0`, use it. If it's `patch`, `minor` or `major`, work out X.Y.Z from the version in `package.json`.
- With no argument, suggest one. Any `feat` commit since `$LAST` means minor. Only `fix`, `docs`, `chore` or `ci` commits mean patch. Never suggest major; use it only when the user asks.
- Show the current version, the new version and the commit list. Wait for the user to confirm the number.
- If `git ls-remote --tags origin vX.Y.Z` prints a line, stop: that version is already released.

## 2b. Check What's new
- A release with any `feat` commit since `$LAST` needs its own What's new steps: `grep -n "VERSIONS =" src/WhatsNew.svelte` must list `"X.Y.Z"`. Only `fix`, `docs`, `chore` or `ci` commits need none.
- If it's missing, stop. List the features without a step and offer to design them in a normal feature PR that merges before this release (the release PR only changes the version). One step per user-facing feature of X.Y.Z only, never repeating earlier versions' features: a short Buddy-voice headline, one or two plain sentences, and a small demo drawn with the board's own stickers, like the existing steps. Append `"X.Y.Z"` to `VERSIONS` once per new step.
- No version in `VERSIONS` may be newer than X.Y.Z. A step tagged with a guessed version that this release isn't (say `"0.3.0"` when releasing 0.2.1) would hide the real 0.3.0's steps later. If you find one, stop and retag it to X.Y.Z in a fix PR.

## 3. Open the release PR
    git switch -c release/vX.Y.Z
    npm version X.Y.Z --no-git-tag-version
    git diff --stat
    git commit -am "chore: release vX.Y.Z"
    git push -u origin release/vX.Y.Z
    gh pr create --title "chore: release vX.Y.Z" --body "<body>"
- `git diff --stat` must list only `package.json` and `package-lock.json`.
- The body is `Releases **vX.Y.Z**: everything merged since $LAST.`, then one bullet per commit title from step 1.
- Note the PR number.

## 4. Wait for CI
- Run `gh pr checks <PR> --watch` in the background. It can take 5–15 minutes.
- If it reports no checks yet, wait a few seconds and run it again.
- If a check fails, show `gh run view <run-id> --log-failed`, then stop. Don't merge.

## 5. Ask before merging, every time
Ask: "CI is green. Merging publishes vX.Y.Z to everyone, and their NBSBuddy installs it at its next start. Merge now?"

Merge only on a yes given to this question, even if the user said "go ahead" earlier:

    gh pr merge <PR> --squash --delete-branch
    git switch main && git pull --ff-only

## 6. Watch the release run
- `SHA=$(gh pr view <PR> --json mergeCommit --jq .mergeCommit.oid)`
- `gh run list --workflow release.yml --commit $SHA` gives the run ID. The run can take a few seconds to appear.
- Run `gh run watch <id> --exit-status` in the background. A release takes 15–25 minutes.
- If the run fails, the release stays a hidden draft and nobody gets it. Show `gh run view <id> --log-failed`, then stop. The fix goes through a normal fix PR. Once that merges, `gh workflow run release.yml` finishes the release.

## 7. Confirm it's live
- `gh release view vX.Y.Z --json isDraft,assets`: not a draft, and the assets are `latest.json`, `NBSBuddy_X.Y.Z_x64-setup.exe`, its `.sig` and `NBSBuddy-setup.exe`.
- `curl -sL https://github.com/JobertQDomingo/NBSBuddy/releases/latest/download/latest.json` shows `"version": "X.Y.Z"`. GitHub may serve the old file for a minute.
- `git fetch --tags`, then `git tag` lists `vX.Y.Z`.
- Report the release URL and what shipped. Say that installed copies update at their next start.

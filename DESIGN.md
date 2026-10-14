---
name: NBSBuddy
description: "Sticker Board": a loud, flat sticker board for filing and checking OB + CS payroll filings.
colors:
  ink: "#111111"
  canvas: "#cbbcff"
  surface: "#ffffff"
  surface-alt: "#ebe4ff"
  hover: "#f3efff"
  text-muted: "#5b5275"
  on-canvas-muted: "#3a3159"
  lemon: "#ffe14d"
  lemon-hover: "#ffd426"
  lemon-disabled: "#f4ecb8"
  mint: "#5ee3a1"
  tomato: "#ff5a4e"
  row-weekend: "#ebe4ff"
  row-warning: "#fff09a"
  row-mismatch: "#ffcdc6"
  accent: "#3d2bff"
  on-accent: "#ffffff"
  danger-line: "#d0201a"
  selection: "#d9d2ff"
typography:
  display:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "38px"
    fontWeight: 900
    fontStretch: "125%"
  dialog-title:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "24px"
    fontWeight: 900
    fontStretch: "125%"
  body:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "10pt"
    fontWeight: 400
  button:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "10pt"
    fontWeight: 700
  table-header:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "10pt"
    fontWeight: 800
    textTransform: "uppercase"
  card-label:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "8pt"
    fontWeight: 900
    textTransform: "uppercase"
    letterSpacing: "0.08em"
  pill:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "9pt"
    fontWeight: 800
  keycap:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "12px"
    fontWeight: 800
    lineHeight: 1.5
  page-display:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "clamp(42px, 5.4vw, 78px)"
    fontWeight: 900
    lineHeight: 0.98
    letterSpacing: "-0.03em"
  page-headline:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "28px"
    fontWeight: 900
    lineHeight: 1.12
    letterSpacing: "-0.015em"
  page-lede:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "21px"
    fontWeight: 400
    lineHeight: 1.5
  page-body:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "18px"
    fontWeight: 400
    lineHeight: 1.55
  page-small:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "15px"
    fontWeight: 600
  page-wordmark:
    fontFamily: "Archivo, Segoe UI, sans-serif"
    fontSize: "min(13.2vw, 196px)"
    fontWeight: 900
    lineHeight: 0.8
    letterSpacing: "-0.04em"
rounded:
  pill: "6px"
  tile: "7px"
  control: "8px"
  container: "12px"
  popup: "0"
borders:
  rest: "2px"
  heavy: "3px"
  today: "4px"
  focus: "3px"
shadows:
  pill: "2px 2px 0 #111111"
  tile: "2px 2px 0 #111111"
  button: "3px 3px 0 #111111"
  dialog-card: "4px 4px 0 #111111"
  card: "5px 5px 0 #111111"
spacing:
  xs: "4px"
  sm: "8px"
  gap: "10px"
  md: "12px"
  lg: "16px"
components:
  button:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    typography: "{typography.button}"
    rounded: "{rounded.control}"
    border: "2px solid {colors.ink}"
    shadow: "{shadows.button}"
    padding: "5px 14px"
  button-primary:
    backgroundColor: "{colors.lemon}"
    textColor: "{colors.ink}"
    fontWeight: 800
  button-primary-hover:
    backgroundColor: "{colors.lemon-hover}"
  button-disabled:
    backgroundColor: "{colors.surface-alt}"
    textColor: "{colors.text-muted}"
    border: "2px solid {colors.text-muted}"
    shadow: "none"
  status-pill-done:
    backgroundColor: "{colors.mint}"
    textColor: "{colors.ink}"
  status-pill-waiting:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
  status-pill-due-today:
    backgroundColor: "{colors.lemon}"
    textColor: "{colors.ink}"
  status-pill-needs-action:
    backgroundColor: "{colors.tomato}"
    textColor: "{colors.ink}"
    border: "3px solid {colors.ink}"
  card:
    backgroundColor: "{colors.surface}"
    border: "2px solid {colors.ink}"
    rounded: "{rounded.container}"
    shadow: "{shadows.card}"
  table-header:
    backgroundColor: "{colors.lemon}"
    textColor: "{colors.ink}"
    typography: "{typography.table-header}"
    padding: "7px 8px"
  table-row:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    height: "30px"
    borderBottom: "2px solid {colors.ink}"
  kbd:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    typography: "{typography.keycap}"
    rounded: "{rounded.pill}"
    padding: "0 6px"
  page-download-button:
    backgroundColor: "{colors.lemon}"
    textColor: "{colors.ink}"
    rounded: "{rounded.container}"
    padding: "15px 24px"
  page-download-button-hover:
    backgroundColor: "{colors.lemon-hover}"
---

# Design System: Payroll Filings

## Overview

**Creative North Star: "Sticker Board"**

Payroll is a loud, flat sticker board. Every element has a thick ink outline. Blocks are saturated, and every day's verdict is a sticker stuck on the board. Shadows are hard ink copies of a shape, offset down-right, with no blur. The design rejects the grey admin dashboard with its one blue accent. The user asked for "super extra, like all the 21st century designs" and picked this direction on the decision page.

**The story:** the user opens the app and reads one huge line saying how many days need action before cutoff. They see the period as a rail of day stickers. They file with the lemon button or by double-clicking a row, and then they leave.

**Key characteristics:**
- Lilac canvas, white cards, 2px ink (#111) outlines on everything.
- Hard offset ink shadows: 2px on pills and tiles, 3px on buttons, 4px on dialog cards, 5px on the main cards. Never blurred.
- Lemon is the primary action and the table header. Mint, lemon and tomato are the verdicts.
- Indigo (#3d2bff) is used only for focus rings and the selected-row outline.
- Archivo throughout, with Archivo Expanded Black for the scale-contrast headline.
- Outline weight carries urgency: 2px at rest, 3px for needs-action, 4px for today.
- App controls drop onto their shadow in one step, with no transition. State changes never animate.
- Motion lives in two places only, onboarding and the download page: stickers are slapped onto the board, the day row lands on one downbeat, and Buddy blinks and watches the pointer.
- Buddy speaks: the mascot beside a sticker speech bubble whose tail points at him, in onboarding and in every section of the download page.

Tokens and shared components live in [src/app.css](src/app.css); each screen is a Svelte file in [src/](src/). The design started as the Sticker Board theme of the earlier Qt app (FilingsValidator) and carries over unchanged. The download page ([docs/index.html](docs/index.html), GitHub Pages) is one static file that restates the same tokens in its own `:root`, adds two easing tokens (`--land`, `--out`), and runs its own larger type scale.

## Colors

### Ground and ink
- **Ink** (#111111): every outline, every hard shadow, and all body text. Text on every sticker fill is Ink.
- **Lilac Canvas** (#cbbcff): the window and dialog ground.
- **Surface** (#ffffff): cards, the table, fields, secondary buttons, and Waiting pills.
- **Surface Alt** (#ebe4ff): read-only fields, disabled controls, the pressed secondary button.
- **Hover** (#f3efff): hover fill for secondary buttons, combos and date fields.
- **Muted text** (#5b5275 on white or washes, ≥ 6:1). **On-canvas muted** (#3a3159, 6.9:1) is for secondary text placed directly on the lilac: the footer and the dialog subtitle.

### Sticker fills (always Ink text and an Ink outline)
- **Lemon** (#ffe14d): the primary button, the table header, Due-today pills, the selected period cell, and attention notes. Hover #ffd426, disabled #f4ecb8.
- **Mint** (#5ee3a1): Done.
- **Tomato** (#ff5a4e): Needs action, error notes, and the danger headline.

### Row washes (the Note column always says why in words)
- **#ebe4ff:** weekend and upcoming rows (text recedes to muted).
- **#fff09a:** late, undertime, and due-today rows.
- **#ffcdc6:** needs-action rows.
- Precedence: needs action > due today or late/undertime > weekend or upcoming > plain.

### Interaction
- **Indigo** (#3d2bff): 3px focus rings, the selected-row outline, the focused table frame, and the calendar selection (white text on it). Never used for status or decoration.
- **Danger line** (#d0201a): the 3px border on an invalid field (5.4:1 on white).
- **Selection** (#d9d2ff): text selection inside fields, with Ink text.

## Typography

**Archivo** (SIL OFL, bundled as `assets/fonts/Archivo.ttf`, a variable font with `wdth` 62–125 and `wght` 100–900). Segoe UI is the fallback.

- **Display:** Archivo Expanded Black (wght 900, wdth 125, using the real width axis, not a synthetic stretch). The status headline starts at 38px and shrinks in 1px steps down to 26px so it fits beside the day rail. The dialog title is 24px.
- **Body:** 10pt / 400. Covers table cells, fields, form labels and detail lines.
- **Buttons:** 700. The primary button and the period label use 800.
- **Table header:** 800, uppercase.
- **Card labels** (fieldset legends "DAY AND SHIFT", "REASONS AND CUSTOMER", "FILING DEFAULTS"): 8pt / 900, uppercase, 108% letter spacing, kerning off. Archivo pulls "Y " together when kerning is on.
- **Pills:** 9pt / 800. **Rail numbers:** 10pt / 800. **CUTOFF tag:** 6.5pt / 900. **Keycaps:** 12px / 800.
- **Onboarding:** the step title is display type at 38px (-0.02em); bubble copy is 15px; today's line is 15px / 800.
- **Strip meta line** ("Cutoff Wed, Jul 15 · 4 workdays left · …"): 800.
- Use tabular numerals for times and hours.

### Download page scale
The page reads at arm's length, so it runs its own steps: **15 / 18 / 21 / 28 / 78**, plus the footer wordmark.
- **Display** (h1 and section h2): Expanded Black, `clamp(42px, 5.4vw, 78px)`, line-height 0.98, -0.03em, balanced wrap, two lines at most.
- **Headline** 28px / 900 at wdth 112 (the trust claims and the SmartScreen question); 21px under 520px.
- **Lede** 21px: the hero sentence (38ch max) and every bubble's paragraph; 18px under 520px. Step titles and the Download label are 21px / 900 at wdth 112.
- **Body** 18px / 400, line-height 1.55 (claims, steps; today's line at 800).
- **Small** 15px / 600: notes and the version line in on-canvas-muted, nav links at 700, keycaps and the small Download at 800.
- **Footer wordmark:** "NBSBuddy" in Expanded Black at `min(13.2vw, 196px)`, line-height 0.8, -0.04em, on one line, its bottom cropped by the page edge.

## Layout

The window is a fixed 1133×778 (it can't be resized or maximized), laid out as a single column with 14–16px margins and 12px vertical spacing; the table's Reasons column takes the leftover width.
1. **Top bar** (8px gaps): on the left, the joined period stepper with a 3px shadow, plus a "Back to current period" link that appears only when you've moved away. On the right, three square icon buttons drawn in ink (refresh, the lemon plus that opens **File CS/OB**, and the settings gear), each naming itself and its shortcut in a tooltip and accessible name, then the user chip (initials sticker + first name) that opens the profile.
2. **Status strip card** (5px shadow):
   - Left: the display headline inside a tone sticker, then the detail line ("To file: Jul 7 · Disapproved, refile in NBS: Jul 8 · Late/undertime: …"), then the meta line.
   - Right: the day rail.
3. **Table card** (5px shadow) takes the remaining height. Columns are Date 104, Status 128, OB time 104, OB hrs 72, Shift 252 and Note 236 (px), and Reasons stretches. Rows are 31px and single-line, with the full text in tooltips.
4. **Footer:** an on-canvas-muted 600-weight line above a 2px ink rule ("Loaded Jul 1–15 at 9:42 AM.").

The File CS/OB dialog is one column with 16px margins and 12px gaps, at least 480px wide:
1. The title.
2. A subtitle note, which turns into a lemon sticker when it needs attention.
3. A **Day and shift** card (4px shadow): the date, the shift dropdown, and the time row ("07:30 to 17:30 · **10** hours").
4. A **Reasons** card (4px shadow): OB and CS reason fields that auto-grow from 1 to 12 lines.
5. A tomato error sticker.
6. A tomato **Delete OB/CS…** button at the left when the date has a filing still *For Approval*, then the lemon **File CS/OB** and **Cancel** at the right. On a fully filed date every field is read-only and only Delete and **Close** remain.

Both cards share one label column.

**Onboarding** (first run, inside the fixed window) is a two-column grid (320px and up to 620px), centered, with a 64px gap and 32px padding:
1. **Buddy** at 300px, tilted -6deg, right-aligned in the first column.
2. **The bubble:** a white card (5px shadow, 480px minimum height, 36px x 40px padding) with a tail on its left edge pointing at Buddy. It holds one step at a time, then the footer: step dots on the left (14px squares, 4px radius, Lemon for the current step), a **Back** link from the second step on, and one primary button that reads **Next** and finally **Sign in**. It stays one button so keyboard focus stays put; the arrow keys also step.
3. **The four steps:** hello, with today's line and today's real period as a row of day tiles (today Lemon with a 4px border, upcoming dashed, CUTOFF on the last workday); ZeroTier, with a live "NBS is reachable" check (Mint, or Tomato with Check again); read the board (the four verdict pills, then how to file and delete); meet the buttons (the three header icons in 40px tiles with their keycaps, then Ctrl + Enter).

**The download page** is one long column, 1280px max with a `clamp(16px, 3vw, 40px)` gutter. Sections are told apart by their ground, not by rules: lilac canvas (hero, what you get, close), Surface Alt (how fast, install) and white Surface (is it safe). Section padding is `clamp(80px, 11vw, 170px)`. Asymmetric two-column grids pair Buddy's answer with the real app; captures and stickers sit at slight tilts (1-3deg either way), and the four verdict stickers are pinned over the board capture. At 900px every grid becomes one column, the nav links hide and the pins become a plain list; at 520px the type steps down.
- **Captures** are the real app screens (board, File CS/OB, sign-in, Settings) rendered at 2x with sample data, in `docs/assets/*.webp`, each with a `.webp.json` stating its origin. The footer says "Screens show sample data."

## Elevation and Depth

Depth means **hard offset shadows only**: a solid Ink copy of the shape, shifted down-right, with no blur and no spread. It is `box-shadow: Npx Npx 0 var(--ink)` (tokens `--shadow-1` to `--shadow-4`).

| Element | Offset |
|---|---|
| Status pill, rail tile | 2px |
| Buttons, period stepper group | 3px |
| Dialog cards, profile card | 4px |
| Status strip, table card, dialogs, sign-in and onboarding cards, period-grid popover | 5px |
| Keycap | 2px |
| Download page | day tiles 2px; header icon tiles and the small Download 3px; pinned stickers, click chips and the delete note 4px; bubbles, captures, install steps and the Download button 5px |

- **Pressed:** the face translates by the shadow offset and the shadow disappears, in one step with no transition. In the app that is the whole button behavior. On the download page the Download button also lifts on hover (-2px, the shadow grows to 7px, 0.18s with `--out`), but its press is still instant.
- **Disabled:** the shadow is removed.
- **Flat items (no shadow):** upcoming rail tiles, which are dashed and transparent.

## Shapes

- **Radii:** 6px pills, 7px rail tiles, 8px controls (buttons, fields, combos, tone stickers), 12px containers (strip, table, dialog cards).
- **Square popup:** the period-grid popover keeps the square corners of the Qt original.
- **Borders:** 2px Ink at rest, 3px for needs-action stickers and the current period cell, and 4px for today's rail tile.
- **Focus:** a 3px indigo ring with padding reduced by 1px, so nothing shifts.
- **Joined stepper:** `[<][ Jul 1–15, 2026 ⌄ ][>]` is one sticker with one shadow. Only its outer corners are rounded, and the label has no side borders.
- **Icons:** drawn in SVG, never font glyphs or emoji: the chevrons, and the header set in [src/Icon.svelte](src/Icon.svelte) (gear, refresh, plus) on one 24-unit grid with a 2.6 round Ink stroke.
- **Speech-bubble tail:** a square of the bubble's own fill with two 2px Ink edges, rotated 45deg and tucked half under the bubble's border, so bubble and tail read as one cut-out sticker.

## Components

### Buttons
- **Shape:** white fill, 2px Ink border, 8px radius, 5px × 14px padding, 700 weight, 3px hard shadow.
- **Primary** (one per surface): Lemon fill, 800 weight. The label names the action ("File CS/OB", never "OK"); an icon-only primary (the header plus) names it in its accessible name and tooltip.
- **States:** hover is #f3efff (Lemon Hover on the primary). Pressed drops onto the shadow. Focus is a 3px indigo ring. Disabled is Surface Alt with muted text and a muted border, and no shadow.
- **Link:** transparent, 800 weight, underlined Ink ("Back to current period").
- **Keyboard:** F5 to refresh, Ctrl+N to file today, Alt+←/→ to step the period, ↑/↓ and Enter on table rows.
- **Danger:** Tomato fill, 800 weight, only for Delete. It always opens a confirm first.

### Status Pills (signature)
One verdict per day, painted as a sticker centered in the Status cell. It has a 2px shadow and a 2px Ink outline (3px for Needs action).
- **Done** (Mint): OB and CS are both approved.
- **Waiting** (white): filed and awaiting approval.
- **Due today** (Lemon): today, not fully filed.
- **Needs action** (Tomato): a past workday missing OB or CS, a disapproved filing, or a time mismatch.

Weekend and upcoming days get no pill. The tooltip shows the OB and CS statuses and the customer.

### Status Strip
- **Headline sticker:** shows the tone. Tomato means days need action ("2 days need action"), Lemon means only today is left, and Mint means all filed. While loading, it is plain muted text with no sticker.
- **Load error:** the headline goes Tomato ("Couldn't load Jul 1–15"), the detail line gives the recovery, and the raw error goes in a tooltip.
- **Day rail:** one 32×42 tile per workday, 5px apart, each with a 2px shadow.
  - Done is Mint, Waiting is white, and Needs action is Tomato with a 3px outline.
  - Today is white with a 4px outline.
  - Upcoming tiles are dashed and transparent, with no shadow.
  - A **disapproved day is struck through** with a 2.5px round-cap diagonal, not removed.
  - The cutoff tile carries an Ink tag with Lemon "CUTOFF" text.

### Dashboard Table (signature)
- **Frame and header:** a white card with a 2px Ink frame, 12px radius and 5px shadow. The header is Lemon with uppercase 800 labels over a 2px Ink rule.
- **Rows:** every row gets a 2px Ink rule underneath.
- **Selection:** a 3px indigo outline, **never a fill**, so the row wash stays readable.
- **Alignment:** Status and OB hrs are centered. Everything else is left-aligned. Today's date is bold.
- **Filing from a row:** double-click or Enter opens File CS/OB on that date.

### Period Picker
- **Stepper:** the label is 800 weight with a 186px minimum width, so the arrows never move.
- **Year grid popover:** a native `popover` anchored under the label with CSS anchor positioning; white, square, 2px Ink border.
  - Header: the year in 900 weight, with step arrows.
  - Body: two columns of months, with month names in muted 700 weight. Each half-month is a sticker button ("1–15", "16–31").
  - The selected period is Lemon at 900 weight. The current period has a 3px border at 900 weight.
  - The range is last year to next year.

### Fields
- **Style:** white fill, 2px Ink border, 8px radius, 4px × 8px padding, 34px minimum height. Date and times are native `date`/`time` inputs; reasons are textareas with `field-sizing: content` (1 to 12 lines).
- **Focus:** the border turns indigo plus a 1px indigo ring outside it (3px total, nothing moves).
- **Invalid:** a #d0201a border and ring, and a Tomato error sticker below the cards names the field. While the field has focus the indigo ring wins; the red comes back when you leave it. The sticker appears once you leave the field or press Save, never while typing, and the field points to it with `aria-describedby`. Only the field the sticker names is marked invalid, so red never appears without words.
- **Read-only:** Surface Alt fill with muted text and a muted border. This is the "already filed" view, where every field shows what NBS has.
- **Hours:** a derived value shown as bold plain text, not a box, and not a Tab stop.
- **Shift picker:** a single dropdown with the full NBS codes. The user rejected a visible list.
- **Customer picker** (Settings): a single dropdown of NBS's customers once the list loads. A saved name NBS doesn't have shows as "(not in NBS)" and blocks Save. While loading or offline it is the same dropdown, disabled ("Connect to NBS to change the customer."), with a Lemon notice and Try again; the button keeps focus through the retry and hands it to the dropdown when the list arrives.

### Buddy speaks (signature)
Buddy beside a white sticker speech bubble (2px Ink, 12px radius, 5px shadow) whose tail points at him. Buddy is tilted (-4deg to -6deg) and speaks in the first person. When the row gets narrow (the page under 900px), Buddy sits above the bubble and the tail moves to its top edge, pointing up. Used for every onboarding step and in every section of the download page: a giant Buddy (up to 470px) in the hero and the close, a small one (72-104px) in the tour sections.

### Keycap
`kbd` is a tiny sticker: white, 2px Ink, 6px radius, 2px hard shadow, 0 x 6px padding, 12px / 800 (15px on the download page). One key per cap, joined by a plain " + " ("Ctrl + N"). Used wherever a shortcut is named in onboarding and on the page.

### Motion (onboarding and the download page only)
One grammar, used in two places. Nothing on the board, in dialogs or in sign-in animates.
- **The slap** (signature): a sticker arrives lifted (-16px, -20px, scale 1.06) and lands on its shadow with a weighted overshoot: `backOut` in Svelte, `cubic-bezier(.34, 1.56, .64, 1)` (`--land`) on the page. In onboarding, Buddy and the bubble's content slap again on every step (560ms, the content 70ms behind Buddy), and legend pills and button rows follow at 110ms intervals. On the page each sticker slaps once as it scrolls into view (620ms, 110ms stagger).
- **The beat:** today's day row lands on one downbeat. Every tile drops from -30px (-34px on the page) at -8deg with a small delay of its own (0-80ms, from a sine), so the row leans together instead of marching. 700ms in onboarding, 800ms on the page.
- **Buddy alive:** a blink every 5.2s (the eyes squash to 12% height for a moment), and eyes that follow the pointer by a few pixels, easing over 0.35s with `cubic-bezier(.16, 1, .3, 1)` (`--out`). In the app this is the `alive` prop on [src/Buddy.svelte](src/Buddy.svelte), used only by onboarding; the header and sign-in Buddies stay still. On the page every Buddy blinks, and the eyes follow only with a fine pointer.
- **Board drift** (page only): the board capture starts tilted back (rotateX 9deg, scale 0.94) and settles flat as it scrolls past, tied to scroll with `animation-timeline: view()`. Browsers without it show the board flat.
- **SmartScreen rings** (page only): Lemon rings circle More info, then Run anyway, one after the other (1.2s each, starting 0.5s and 1.5s in).
- **Small page affordances:** nav links draw their underline on hover, day tiles rise 3px, header icon tiles press down on hover.

**The Moves-Never-Hides Rule.** In the app a transition animates transform only, never opacity: if an animation stalls, the content is already there to read.

**The Still Rule.** Under `prefers-reduced-motion: reduce` everything arrives in place: onboarding transitions run at 0ms, the page turns off every animation and transition and shows every entrance, and Buddy neither blinks nor looks around.

### Sign-in, onboarding, profile, settings
- **Onboarding** (first run only): Buddy speaks through four steps (see Layout), with the slap and the beat.
- **Sign-in:** the mascot, username, password, and "Remember me on this PC" (on by default; the login goes to Windows Credential Manager). Errors are a Tomato sticker; a lost connection is a Lemon notice with Try again.
- **Settings** (the drawn gear button in the header, or Ctrl+,): a dialog with one "Filing defaults" card: OB reason, CS reason and the NBS customer every OB goes to. Saved on this PC. Save stays disabled until something changes; Reset to app defaults refills the form without saving. File CS/OB shows the customer as a read-only line (a Lemon sticker plus "your pick" when it isn't the app default) with a Change in Settings link that opens Settings on top; Ctrl+, does the same.
- **Profile:** a dialog with a Mint initials sticker, the name in display type, and a card of work fields from NBS (empty ones hidden). Log out also forgets the saved login.
- **Mascot:** a Lemon rounded sticker with an Ink face and a hard Ink shadow; it is also the app icon ([app-icon.svg](app-icon.svg)). It is still by default; `alive` adds the blink and the pointer-following eyes.

### Download page
- **Nav:** the wordmark (Buddy plus "NBSBuddy" at 21px Expanded Black), three 15px / 700 links with a drawn 2px underline on hover, and a small Lemon Download (8px radius, 3px shadow).
- **Download button:** Lemon, 12px radius, 5px shadow, 15px x 24px padding, 21px / 900 at wdth 112, a drawn download arrow and "Download for Windows", with the version and OS line beside it in 15px on-canvas-muted. It links the latest release's `NBSBuddy-setup.exe`.
- **Today's period:** the hero shows today's real period with the app's day tiles (up to 46 x 56); a tile is a toggle that makes Buddy's line describe that day. Without script the line says it in words.
- **Trust claims:** 28px headlines, each with a link to the source file that proves it.
- **Install steps:** three stickers (the first Lemon, the others white) at small tilts, then the SmartScreen drawing beside Buddy's bubble.

### Confirms
Native modal `<dialog>`s on the lilac canvas with a 5px shadow: title in display type, one sentence, then the action button and a white "Go back"/"Keep it" that has focus. Weekend filing and Delete both confirm; Delete lists the pending OB/CS as ticked checkboxes.

## Implementation notes

- Tokens: the `:root` block in [src/app.css](src/app.css). Archivo is self-hosted from `src/assets/fonts` (SIL OFL), never a CDN.
- Ctrl+Enter submits any dialog's form from any field, even a multi-line reason (`Modal.svelte`). Closing a dialog returns focus to whatever opened it.
- Native platform first: `<dialog>` for modals, `popover` + CSS anchor positioning for the period grid, `field-sizing: content` for reasons, `date`/`time` inputs.
- `color-scheme: light` pins the theme; it stays light until a real dark palette is designed.

## Do's and Don'ts

### Do:
- **Do** take every color from the named tokens. The CSS variables in `src/app.css` are the single source.
- **Do** keep Ink text and an Ink outline on every sticker fill.
- **Do** say every tint in words too: the Note column for rows and the detail line for the strip.
- **Do** keep the selected row an indigo outline, never a fill.
- **Do** keep focus as a 3px indigo ring with padding reduced by 1px, so nothing moves.
- **Do** show a new problem state with an existing verdict and wash, and fit it into the row precedence.
- **Do** keep periods visible (stepper and year grid) and the shift picker a single dropdown. The user asked for both.
- **Do** give every animation a `prefers-reduced-motion` path where the content arrives in place.
- **Do** show the product with real app captures with sample data, and say so.

### Don't:
- **Don't** blur a shadow, add a glow, or add a gradient. Shadows are hard Ink offsets only.
- **Don't** animate state changes or add transitions to app controls. Press is one step. Motion belongs to onboarding and the download page, and only as the slap, the beat or Buddy.
- **Don't** use indigo for status, or use Mint, Lemon or Tomato for decoration.
- **Don't** let color be the only signal on a row or a sticker.
- **Don't** introduce a second typeface or use font glyphs or emoji as icons.
- **Don't** follow the OS dark mode until a real dark palette exists.

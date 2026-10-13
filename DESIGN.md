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
- Pressed buttons drop onto their shadow in one step. Nothing fades or animates.

Tokens and shared components live in [src/app.css](src/app.css); each screen is a Svelte file in [src/](src/). The design started as the Sticker Board theme of the earlier Qt app (FilingsValidator) and carries over unchanged.

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
- **Card labels** (dialog "DAY AND SHIFT", "REASONS"): 8pt / 900, uppercase, 108% letter spacing, kerning off. Archivo pulls "Y " together when kerning is on.
- **Pills:** 9pt / 800. **Rail numbers:** 10pt / 800. **CUTOFF tag:** 6.5pt / 900.
- **Strip meta line** ("Cutoff Wed, Jul 15 · 4 workdays left · …"): 800.
- Use tabular numerals for times and hours.

## Layout

The window opens at 1133×778 (minimum 980×620) as a single column with 14–16px margins and 12px vertical spacing; extra width goes to the table's Reasons column.
1. **Top bar** (8px gaps): on the left, the joined period stepper with a 3px shadow, plus a "Back to current period" link that appears only when you've moved away. On the right, Refresh, the lemon **File CS/OB**, then the user chip (initials sticker + first name) that opens the profile.
2. **Status strip card** (5px shadow):
   - Left: the display headline inside a tone sticker, then the detail line ("To file: Jul 7 · Disapproved, refile in NBS: Jul 8 · Late/undertime: …"), then the meta line.
   - Right: the day rail.
3. **Table card** (5px shadow) takes the remaining height. Columns are Date 104, Status 128, OB time 104, Shift 252, Hrs 52 and Note 236 (px), and Reasons stretches. Rows are 31px and single-line, with the full text in tooltips.
4. **Footer:** an on-canvas-muted 600-weight line above a 2px ink rule ("Loaded Jul 1–15 at 9:42 AM.").

The File CS/OB dialog is one column with 16px margins and 12px gaps, at least 480px wide:
1. The title.
2. A subtitle note, which turns into a lemon sticker when it needs attention.
3. A **Day and shift** card (4px shadow): the date, the shift dropdown, and the time row ("07:30 to 17:30 · **10** hours").
4. A **Reasons** card (4px shadow): OB and CS reason fields that auto-grow from 1 to 12 lines.
5. A tomato error sticker.
6. A tomato **Delete OB/CS…** button at the left when the date has a filing still *For Approval*, then the lemon **File CS/OB** and **Cancel** at the right. On a fully filed date every field is read-only and only Delete and **Close** remain.

Both cards share one label column.

## Elevation and Depth

Depth means **hard offset shadows only**: a solid Ink copy of the shape, shifted down-right, with no blur and no spread. It is `box-shadow: Npx Npx 0 var(--ink)` (tokens `--shadow-1` to `--shadow-4`).

| Element | Offset |
|---|---|
| Status pill, rail tile | 2px |
| Buttons, period stepper group | 3px |
| Dialog cards, profile card | 4px |
| Status strip, table card, dialogs, sign-in and onboarding cards, period-grid popover | 5px |

- **Pressed:** the face translates by the shadow offset and the shadow disappears, in one step with no transition.
- **Disabled:** the shadow is removed.
- **Flat items (no shadow):** upcoming rail tiles, which are dashed and transparent.

## Shapes

- **Radii:** 6px pills, 7px rail tiles, 8px controls (buttons, fields, combos, tone stickers), 12px containers (strip, table, dialog cards).
- **Square popup:** the period-grid popover keeps the square corners of the Qt original.
- **Borders:** 2px Ink at rest, 3px for needs-action stickers and the current period cell, and 4px for today's rail tile.
- **Focus:** a 3px indigo ring with padding reduced by 1px, so nothing shifts.
- **Joined stepper:** `[<][ Jul 1–15, 2026 ⌄ ][>]` is one sticker with one shadow. Only its outer corners are rounded, and the label has no side borders.
- **Icons:** drawn chevrons, never font glyphs or emoji.

## Components

### Buttons
- **Shape:** white fill, 2px Ink border, 8px radius, 5px × 14px padding, 700 weight, 3px hard shadow.
- **Primary** (one per surface): Lemon fill, 800 weight. The label names the action ("File CS/OB", never "OK").
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
- **Alignment:** Status and Hrs are centered. Everything else is left-aligned. Today's date is bold.
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
- **Invalid:** a #d0201a border and ring, and a Tomato error sticker below the cards names the field.
- **Read-only:** Surface Alt fill with muted text and a muted border. This is the "already filed" view, where every field shows what NBS has.
- **Hours:** a derived value shown as bold plain text, not a box, and not a Tab stop.
- **Shift picker:** a single dropdown with the full NBS codes. The user rejected a visible list.

### Sign-in, onboarding, profile
- **Onboarding** (first run only): one white card, three steps with lemon step dots: what NBSBuddy does, a live "NBS is reachable" check (Mint, or Tomato with Check again), and the sticker legend.
- **Sign-in:** the mascot, username, password, and "Remember me on this PC" (on by default; the login goes to Windows Credential Manager). Errors are a Tomato sticker; a lost connection is a Lemon notice with Try again.
- **Profile:** a dialog with a Mint initials sticker, the name in display type, and a card of work fields from NBS (empty ones hidden). Log out also forgets the saved login.
- **Mascot:** a Lemon rounded sticker with an Ink face and a hard Ink shadow; it is also the app icon ([app-icon.svg](app-icon.svg)).

### Confirms
Native modal `<dialog>`s on the lilac canvas with a 5px shadow: title in display type, one sentence, then the action button and a white "Go back"/"Keep it" that has focus. Weekend filing and Delete both confirm; Delete lists the pending OB/CS as ticked checkboxes.

## Implementation notes

- Tokens: the `:root` block in [src/app.css](src/app.css). Archivo is self-hosted from `src/assets/fonts` (SIL OFL), never a CDN.
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

### Don't:
- **Don't** blur a shadow, add a glow, or add a gradient. Shadows are hard Ink offsets only.
- **Don't** fade or animate state changes. Press is one step.
- **Don't** use indigo for status, or use Mint, Lemon or Tomato for decoration.
- **Don't** let color be the only signal on a row or a sticker.
- **Don't** introduce a second typeface or use font glyphs or emoji as icons.
- **Don't** follow the OS dark mode until a real dark palette exists.

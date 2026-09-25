---
name: Mailboar
description: Tabler inspection bench for catching and reading test mail.
colors:
  satchel-orange: "#c2560c"
  satchel-ember: "#d55e11"
  satchel-night: "#f08a3e"
  satchel-on: "#f9fafb"
  paper-canvas: "#f9fafb"
  white-surface: "#ffffff"
  slate-ink: "#374151"
  muted-ink: "#6b7280"
  hairline: "#e5e7eb"
  night-paper: "#111827"
  slate-surface: "#1f2937"
  night-hairline: "#374151"
  night-muted: "#9ca3af"
  danger: "#c92a2a"
  danger-night: "#f06a6a"
typography:
  display:
    fontFamily: "Inter Var, Inter, -apple-system, BlinkMacSystemFont, San Francisco, Segoe UI, Roboto, Helvetica Neue, sans-serif"
    fontSize: "1.5rem"
    fontWeight: 600
    lineHeight: 1.33
    letterSpacing: "normal"
  headline:
    fontFamily: "Inter Var, Inter, -apple-system, BlinkMacSystemFont, San Francisco, Segoe UI, Roboto, Helvetica Neue, sans-serif"
    fontSize: "1.25rem"
    fontWeight: 600
    lineHeight: 1.4
    letterSpacing: "normal"
  title:
    fontFamily: "Inter Var, Inter, -apple-system, BlinkMacSystemFont, San Francisco, Segoe UI, Roboto, Helvetica Neue, sans-serif"
    fontSize: "1.125rem"
    fontWeight: 600
    lineHeight: 1.5
    letterSpacing: "normal"
  body:
    fontFamily: "Inter Var, Inter, -apple-system, BlinkMacSystemFont, San Francisco, Segoe UI, Roboto, Helvetica Neue, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.4285714286
    letterSpacing: "0"
  label:
    fontFamily: "Inter Var, Inter, -apple-system, BlinkMacSystemFont, San Francisco, Segoe UI, Roboto, Helvetica Neue, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.4285714286
    letterSpacing: "normal"
  caption:
    fontFamily: "Inter Var, Inter, -apple-system, BlinkMacSystemFont, San Francisco, Segoe UI, Roboto, Helvetica Neue, sans-serif"
    fontSize: "0.75rem"
    fontWeight: 400
    lineHeight: 1.4
    letterSpacing: "normal"
  mono:
    fontFamily: "Monaco, Consolas, Liberation Mono, Courier New, monospace"
    fontSize: "0.75rem"
    fontWeight: 400
    lineHeight: 1.5
    letterSpacing: "normal"
rounded:
  sm: "4px"
  md: "6px"
  lg: "8px"
spacing:
  sm: "8px"
  md: "16px"
  lg: "24px"
components:
  button-default:
    backgroundColor: "{colors.white-surface}"
    textColor: "{colors.slate-ink}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: "9px 16px"
    height: "40px"
  button-primary:
    backgroundColor: "{colors.satchel-orange}"
    textColor: "{colors.satchel-on}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: "9px 16px"
    height: "40px"
  button-ghost-danger:
    backgroundColor: "transparent"
    textColor: "{colors.danger}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: "9px 16px"
    height: "40px"
  input:
    backgroundColor: "{colors.white-surface}"
    textColor: "{colors.slate-ink}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: "8px 12px"
    height: "36px"
  card:
    backgroundColor: "{colors.white-surface}"
    textColor: "{colors.slate-ink}"
    rounded: "{rounded.lg}"
    padding: "0"
  list-item:
    backgroundColor: "{colors.paper-canvas}"
    textColor: "{colors.slate-ink}"
    typography: "{typography.body}"
    rounded: "0"
    padding: "14px 16px"
  list-item-active:
    backgroundColor: "{colors.white-surface}"
    textColor: "{colors.slate-ink}"
    padding: "14px 16px"
  segmented-tab:
    backgroundColor: "transparent"
    textColor: "{colors.muted-ink}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: "0 10px"
    height: "36px"
  segmented-tab-active:
    backgroundColor: "{colors.white-surface}"
    textColor: "{colors.slate-ink}"
    rounded: "{rounded.md}"
---

# Design System: Mailboar

## Overview

**Creative North Star: "The Inspection Bench"**

Mailboar is a tester's bench, not a product site. The job is to scan a caught-mail list, open a payload, and read HTML, plain, or source without the chrome competing with the letter. Tabler (on Bootstrap 5) is the chassis: Inter, 14px density, gray paper, 6–8px corners, faint admin shadows.

Personality is a little more animal/tool than a generic dashboard, and it now comes from the product's own colour rather than a borrowed one. The accent is lifted straight out of the boar mark, so the app and its mascot share a pigment. That is the whole brand move: no display face, no marketing layout, no illustration library.

Light and dark are equal operating modes. Theme follows `prefers-color-scheme` on first load, is remembered in `localStorage` after that, and flips from a moon/sun toggle. New UI must ship both, and both must clear 4.5:1 on text.

**Key Characteristics:**
- List/detail inspection layout, not a marketing funnel
- Tabler admin density (14px Inter) with the boar mark as the only mascot
- Satchel Orange for committed actions and for "this is the row you are reading"
- Paper canvas vs raised surfaces; shadows are 1–4px whispers
- The email HTML is an iframe artifact — framed like a letter, never restyled

## Colors

A cool gray lab bench with one warm pigment borrowed from the animal. The accent is a signal, not a wash.

### Primary
- **Satchel Orange** (`#c2560c`): filled primary controls. Deepened from the mark so `satchel-on` text clears 4.5:1 on the fill.
- **Satchel Ember** (`#d55e11`): the mark's own orange, exposed as `--mb-ember`. Used for hairlines, selection, caret, and focus rings — places where it is a stroke rather than a text background.
- **Satchel Night** (`#f08a3e`): the dark-mode value of both. Filled controls in dark take near-black ink (`#1b1207`).
- **Satchel On** (`#f9fafb`): text and icons sitting on a Satchel Orange fill.

### Neutral
- **Paper Canvas**: page body and resting list rows in light — the bench top.
- **White Surface**: cards, default buttons, the selected list row, the active segment.
- **Slate Ink**: body text.
- **Muted Ink**: addresses, stamps, secondary copy. Light only.
- **Hairline**: borders and default-button stroke.
- **Night Paper** (`#111827`): dark page body and dark resting list rows.
- **Slate Surface** (`#1f2937`): dark cards, navbar, the selected row.
- **Night Hairline** (`#374151`) / **Night Muted** (`#9ca3af`): the dark counterparts. Tabler ships neither muted value dark enough, so `--tblr-muted` is overridden in the dark block.

### Semantic
- **Danger** (`#c92a2a`): destructive meaning only. Tabler's stock red only reached 4.39:1 against Paper Canvas, which is exactly where the delete controls sit.
- **Danger Night** (`#f06a6a`): the dark value, set on `--tblr-danger` so the ghost delete button and its hover fill both shift.

### Named Rules
**The Satchel Orange Rule.** The accent marks two things and nothing else: the committed action of a view (Forward), and the message you are currently reading. It never paints nav, page chrome, or more than a thin slice of the screen.

**The Dual-Shift Rule.** Every surface has a paper reading and a night-paper reading, and the relationship between them must survive the flip. Tabler's tertiary surface collapses onto its card surface in dark, so the list rows recess to Night Paper there instead — the selected row still lifts either way.

**The 4.5 Floor Rule.** Every text colour is measured against the surface it actually lands on, in both themes, before it ships. Four of the accents in this system were re-cut for that reason.

## Typography

**Display Font:** Inter Var / Inter (system UI stack)
**Body Font:** same Inter stack
**Mono Font:** Monaco, Consolas, Liberation Mono, Courier New — raw source only

**Character:** One family, small and even. Weight and size do the hierarchy. No display serif, no marketing clamp type.

### Hierarchy
- **Display** (600, 1.5rem / 24px): the page title ("3 messages"). The one place the app raises its voice.
- **Headline** (600, 1.25rem / 20px): the message subject in the detail header.
- **Title** (600, 1.125rem): empty-state title, modal title.
- **Body** (400, 0.875rem / 14px): default UI, addresses, route labels.
- **Caption** (400, 0.75rem): list-row route line, detail stamp line, attachment table.
- **Mono** (400, 0.75rem): the source pane.

Anything numeric that a reader compares down a column — sizes, timestamps — carries `.mb-numeric` (`font-variant-numeric: tabular-nums`).

### Named Rules
**The One Voice Type Rule.** Inter is the UI. Source is the only mono exception. Do not introduce a second face.

**The Subject-First Rule.** In any list of caught mail, the subject is the headline and the addresses are the caption. A tester scans for what was sent, not who sent it.

## Layout

Bootstrap/Tabler grid inside `container-xxl`, with Mailboar's one override: extra-wide `xxl` max width **1520px**. Horizontal rhythm is Tabler's 8px spacer.

App shell: 56px top navbar (boar mark + wordmark + version chip + icon-only GitHub and theme controls), then a page header with the message count and "Delete all". When mail exists: **4 / 8** columns (`col-sm-4` list, `col-sm-8` detail).

The list is an instrument well. From `sm` up it is `position: sticky` at `--mb-stick-top` (1rem) with a `calc(100vh - 2rem)` scroll body, so the index stays put while a long message scrolls. Below `sm` the two columns stack, the well drops to `--mb-list-height: 40vh`, and the sticky behaviour is switched off — a pinned full-height list above the message would bury the thing the user opened.

## Elevation & Depth

Tonal layering first. Shadows are whispers, structural not theatrical.

### Shadow Vocabulary
- **Button rest** (`0 1px 1px 0 rgba(31, 41, 55, .06)`): default chips.
- **Card / ambient** (`0 0 4px 0 rgba(31, 41, 55, .04)`): cards sit barely off the paper.
- **Selection hairline** (`inset 1px 0 0 0 var(--mb-ember)`): the selected list row. A 1px inset, not a rail.

### Named Rules
**The Whisper Shadow Rule.** Surfaces are separated by tone and 1px hairlines. If a shadow is heavier than ~4px blur, it is out of system.

## Shapes

Soft-admin geometry, not pills-as-brand.

- **Default radius 6px**: buttons, inputs, segment tabs.
- **Card radius 8px**: the list card, the message card, the letter frame.
- **Tight 4px**: nested chips only.
- Hairline borders, 1px. Icons: Tabler outline, 24×24, stroke 2, currentColor.

## Components

### Buttons
- **Default:** white (slate in dark) fill, Hairline border, whisper shadow.
- **Primary:** Satchel Orange fill. The committed action of a view — Forward in the message header, Forward in the modal.
- **Ghost danger:** transparent fill, Danger text, icon + label. Delete and Delete all. Destructive work reads as a warning, not as a filled red button competing with the primary.
- **Icon-only:** `btn-icon btn-ghost-secondary` for navbar utilities. Always carries a `title`, since there is no visible label.
- **Focus:** a 2px Satchel Ember `:focus-visible` outline at 2px offset, app-wide.

### Cards / Containers
One card per job. The message detail is a single card — header block, segmented format bar, body — not a stack of nested cards.

- **Corner Style:** 8px
- **Background:** White Surface (light) / Slate Surface (dark)
- **Shadow Strategy:** whisper ambient

### Message list item (signature)
A `router-link` row, so a message is a real URL a tester can share and the browser handles active state. Layout is an avatar (sender initial) beside a two-line body: the subject on its own full-width line, then a caption line with `from → to` on the left and a relative stamp on the right, both truncating with ellipsis.

Resting rows sit on Paper Canvas (light) or Night Paper (dark). The selected row lifts to the card surface, gains the ember inset hairline, and its avatar fills with the accent. Rows animate in with a 0.3s `message-arrives` fade-and-settle, because arrival over SSE is the one genuinely live moment in the app — and the keyframe deliberately has no fill-mode, so a row whose animation never runs still renders visible.

### Format switcher (signature)
`nav-segmented` in its own bar between the header and the body: HTML / Source / Plain text, with an attachment count badge when the message carries parts. Proper labels, not lowercase MIME shorthand.

### Message HTML frame (signature)
The letter renders in an iframe on its own white sheet — a bordered, 8px-radius card inside the message body — so caught HTML reads as a document laid on the bench rather than as part of the app. Height is measured from `documentElement.scrollHeight`, which accounts for the body margin most mail carries. Never restyle the letter itself. Links inside open in a new tab.

### Message route (detail header)
A `<dl>` of From / To beside the sender avatar, then a single caption line reading `time · size · type`. One line of metadata, not a striped definition table.

### Source and plain panes
Both are `<pre>`. Plain text wraps at a 70ch measure so a long paragraph stays readable. Source is mono on a recessed surface with a custom thin scrollbar (`.mb-scroll`), and reads as a code surface rather than body copy.

### Empty state (signature)
Tabler `.empty` carrying the boar mark at its native 120×90, a 1.125rem title, and one muted line of instruction. Copy states what the user should do and what will happen ("Point your app's SMTP at this server. New messages appear here on their own."), because the list fills itself over SSE and telling someone to refresh would be a lie.

## Do's and Don'ts

### Do:
- **Do** keep Inter at 14px and Tabler spacing (8 / 16 / 24) unless a measured exception already exists (1520px `xxl`, the list well).
- **Do** put Satchel Orange on the committed action and on the selected row.
- **Do** measure new text colours against their real surface in both themes.
- **Do** bind surfaces to Tabler `--tblr-*` variables and override them per theme, rather than writing one-off hexes.
- **Do** frame the caught HTML as a letter and leave its contents alone.
- **Do** give every icon-only control a `title`.

### Don't:
- **Don't** add hero type, marketing sections, or a second typeface.
- **Don't** wash backgrounds or nav in the accent.
- **Don't** introduce shadows heavier than the whisper set, or radii outside 4 / 6 / 8.
- **Don't** nest a card inside a card to separate two parts of one message.
- **Don't** restyle the email document to match the app.
- **Don't** write light-only hexes into a component; the theme block is the place for a dark value.

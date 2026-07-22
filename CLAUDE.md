# platinum-ui

Mac OS 8/9 "Platinum" design system for Leptos 0.8 (CSR, nightly): Rust
components in `src/` + the skin in `platinum.css`.

**This is an entire design system, not a widget library — all or nothing.**
It owns the page it's mounted on: a global reset (Tailwind-style preflight),
element-level rules, tokens. Never design a component to "fit into" a
non-Platinum host page, and never add opt-outs that let parts of a page
escape the system.

**One consumption path** (there used to be a Tailwind-import mode; it was
removed — no Tailwind/Node anywhere): `build.rs` generates, at compile time
with encre-css (pure-Rust Tailwind v4 generator, custom colors registered
there), BOTH the preflight and the utility CSS; `PLATINUM_CSS` = preflight +
platinum.css + utilities (that order: reset loses to skin, utilities win
over both), and the app mounts `<PlatinumStyles/>` once to inject it. The
preflight carries the `--en-*` variable defaults that composed utilities
(translate-*, ring-*, …) resolve against — components are written against
that flat baseline, never against browser UA defaults (the historical
"preflight gap" bug family came from one consumer missing it). Apps generate
utilities for their OWN markup the same way, injected as a second `<style>`
after `<PlatinumStyles/>` — their build.rs is one call into the shared
`build-helper/` crate (`platinum_ui_build::generate_utilities("src", …)`),
which owns the token registry and font stacks; add new platinum.css tokens
to `TOKENS` there and every app picks them up. CONSTRAINT: every utility
class a component
   emits must be encre-css-parseable — nested-bracket arbitrary variants like
   `[&_x:not([a='b'])]:…` are not; write those as plain rules in platinum.css
   (see the Button icon-size rule). When adding a utility class to a
   component, add it to `tests/utility_coverage.rs` — `cargo test` (also run
   by CI) asserts every listed class produced a rule in PLATINUM_CSS. Also:
   never glue a `format!` placeholder directly onto a class string
   (`"…foo{}"`) — the scanner sees `foo{}` and generates nothing; join with
   `crate::classes()` instead.

## The bevel system — read this before touching platinum.css

Everything is built from two dual constructions plus composites:

- **Raised** (platform): real 1px `--pl-outline` (#555) border + mitered raised
  bevel (`--pl-bc-raised`) on `::after` at inset 0. Members: `[data-name="Button"]`
  (guarded with `:not(.pl-plaque)`), `.pl-platform`, scroll/select arrows.
  Pressed (`:active` / `[aria-current="page"]`) darkens the face and inverts the
  `::after` border-color to `--pl-bc-well`.
- **Recessed** (well): mitered well bevel as the real border (`--pl-bw` wide,
  `--pl-bc-well`) + a 1px `--pl-outline-inset` ring drawn inside. Members:
  `.pl-well`, `.pl-plaque`, `.pl-palette-plaque`, `.pl-select-field`, text
  inputs/textareas (white fill instead of face).
- **Composites are group memberships, not new CSS.** A plaque (platform seated
  in a well, sharing the ring as a 1px seam) is: well group + the shared raised
  `::after` list + `.pl-plaque::after { inset: 1px }`. A plaque *button* is the
  same via `ButtonVariant::Plaque` emitting `pl-plaque` — the raised group's
  `:not(.pl-plaque)` guard makes the well dressing win structurally, NOT by
  file order. A plaque *palette* is well group + `padding: 1px` (the #555 strip
  background shows through as the seam).

Other pieces: bevels are mitered because they're **borders** (4-value
`border-color` must be its own declaration — the shorthand takes one color).
Parts that can't take `::after` (the webkit scrollbar) use the 1px
`--pl-raised-thin`/`--pl-well-thin` box-shadow bevels; checkboxes/radios are
full well-group members (border bevel + ring), with `::after` free for the
check/dot glyph since wells need no pseudo-element. The scroll thumb
recolors the shared `::after` bevel by overriding `--pl-bc-*` (custom props
inherit into pseudo-elements). `--pl-bw` on any container resizes every bevel
inside it. `.pl-seam-{top,right,bottom,left}` drop one outline edge where two
raised surfaces abut, so outlines never double to 2px.

Known limits (documented in the CSS): plaque buttons don't go inside a
`.pl-palette` (the strip zeroes cell borders at higher specificity) — put
`.pl-palette-plaque` on the strip instead. Disabled buttons dim only the label,
icon children, and the `::after` platform bevel; the outline / well bevel /
ring stay full-strength (so no `disabled:opacity-*` utilities on the whole box).

## Hard rules

- **Never route `pl-*` classes through tw_merge** — it parses any `pl-*` as
  Tailwind padding-left (no value validation) and deletes it against `p-*`/`px-*`
  utilities. This bug is why `Button` is hand-written and why the crate has no
  `tw_merge`/`leptos_ui` dependency. Join classes with the crate-private
  `classes()` helper in `lib.rs` (plain concatenation) and write base class
  lists that compose without utility conflicts (e.g. DialogContent uses
  `w-[calc(100%-2rem)]` so caller `max-w-*` composes as `min()`).
- **No rounded corners, shadows, or transitions in component class strings** —
  the skin `!important`-overrides them, so they'd be dead on arrival. Component
  Rust files carry only layout, typography, sizing, and interaction utilities;
  the CSS owns all visuals, keyed off `data-name="…"` or `pl-*` classes.
- **Ephemeral per-control UI state lives on the DOM node, not in signals.**
  `DebounceButton` is the canonical example: pending label + `disabled` are
  written imperatively to the `<button>` element because an action that
  refetches the list the button lives in rebuilds the surrounding view — an
  instance-owned signal gets disposed mid-flight (stranding attributes, or
  panicking and poisoning the whole wasm runtime when a stale listener fires).
  Where signals must be read from possibly-outliving closures, use
  `try_get().unwrap_or_default()`.
- **Fonts: Geneva and Monaco are Apple-licensed — never bundle them.** The
  crate ships Arimo/Cousine (Apache-2.0, `fonts/` with their LICENSE files) as
  metric-compatible stand-ins; the stacks put the Apple faces first so Macs get
  the real thing. Only ever redistribute the Arimo/Cousine woff2s.

## Component notes

- `ui_button.rs`: `ButtonVariant` means *construction* — `Default` (raised
  platform) or `Plaque`. Sizes Sm/Default/Lg. No href support (nav uses raw
  `<A attr:data-name="Button">`).
- `ui_dialog.rs`: rust-ui port; open/close runs on a per-instance `<script>`
  (data-attribute machinery), not Leptos signals. Render dialogs OUTSIDE any
  `Transition`/`Suspend` content that refetches, or the rebuild snaps them shut.
  The layout wrappers (DialogBody etc.) come from the local `wrapper!` macro.
- `platinum_scroll.rs` / `platinum_select.rs`: hand-drawn scrollbar and popup
  menu (CSS scrollbar buttons don't render on macOS Chrome; native `<select>`
  was dropped entirely). Their arrow buttons are members of the raised group,
  seated with `.pl-seam-*` + 1px margins. `ScrollWell` (same file) is a
  fitted well with `PlatinumScroll` in welled mode: bar always in layout
  (a fitting page shows a full-track thumb), `.pl-scrollbar-welled` carries
  the divider, arrow seams face the well ring. Caller sets the size via
  `class`.

## The showcase

The living style guide is the `showcase/` workspace member (`PlatinumStyles`
+ its own build.rs-generated utilities for showcase-only classes), served by
`showcase-server/` (axum static + SPA fallback,
`SHOWCASE_ADDR`, default 127.0.0.1:3050). Build and run from the repo root:
`(cd showcase && trunk build)` then `cargo run -p platinum-showcase-server`.
Navigation is the library's own `Sidebar` (`platinum_sidebar.rs`) — a vertical
palette dual: cells drop their outline, the strip's #555 background is the
divider/frame, active entry takes `aria-current="page"` → pressed.

## Verifying changes

For the showcase: `trunk build` in `showcase/` (no watch), then eyeball at
:3050 plus computed-style probes after CSS work. When consumed by chdkpano,
also run `trunk build` in `chdkpano/client` — its dev server serves `dist/`
statically. Hashed bundle filenames identify stale browser console entries:
filter console reads by the new hash before trusting errors, and cache-bust
reloads with `?v=N`.

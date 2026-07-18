# platinum-ui

A Mac OS 8/9 **"Platinum"**-styled component library for [Leptos](https://leptos.dev)
(0.8, CSR, nightly): beveled buttons, modal dialogs, collapsibles, a hand-drawn
popup menu, a debounced action button, and classic Mac scrollbars — including a
`ScrollWell`, a recessed panel with a permanently visible scrollbar.

The crate is two halves that ship together:

- **Rust components** (`src/`) that emit Tailwind utility classes and
  `data-name`/`pl-*` hooks.
- **`platinum.css`** — the skin. All visuals (faces, mitered bevels, outlines,
  pressed states) live here, keyed off those hooks. The components carry only
  layout/typography/sizing utilities.

Bundled fonts: [Arimo and Cousine](https://fonts.google.com/specimen/Arimo)
(Apache-2.0, license files included) as metric-compatible stand-ins for
Geneva/Monaco — Macs get the real Apple faces first in the font stacks, every
other OS gets the bundled ones.

## Installing — no Tailwind needed

The crate compiles its own utility CSS at build time (via
[encre-css](https://crates.io/crates/encre-css), a pure-Rust Tailwind v4
generator, in `build.rs`) — so the default setup is just a dependency and one
component:

```toml
[dependencies]
platinum-ui = { git = "https://github.com/joseph-x-li/platinum-ui" }
```

```rust
use platinum_ui::{PlatinumStyles, Button, ButtonVariant};

view! {
    <PlatinumStyles/>   // injects the whole design system as a <style>
    <Button variant=ButtonVariant::Plaque>"Hello"</Button>
}
```

Optionally serve the woff2 files from `fonts/` at your web root so every OS
renders the bundled faces (without them you get each platform's sans/mono
fallbacks; Macs get Geneva/Monaco either way). With [Trunk](https://trunkrs.dev):

```html
<link data-trunk rel="copy-file" href="platinum-ui/fonts/arimo-latin-400-normal.woff2"/>
<link data-trunk rel="copy-file" href="platinum-ui/fonts/arimo-latin-700-normal.woff2"/>
<link data-trunk rel="copy-file" href="platinum-ui/fonts/arimo-latin-400-italic.woff2"/>
<link data-trunk rel="copy-file" href="platinum-ui/fonts/cousine-latin-400-normal.woff2"/>
<link data-trunk rel="copy-file" href="platinum-ui/fonts/cousine-latin-700-normal.woff2"/>
```

(The `platinum-ui/…` paths above assume a checkout — see below. With a plain
git dependency, copy the five woff2 files out of this repo instead.)

## Alternative: your app already runs Tailwind v4

If your build compiles Tailwind anyway, skip `<PlatinumStyles/>` and let your
Tailwind own one deduplicated stylesheet and one theme for app + library
alike. Check the crate out at a stable path (git submodule or sibling clone —
Tailwind must scan its sources, and the CSS/fonts are plain files):

```sh
git submodule add https://github.com/joseph-x-li/platinum-ui
```

```toml
[dependencies]
platinum-ui = { path = "./platinum-ui" }
```

```css
@import "tailwindcss";
@import "./platinum-ui/platinum.css";

@source "./src";
@source "./platinum-ui/src";
```

This is how [chdkpano](https://github.com/joseph-x-li/chdkpano), the app this
library grew out of, consumes it.

## Components

| Component | What it is |
|---|---|
| `Button` | Beveled Platinum button. `ButtonVariant::Default` (raised platform) or `::Plaque` (platform seated in a well); sizes `Sm`/`Default`/`Lg`. |
| `DebounceButton` | Button that disables itself and shows a pending label while its async action runs (held ≥ 200 ms so fast actions don't flicker). |
| `Dialog` family | Modal window with the classic hard-edged drop shadow: `Dialog`, `DialogTrigger`, `DialogContent`, `DialogHeader`/`Title`/`Description`/`Body`/`Footer`, `DialogClose`. |
| `PlatinumSelect` | Hand-drawn popup menu replacing `<select>`: recessed value well + raised arrow, white overlay menu with ✓ — `position: fixed`, so it never clips inside scroll containers. |
| `PlatinumScroll` | Custom scrollbar with real arrow buttons and a draggable purple thumb (CSS scrollbar buttons don't render on macOS Chrome). |
| `ScrollWell` | A well whose right side is a permanently visible scrollbar and whose left side scrolls — the bar keeps its place even when content fits. Size it from the call site via `class` (a Tailwind utility like `h-64`, or any CSS class of your own). |
| `Collapsible` family | Disclosure widget: `Collapsible`, `CollapsibleTrigger`, `CollapsibleContent`. |

CSS-only primitives (put the class on any element): `pl-platform` (raised
panel), `pl-well` (recessed panel), `pl-plaque` (platform seated in a well,
one element), `pl-well-fitted` (1 px padding seating content inside a well's
outline ring), `pl-palette` / `pl-palette-icons` / `pl-palette-plaque` (tool
palettes), `pl-tabs` / `pl-tab` / `pl-tab-active` (folder tabs),
`pl-seam-top/right/bottom/left` (drop one outline edge where two raised
surfaces abut), `pl-screen` (recessed content screen). Set `--pl-bw` on a
container to resize every bevel inside it.

## License

MIT (see `LICENSE`). The bundled Arimo/Cousine fonts are Apache-2.0 —
see `fonts/LICENSE-arimo.txt` and `fonts/LICENSE-cousine.txt`.

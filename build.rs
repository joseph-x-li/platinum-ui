//! Generates the utility CSS for every Tailwind-compatible class the
//! components emit, at compile time, with no Node toolchain: encre-css (a
//! Rust reimplementation of the Tailwind v4 generator) scans this crate's own
//! sources the same way a consumer's Tailwind `@source` line would have to.
//! lib.rs concatenates the result with platinum.css into `PLATINUM_CSS`, so a
//! consumer that isn't running Tailwind at all can just mount
//! `<PlatinumStyles/>`.
//!
//! CONSTRAINT this creates: every utility class emitted by a component must
//! be parseable by encre-css. Its coverage of Tailwind v4 is near-complete
//! (verified against this crate's whole inventory), but e.g. arbitrary
//! variants with nested brackets + quotes ([&_svg:not([class*='size-'])]:…)
//! are not — express such selectors as plain rules in platinum.css instead.

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=platinum.css");

    let mut sources = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir("src")
        .expect("read src/")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "rs"))
        .collect();
    entries.sort(); // deterministic output ordering
    for path in &entries {
        sources.push(std::fs::read_to_string(path).expect("read source file"));
    }

    // The preflight (Tailwind-v4-style global reset) is generated as its OWN
    // file, not mixed into the utilities: lib.rs must order it preflight →
    // platinum.css → utilities, so the reset loses to the skin's element
    // rules and the utilities win over both. It also carries the --en-*
    // variable defaults that composed utilities (translate-*, ring-*, …)
    // resolve against — without it they parse but compute to nothing.
    // Standalone mode ships it because platinum-ui is an entire design
    // system that owns the page (see lib.rs); Tailwind-mode consumers never
    // see this file — their own Tailwind build provides the preflight.
    let mut preflight_config = encre_css::Config::default();
    preflight_config.preflight = encre_css::Preflight::new_full()
        .font_family_sans(r#""Geneva", "Arimo", "Helvetica Neue", Helvetica, Arial, sans-serif"#);
    let preflight = encre_css::generate([], &preflight_config);
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    std::fs::write(
        Path::new(&out_dir).join("platinum-preflight.css"),
        preflight,
    )
    .expect("write preflight css");

    let mut config = encre_css::Config::default();
    // No preflight in the utilities output — it's the separate file above.
    config.preflight = encre_css::Preflight::None;
    // The palette tokens components reference (text-muted-foreground etc.).
    // Pointed straight at the :root variables platinum.css defines — no
    // Tailwind-style --color-* indirection needed.
    config.theme.colors.add("foreground", "var(--foreground)");
    config.theme.colors.add("muted-foreground", "var(--muted-foreground)");

    let css = encre_css::generate(sources.iter().map(String::as_str), &config);

    std::fs::write(Path::new(&out_dir).join("platinum-utilities.css"), css)
        .expect("write generated css");
}

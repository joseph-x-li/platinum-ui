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

    let mut config = encre_css::Config::default();
    // platinum.css is the reset/base layer of this design system; encre's
    // preflight would fight it (and a consumer's own).
    config.preflight = encre_css::Preflight::None;
    // The palette tokens components reference (text-muted-foreground etc.).
    // Pointed straight at the :root variables platinum.css defines — no
    // Tailwind-style --color-* indirection needed.
    config.theme.colors.add("foreground", "var(--foreground)");
    config.theme.colors.add("muted-foreground", "var(--muted-foreground)");

    let css = encre_css::generate(sources.iter().map(String::as_str), &config);

    let out = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR"))
        .join("platinum-utilities.css");
    std::fs::write(out, css).expect("write generated css");
}

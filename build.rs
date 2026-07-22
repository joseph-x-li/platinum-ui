//! Generates the two build-time pieces of `PLATINUM_CSS` (see lib.rs):
//! the preflight and the utility CSS for every Tailwind-compatible class the
//! components emit — compiled by encre-css via the shared config in
//! `build-helper/` (the same crate app build scripts use), so no Node
//! toolchain is involved anywhere.
//!
//! CONSTRAINT this creates: every utility class emitted by a component must
//! be parseable by encre-css. Its coverage of Tailwind v4 is near-complete
//! (verified against this crate's whole inventory), but e.g. arbitrary
//! variants with nested brackets + quotes ([&_svg:not([class*='size-'])]:…)
//! are not — express such selectors as plain rules in platinum.css instead.

fn main() {
    // lib.rs include_str!s platinum.css; listed here so build.rs reruns too.
    println!("cargo:rerun-if-changed=platinum.css");
    platinum_ui_build::generate_preflight("platinum-preflight.css");
    platinum_ui_build::generate_utilities("src", "platinum-utilities.css");
}

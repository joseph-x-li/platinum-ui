//! Build-script helper for apps on the platinum-ui design system.
//!
//! platinum-ui's single consumption path has every app compile utility CSS
//! for its OWN markup with encre-css in its build.rs — the library's
//! `PLATINUM_CSS` (via `<PlatinumStyles/>`) only covers classes its
//! components emit. This crate owns the shared configuration — the
//! design-token registry and font stacks — so app build scripts stay tiny
//! and a token added to platinum.css propagates to every app without
//! touching their build scripts. The complete app-side setup:
//!
//! ```toml
//! [build-dependencies]
//! platinum-ui-build = { path = "…/platinum-ui/build-helper" }
//! ```
//!
//! ```no_run
//! // build.rs
//! fn main() {
//!     platinum_ui_build::generate_utilities("src", "app-utilities.css");
//! }
//! ```
//!
//! then inject the output after `<PlatinumStyles/>`:
//!
//! ```ignore
//! const APP_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/app-utilities.css"));
//! view! { <PlatinumStyles/> <style>{APP_CSS}</style> <App/> }
//! ```

pub use encre_css;

/// Every design token platinum.css defines on `:root`, registered as encre
/// colors so token utilities (`bg-success`, `text-muted-foreground`,
/// `border-border`, …) resolve to the CSS variables. KEEP IN SYNC with
/// platinum.css's token layer.
pub const TOKENS: &[&str] = &[
    "background",
    "foreground",
    "card",
    "card-foreground",
    "popover",
    "popover-foreground",
    "primary",
    "primary-foreground",
    "secondary",
    "secondary-foreground",
    "muted",
    "muted-foreground",
    "accent",
    "accent-foreground",
    "destructive",
    "destructive-foreground",
    "warning",
    "warning-foreground",
    "success",
    "success-foreground",
    "border",
    "input",
    "ring",
];

/// The Platinum UI face: Geneva (Apple machines get the real thing) with the
/// bundled, metric-compatible Arimo as the portable stand-in.
pub const FONT_SANS: &str =
    r#""Geneva", "Arimo", "Helvetica Neue", Helvetica, Arial, sans-serif"#;

/// The classic-Mac monospace: Monaco first, bundled Cousine as the portable
/// stand-in. platinum.css repeats this stack in its `.font-mono` override —
/// change them together.
pub const FONT_MONO: &str =
    r#""Monaco", "Cousine", "Menlo", ui-monospace, SFMono-Regular, monospace"#;

/// encre-css config for utility generation: preflight OFF (`PlatinumStyles`
/// already ships the design system's) and the full [`TOKENS`] registry.
pub fn config() -> encre_css::Config {
    let mut config = encre_css::Config::default();
    config.preflight = encre_css::Preflight::None;
    for name in TOKENS {
        config.theme.colors.add(*name, format!("var(--{name})"));
    }
    config
}

/// The design system's preflight, fonts pointed at the Platinum stacks. Used
/// by platinum-ui's own build.rs; apps never need this — `PLATINUM_CSS`
/// already contains the generated output.
pub fn preflight() -> encre_css::Preflight {
    encre_css::Preflight::new_full()
        .font_family_sans(FONT_SANS)
        .font_family_mono(FONT_MONO)
}

/// The complete app-side build script: read the `.rs` sources under
/// `src_dir` (sorted, so output is deterministic), generate utility CSS with
/// [`config`], write it to `$OUT_DIR/<out_name>`, and emit the
/// `rerun-if-changed` line.
///
/// CAVEAT (inherited from encre-css): classes it can't parse are skipped
/// SILENTLY — if a new class has no visible effect, check it against the
/// "encre-parseable" constraint in platinum-ui's CLAUDE.md.
pub fn generate_utilities(src_dir: &str, out_name: &str) {
    println!("cargo:rerun-if-changed={src_dir}");
    let mut entries: Vec<_> = std::fs::read_dir(src_dir)
        .expect("read source dir")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "rs"))
        .collect();
    entries.sort();
    let sources: Vec<String> = entries
        .iter()
        .map(|p| std::fs::read_to_string(p).expect("read source file"))
        .collect();
    let css = encre_css::generate(sources.iter().map(String::as_str), &config());
    write_out(out_name, &css);
}

/// Generate the preflight stylesheet ([`preflight`]) to `$OUT_DIR/<out_name>`.
/// Only platinum-ui's own build.rs needs this.
pub fn generate_preflight(out_name: &str) {
    let mut config = encre_css::Config::default();
    config.preflight = preflight();
    let css = encre_css::generate([], &config);
    write_out(out_name, &css);
}

fn write_out(name: &str, contents: &str) {
    let out = std::path::Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR")).join(name);
    std::fs::write(out, contents).expect("write generated css");
}

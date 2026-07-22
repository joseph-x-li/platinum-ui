//! Generates utility CSS for the showcase's OWN classes, the same way the
//! library's build.rs does for component-emitted ones. `PLATINUM_CSS`
//! (injected via `<PlatinumStyles/>`) only covers classes the library
//! components emit — layout/typography utilities used by these demo pages
//! live here, compiled by encre-css and injected as a second `<style>`.

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src");

    let mut entries: Vec<_> = std::fs::read_dir("src")
        .expect("read src/")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "rs"))
        .collect();
    entries.sort();
    let sources: Vec<String> = entries
        .iter()
        .map(|p| std::fs::read_to_string(p).expect("read source file"))
        .collect();

    let mut config = encre_css::Config::default();
    // PlatinumStyles already ships the design system's preflight; only the
    // showcase-specific utilities are generated here.
    config.preflight = encre_css::Preflight::None;
    // The full token set platinum.css defines — pointed at the :root vars.
    for name in [
        "background",
        "foreground",
        "card",
        "card-foreground",
        "muted",
        "muted-foreground",
        "accent",
        "accent-foreground",
        "destructive",
        "warning",
        "success",
        "border",
    ] {
        config.theme.colors.add(name, format!("var(--{name})"));
    }

    let css = encre_css::generate(sources.iter().map(String::as_str), &config);
    let out = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR"))
        .join("showcase-utilities.css");
    std::fs::write(out, css).expect("write generated css");
}

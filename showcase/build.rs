//! Utility CSS for the showcase's OWN classes — `PLATINUM_CSS` (via
//! `<PlatinumStyles/>`) only covers classes the library components emit.
//! The shared platinum-ui-build config supplies the token registry and
//! leaves preflight off (PlatinumStyles already ships it).

fn main() {
    platinum_ui_build::generate_utilities("src", "showcase-utilities.css");
}

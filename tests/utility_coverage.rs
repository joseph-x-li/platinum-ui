//! Guards the standalone-mode pipeline: every Tailwind-compatible utility
//! class the components emit must come out of build.rs's encre-css run as a
//! real rule in [`platinum_ui::PLATINUM_CSS`]. encre-css skips classes it
//! can't parse SILENTLY, so without this test an unsupported class ships as
//! dead markup for non-Tailwind consumers (see CLAUDE.md, "encre-parseable"
//! constraint).
//!
//! MAINTENANCE: when a component gains a utility class, add it to
//! `EMITTED_UTILITIES` below. If this test fails for a class you added, the
//! class is beyond encre-css's parser — express it as a plain rule in
//! platinum.css instead (like the Button icon-size rule).

/// Every utility class currently emitted by a component in src/.
const EMITTED_UTILITIES: &[&str] = &[
    // ui_button.rs — BASE
    "inline-flex",
    "items-center",
    "justify-center",
    "gap-2",
    "whitespace-nowrap",
    "text-sm",
    "font-medium",
    "disabled:pointer-events-none",
    "[&_svg]:pointer-events-none",
    "shrink-0",
    "[&_svg]:shrink-0",
    "outline-none",
    "w-fit",
    "hover:cursor-pointer",
    "touch-manipulation",
    "[-webkit-tap-highlight-color:transparent]",
    "select-none",
    "[-webkit-touch-callout:none]",
    // ui_button.rs — sizes
    "h-9",
    "px-4",
    "py-2",
    "has-[>svg]:px-3",
    "h-8",
    "gap-1.5",
    "px-3",
    "has-[>svg]:px-2.5",
    "h-10",
    "px-6",
    "has-[>svg]:px-4",
    // ui_dialog.rs — content, backdrop, close button, wrappers
    "p-6",
    // platinum_scroll.rs — PlatinumTextarea inner padding/typography
    "p-3",
    "w-[calc(100%-2rem)]",
    "max-h-[85vh]",
    "fixed",
    "top-[50%]",
    "left-[50%]",
    // NOTE: translate-* (and other composed-var utilities) resolve against
    // --en-* defaults that live in the PREFLIGHT — build.rs ships it as part
    // of PLATINUM_CSS so they do work, but remember this test only proves
    // rule-EXISTS, not rule-works. The
    // dialog's centering pull-back predates that and stays a plain
    // [data-target] rule in platinum.css.
    "z-100",
    "data-[state=closed]:opacity-0",
    "data-[state=open]:opacity-100",
    "inset-0",
    "pointer-events-none",
    "z-60",
    "bg-black/50",
    "absolute",
    "top-4",
    "right-4",
    "p-1",
    "focus:ring-2",
    "focus:ring-offset-2",
    "focus:outline-none",
    "text-muted-foreground",
    "hover:text-foreground",
    "hidden",
    "flex",
    "flex-col",
    "gap-4",
    "text-center",
    "sm:text-left",
    "text-lg",
    "leading-none",
    "font-semibold",
    "flex-col-reverse",
    "sm:flex-row",
    "sm:justify-end",
    // ui_collapsible.rs
    "grid",
    "overflow-hidden",
    "transition-all",
    "duration-300",
    "data-[state=closed]:grid-rows-[0fr]",
    "data-[state=open]:grid-rows-[1fr]",
    "min-h-0",
    // platinum_scroll.rs — content wrapper
    "min-h-full",
    // platinum_meter.rs
    "relative",
    "inset-y-0",
    "left-0",
    "bg-success",
    "bg-destructive",
    // platinum_info.rs — PropertyList / Property / StatWell
    "grid-cols-[auto_1fr]",
    "gap-x-4",
    "gap-y-2",
    "items-baseline",
    "text-xs",
    "uppercase",
    "tracking-wide",
    "font-mono",
    "break-all",
    "tabular-nums",
    // platinum_shell.rs
    "h-screen",
    "py-4",
    "gap-6",
    "tracking-tight",
    "ml-auto",
    "flex-1",
    "h-full",
    "py-6",
    "w-full",
    "max-w-6xl",
    "mx-auto",
];

/// Spot checks that the hand-written skin made it into the concatenation.
const SKIN_SELECTORS: &[&str] = &[
    "[data-name=\"Button\"]",
    ".pl-well",
    ".pl-plaque",
    ".pl-palette",
    ".pl-tabs",
    ".pl-scroll-thumb",
    ".pl-select-menu",
    ".pl-well-fitted",
];

/// CSS-escape a class name the way Tailwind/encre selectors do.
fn css_escape(class: &str) -> String {
    class
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c.to_string()
            } else {
                format!("\\{c}")
            }
        })
        .collect()
}

#[test]
fn every_emitted_utility_has_generated_css() {
    let css = platinum_ui::PLATINUM_CSS;
    let missing: Vec<&&str> = EMITTED_UTILITIES
        .iter()
        .filter(|class| !css.contains(&format!(".{}", css_escape(class))))
        .collect();
    assert!(
        missing.is_empty(),
        "encre-css generated no rule for: {missing:?}\n\
         Either the class is new (build ran on stale sources?) or it is \
         beyond encre-css's parser — move it into platinum.css as a plain \
         rule. See CLAUDE.md."
    );
}

#[test]
fn skin_is_included() {
    for sel in SKIN_SELECTORS {
        assert!(
            platinum_ui::PLATINUM_CSS.contains(sel),
            "platinum.css selector missing from PLATINUM_CSS: {sel}"
        );
    }
}

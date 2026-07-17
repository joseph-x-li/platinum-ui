//! platinum-ui — a Mac OS Platinum-styled Leptos component library.
//!
//! Components (shadcn-for-Leptos primitives reskinned Platinum, plus a few
//! custom controls). Pair with `platinum.css` (shipped in this crate) and add
//! this crate's `src` to your Tailwind `@source` scan.

pub mod debounce_button;
pub mod platinum_scroll;

/// Join a component's base utility classes with caller-supplied ones — plain
/// concatenation, deliberately NOT tw_merge: its class-name parsing mis-reads
/// design-system classes (`pl-*` → Tailwind padding-left) and silently deletes
/// them (see ui_button.rs). Base class lists are written to compose without
/// utility conflicts instead of relying on a merger to resolve them.
pub(crate) fn classes(base: &str, extra: &str) -> String {
    if extra.is_empty() {
        base.to_string()
    } else if base.is_empty() {
        extra.to_string()
    } else {
        format!("{base} {extra}")
    }
}
pub mod platinum_select;
pub mod ui_button;
pub mod ui_collapsible;
pub mod ui_dialog;

// Flat re-exports for the common case.
pub use debounce_button::{boxed, BoxFut, DebounceButton, DEBOUNCE_MS};
pub use platinum_scroll::{PlatinumScroll, ScrollWell};
pub use platinum_select::PlatinumSelect;
pub use ui_button::{Button, ButtonSize, ButtonVariant};
pub use ui_collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

//! platinum-ui — a Mac OS Platinum-styled Leptos component library.
//!
//! Components (shadcn-for-Leptos primitives reskinned Platinum, plus a few
//! custom controls). Pair with `platinum.css` (shipped in this crate) and add
//! this crate's `src` to your Tailwind `@source` scan.

pub mod debounce_button;
pub mod platinum_scroll;
pub mod platinum_select;
pub mod ui_button;
pub mod ui_collapsible;
pub mod ui_dialog;

// Flat re-exports for the common case.
pub use debounce_button::{boxed, BoxFut, DebounceButton, DEBOUNCE_MS};
pub use platinum_scroll::PlatinumScroll;
pub use platinum_select::PlatinumSelect;
pub use ui_button::{Button, ButtonSize, ButtonVariant};
pub use ui_collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

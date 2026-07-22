//! The Platinum application window: pinned header platform (brand + palette
//! nav), full-bleed content platform below. Lifted from chdkpano's app
//! shell.

use leptos::children::ViewFn;
use leptos::prelude::*;

use crate::classes;

/// Full-viewport app frame.
///
/// Layout contract: the shell fills the viewport and never scrolls — the
/// header stays pinned and the content pane is FIXED height, so pages own
/// their scrolling (put long content in a `ScrollWell`/`PlatinumScroll`
/// sized off the `h-full` chain; a page-level Platinum scrollbar then reads
/// as the content pane's own). Short pages just sit on the platform.
///
/// Slots:
/// - `title`: brand markup for the `<h1>` (logo + name).
/// - `nav` (optional): entries for the palette-plaque nav — typically router
///   `<A attr:data-name="Button">` links; the active one should carry
///   `aria-current="page"` (leptos_router's `<A>` does this) to render
///   pressed.
/// - `aside` (optional): right-aligned header extras (settings, dark toggle).
/// - `children`: page content, centered at `max-w-6xl` (override the wrapper
///   with `content_class`, e.g. to widen or remove the cap).
#[component]
pub fn Shell(
    #[prop(into)] title: ViewFn,
    #[prop(into, optional)] nav: Option<ViewFn>,
    #[prop(into, optional)] aside: Option<ViewFn>,
    /// Replaces the DEFAULT content wrapper classes
    /// (`w-full max-w-6xl mx-auto px-6 h-full`) when non-empty, rather than
    /// appending — width caps don't compose, so merging would conflict.
    #[prop(into, optional)] content_class: String,
    children: Children,
) -> impl IntoView {
    let content_class = if content_class.is_empty() {
        "w-full max-w-6xl mx-auto px-6 h-full".to_string()
    } else {
        content_class
    };
    view! {
        <div class="h-screen flex flex-col overflow-hidden" data-name="Shell">
            <header class="pl-platform px-6 py-4 flex items-center gap-6 shrink-0">
                <h1 class="text-lg font-semibold tracking-tight flex items-center gap-2">
                    {title.run()}
                </h1>
                {nav.map(|nav| view! {
                    <nav class="pl-palette pl-palette-plaque">{nav.run()}</nav>
                })}
                {aside.map(|aside| view! {
                    <div class="ml-auto flex items-center gap-2">{aside.run()}</div>
                })}
            </header>
            <main class="flex-1 min-h-0 overflow-hidden">
                // Full-bleed raised platform: its BEVEL spans the whole pane;
                // only the frame reaches the edges, content stays centered.
                <div class="pl-platform pl-seam-top h-full py-6">
                    <div class=classes(&content_class, "")>{children()}</div>
                </div>
            </main>
        </div>
    }
}

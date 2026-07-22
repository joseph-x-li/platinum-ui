//! Platinum sidebar: a vertical navigation strip of raised entries.
//!
//! The vertical dual of the tool palette (`.pl-palette`): full-width raised
//! button cells butted into one column, 1px gaps showing the strip's #555
//! background as dividers, the strip's own ring as the outer frame. The
//! active entry takes `aria-current="page"` — the raised group's pressed
//! hook — so selection reads as a pressed-in cell, exactly like the
//! app-chrome nav buttons.

use leptos::prelude::*;

/// Vertical navigation strip. `entries` are the labels in display order;
/// `selected` holds the active index and is written on click.
///
/// The caller owns the width: `<Sidebar entries=… selected=… class="w-44"/>`.
/// The strip is `height: fit-content`, so inside a flex row it hugs its
/// entries instead of stretching to the row.
#[component]
pub fn Sidebar(
    entries: Vec<String>,
    selected: RwSignal<usize>,
    /// Extra classes for the strip (width/sizing is the caller's job).
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    view! {
        <nav class=crate::classes("pl-sidebar", &class)>
            {entries
                .into_iter()
                .enumerate()
                .map(|(i, label)| {
                    view! {
                        <button
                            type="button"
                            data-name="Button"
                            class="inline-flex items-center px-3 py-2 text-sm"
                            aria-current=move || (selected.get() == i).then_some("page")
                            on:click=move |_| selected.set(i)
                        >
                            {label}
                        </button>
                    }
                })
                .collect_view()}
        </nav>
    }
}

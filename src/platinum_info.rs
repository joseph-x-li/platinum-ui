//! Info-display widgets lifted from chdkpano's camera-detail page: the
//! two-column key-value list and the labeled stat well.

use leptos::prelude::*;

use crate::classes;

/// Two-column key-value layout: labels in a muted uppercase column, values
/// beside them. Fill it with [`Property`] children.
#[component]
pub fn PropertyList(
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <dl
            class=classes(
                "grid grid-cols-[auto_1fr] gap-x-4 gap-y-2 text-sm items-baseline",
                &class,
            )
            data-name="PropertyList"
        >
            {children()}
        </dl>
    }
}

/// One row of a [`PropertyList`]. `mono` renders the value in the monospace
/// stack (ids, hex, versions); empty values show as "(empty)" so rows never
/// silently collapse.
#[component]
pub fn Property(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(optional)] mono: bool,
) -> impl IntoView {
    let value_cls = if mono { "font-mono text-xs break-all" } else { "" };
    view! {
        <dt class="text-xs text-muted-foreground uppercase tracking-wide">{label}</dt>
        <dd class=value_cls>
            {if value.is_empty() { "(empty)".to_string() } else { value }}
        </dd>
    }
}

/// A dashboard stat: small muted label over a large tabular-nums value,
/// seated in a well.
#[component]
pub fn StatWell(
    #[prop(into)] label: String,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=classes("pl-well px-3 py-2", &class) data-name="StatWell">
            <div class="text-xs text-muted-foreground">{label}</div>
            <div class="text-lg font-semibold tabular-nums">{children()}</div>
        </div>
    }
}

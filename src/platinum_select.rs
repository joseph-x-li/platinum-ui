//! Reusable Mac OS Platinum popup menu — a hand-drawn replacement for a native
//! `<select>`. The container is a recessed *well* showing the current value; the
//! only interactive part is the raised arrow button, which opens a white overlay
//! menu (black outline + drop shadow, ✓ on the current item, inverted hover).
//! Two-way bound to an `RwSignal<T>` and generic over the value type, so it drops
//! in anywhere a typed select is needed (ISO `i32`, focus `Option<i32>`, …).
//!
//! The menu is `position: fixed`, placed from the field's viewport rect when it
//! opens (and nudged up if it would run off the bottom of the window). Absolute
//! positioning would be clipped by any overflow ancestor — a ScrollWell, or the
//! app's overflow-hidden content pane — leaving bottom options unreachable.
//! While it's open a CSS rule freezes every .pl-scroll-view (same pattern as
//! dialogs), so the fixed menu can't drift away from its field.

use leptos::html;
use leptos::prelude::*;

#[component]
pub fn PlatinumSelect<T>(
    /// `(label, value)` options, in display order.
    options: Vec<(String, T)>,
    /// Two-way bound selected value.
    selected: RwSignal<T>,
    /// Minimum field width in px (the field still grows to fit a longer label).
    #[prop(default = 130)] min_width: u32,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let open = RwSignal::new(false);
    let options = StoredValue::new(options);

    let field_ref: NodeRef<html::Div> = NodeRef::new();
    let menu_ref: NodeRef<html::Ul> = NodeRef::new();
    // Viewport (top, left, width) for the fixed-position menu, captured from
    // the field when the menu opens. -1/+2 offsets: the menu overlays the
    // field's 1px outline, Mac-style.
    let menu_pos = RwSignal::new((0.0_f64, 0.0_f64, 0.0_f64));

    let toggle = move |_| {
        if !open.get_untracked() {
            if let Some(field) = field_ref.get_untracked() {
                let r = field.get_bounding_client_rect();
                menu_pos.set((r.top() - 1.0, r.left() - 1.0, r.width() + 2.0));
            }
        }
        open.update(|o| *o = !*o);
    };

    // After the menu renders, nudge it up if it runs off the window bottom
    // (classic Mac popup menus slide to stay on screen). menu_ref.get() tracks,
    // so this runs exactly when the <Show> mounts the list.
    Effect::new(move |_| {
        let Some(menu) = menu_ref.get() else { return };
        let rect = menu.get_bounding_client_rect();
        let win_h = window()
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(f64::MAX);
        let overflow = rect.bottom() - (win_h - 4.0);
        if overflow > 0.0 {
            menu_pos.update(|pos| pos.0 = (pos.0 - overflow).max(4.0));
        }
    });

    // Label for the current value (empty if it isn't in the list).
    let current_label = move || {
        options.with_value(|opts| {
            let sel = selected.get();
            opts.iter()
                .find(|(_, v)| *v == sel)
                .map(|(l, _)| l.clone())
                .unwrap_or_default()
        })
    };

    view! {
        <div class="pl-select">
            <div class="pl-select-field" node_ref=field_ref style=format!("min-width:{min_width}px")>
                <span class="pl-select-label">{current_label}</span>
                // Seams: the three well-facing sides drop their outline so the
                // well's own outline is the single line there; only the left
                // edge (the divider from the label) draws one.
                <button
                    type="button"
                    class="pl-select-arrow pl-seam-top pl-seam-right pl-seam-bottom"
                    aria-label="Open menu"
                    on:click=toggle
                ></button>
            </div>
            <Show when=move || open.get()>
                // Full-viewport catcher: any click outside the menu closes it.
                <div class="pl-select-backdrop" on:click=move |_| open.set(false)></div>
                <ul
                    class="pl-select-menu"
                    role="listbox"
                    node_ref=menu_ref
                    style=move || {
                        let (top, left, width) = menu_pos.get();
                        format!("top:{top}px;left:{left}px;min-width:{width}px")
                    }
                >
                    {options.with_value(|opts| {
                        opts.iter().map(|(label, val)| {
                            let label = label.clone();
                            let v_click = val.clone();
                            let v_sel = val.clone();
                            let v_check = val.clone();
                            view! {
                                <li
                                    class="pl-select-option"
                                    class:is-selected=move || selected.get() == v_sel
                                    role="option"
                                    on:click=move |_| { selected.set(v_click.clone()); open.set(false); }
                                >
                                    <span class="pl-select-check">
                                        {move || if selected.get() == v_check { "✓" } else { "" }}
                                    </span>
                                    {label}
                                </li>
                            }
                        }).collect_view()
                    })}
                </ul>
            </Show>
        </div>
    }
}

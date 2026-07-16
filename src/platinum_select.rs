//! Reusable Mac OS Platinum popup menu — a hand-drawn replacement for a native
//! `<select>`. The container is a recessed *well* showing the current value; the
//! only interactive part is the raised arrow button, which opens a white overlay
//! menu (black outline + drop shadow, ✓ on the current item, inverted hover).
//! Two-way bound to an `RwSignal<T>` and generic over the value type, so it drops
//! in anywhere a typed select is needed (ISO `i32`, focus `Option<i32>`, …).

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
            <div class="pl-select-field" style=format!("min-width:{min_width}px")>
                <span class="pl-select-label">{current_label}</span>
                // Seams: the three well-facing sides drop their outline so the
                // well's own outline is the single line there; only the left
                // edge (the divider from the label) draws one.
                <button
                    type="button"
                    class="pl-select-arrow pl-seam-top pl-seam-right pl-seam-bottom"
                    aria-label="Open menu"
                    on:click=move |_| open.update(|o| *o = !*o)
                ></button>
            </div>
            <Show when=move || open.get()>
                // Full-viewport catcher: any click outside the menu closes it.
                <div class="pl-select-backdrop" on:click=move |_| open.set(false)></div>
                <ul class="pl-select-menu" role="listbox">
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

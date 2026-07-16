//! Reusable debounced button — the "Capture" trick, factored out.
//!
//! Shows a `pending` label on a *disabled* button while its action runs, held
//! for at least `min_ms` so a fast (or fast-failing) action doesn't flicker the
//! label. Re-clicks during that window are ignored. Used by Refresh, Capture,
//! Stitch, … — anywhere a click kicks off work that shouldn't be double-fired.
//!
//! DESIGN: the pending state lives on the DOM node itself (its `disabled`
//! attribute and label text), NOT in reactive signals. The state describes one
//! physical control for the duration of one click, and the node is the one
//! thing guaranteed to survive whatever the action does — an action that
//! refetches the very list the button lives in re-renders the surrounding
//! view, and any instance-owned signal would be disposed mid-flight (stranding
//! a `disabled` attribute, or panicking when a listener on the reused node
//! fires into freed state). Reading/writing the node directly has no lifecycle
//! to race: `disabled` doubles as the re-entry guard, and restoring a node
//! that has since been thrown away is a harmless no-op.

use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::ui_button::{Button, ButtonSize, ButtonVariant};

/// Default minimum hold for the pending state (ms) — 0.2s.
pub const DEBOUNCE_MS: f64 = 200.0;

/// Type-erased `() -> ()` future the action returns. Boxing keeps the component
/// non-generic over the future type, so call sites just wrap their async block
/// with [`boxed`].
pub type BoxFut = Pin<Box<dyn Future<Output = ()>>>;

/// Wrap an async block into a [`BoxFut`] for a `DebounceButton` action.
///
/// ```ignore
/// action=move || boxed(async move { resource.refetch(); })
/// ```
pub fn boxed(fut: impl Future<Output = ()> + 'static) -> BoxFut {
    Box::pin(fut)
}

#[component]
pub fn DebounceButton<F>(
    /// Label shown when idle.
    #[prop(into)] label: String,
    /// Label shown while the action runs (button is disabled).
    #[prop(into)] pending: String,
    /// Runs on click. The button stays disabled until this future resolves AND
    /// at least `min_ms` have elapsed. Wrap your async block with [`boxed`].
    action: F,
    /// Minimum hold for the pending state. Defaults to [`DEBOUNCE_MS`] (0.2s).
    #[prop(default = DEBOUNCE_MS)] min_ms: f64,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
) -> impl IntoView
where
    F: Fn() -> BoxFut + 'static,
{
    let action = Rc::new(action);
    let idle_label = label.clone();

    let click = move |ev: ev::MouseEvent| {
        // The <button> the listener is on; fall back to closest() in case the
        // click target is a child element.
        let Some(btn) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlButtonElement>().ok())
            .or_else(|| {
                ev.target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                    .and_then(|el| el.closest("button").ok().flatten())
                    .and_then(|el| el.dyn_into::<web_sys::HtmlButtonElement>().ok())
            })
        else {
            return;
        };
        // `disabled` on the node IS the re-entry guard (a disabled button
        // already blocks clicks; this also covers programmatic ones).
        if btn.disabled() {
            return;
        }
        btn.set_disabled(true);
        btn.set_text_content(Some(&pending));
        let idle_label = idle_label.clone();
        let action = action.clone();
        let started = js_sys::Date::now();
        wasm_bindgen_futures::spawn_local(async move {
            action().await;
            let elapsed = js_sys::Date::now() - started;
            if elapsed < min_ms {
                gloo_timers::future::TimeoutFuture::new((min_ms - elapsed) as u32).await;
            }
            btn.set_text_content(Some(&idle_label));
            btn.set_disabled(false);
        });
    };

    view! {
        <Button variant=variant size=size on:click=click>{label}</Button>
    }
}

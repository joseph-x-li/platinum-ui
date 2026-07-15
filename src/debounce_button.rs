//! Reusable debounced button — the "Capture" trick, factored out.
//!
//! Shows a `pending` label as a *disabled* button while its action runs, held
//! for at least `min_ms` so a fast (or fast-failing) action doesn't flicker the
//! label. Re-clicks during that window are ignored. Used by Refresh, Capture,
//! Stitch, … — anywhere a click kicks off work that shouldn't be double-fired.

use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use leptos::prelude::*;

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
    let (busy, set_busy) = signal(false);
    let action = Rc::new(action);

    let click = move |_| {
        // Guard re-entry: a disabled button already blocks clicks, but this also
        // covers programmatic clicks / rapid event coalescing.
        if busy.get_untracked() {
            return;
        }
        set_busy.set(true);
        let action = action.clone();
        let started = js_sys::Date::now();
        wasm_bindgen_futures::spawn_local(async move {
            action().await;
            let elapsed = js_sys::Date::now() - started;
            if elapsed < min_ms {
                gloo_timers::future::TimeoutFuture::new((min_ms - elapsed) as u32).await;
            }
            set_busy.set(false);
        });
    };

    view! {
        <Button variant=variant size=size on:click=click attr:disabled=move || busy.get()>
            {move || if busy.get() { pending.clone() } else { label.clone() }}
        </Button>
    }
}

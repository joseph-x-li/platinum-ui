//! A custom, DOM-built Platinum scrollbar with real up/down arrow buttons.
//!
//! Why not CSS? Chrome on macOS refuses to render `::-webkit-scrollbar-button`
//! (the Mac scrollbar theme has no button parts), so the classic Mac scroll
//! arrows can't exist as pure CSS there. This wraps its children in a scroller
//! whose native scrollbar is hidden, and paints its own bar: a raised arrow
//! button at each end and a draggable purple thumb whose size/position track the
//! content. Arrows step the scroll; the thumb drags it; the wheel/keys still
//! scroll the inner element natively and we mirror that via its scroll event.

use gloo_timers::callback::{Interval, Timeout};
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// Pixels scrolled per arrow-button click.
const STEP: i32 = 48;

/// True while the page is pinch-zoomed. Wheel events then pan the zoomed
/// visual viewport — a browser-owned motion we must not preventDefault.
fn pinch_zoomed() -> bool {
    web_sys::window()
        .and_then(|w| w.visual_viewport())
        .map(|v| v.scale() > 1.001)
        .unwrap_or(false)
}
/// Held arrow button: pause before autorepeat kicks in, then tick cadence.
const REPEAT_DELAY_MS: u32 = 350;
const REPEAT_EVERY_MS: u32 = 60;

#[component]
pub fn PlatinumScroll(
    children: Children,
    /// Welled mode, used by [`ScrollWell`]: the bar column stays in layout even
    /// when the content fits (the thumb just fills the whole track), carries
    /// its own divider against the content, and the arrows' seams face the
    /// surrounding well's inset ring rather than the app chrome.
    #[prop(default = false)] welled: bool,
) -> impl IntoView {
    let view_ref: NodeRef<html::Div> = NodeRef::new();
    let content_ref: NodeRef<html::Div> = NodeRef::new();
    let track_ref: NodeRef<html::Div> = NodeRef::new();
    let thumb_ref: NodeRef<html::Div> = NodeRef::new();

    // Live geometry of the inner scroller, mirrored into signals so the thumb
    // can react. All px, as f64.
    let scroll_top = RwSignal::new(0.0);
    let scroll_height = RwSignal::new(1.0);
    let client_height = RwSignal::new(1.0);

    // Pull the current geometry off the DOM into the signals. try_set: the
    // ResizeObserver callback below can outlive this instance by one firing
    // (removing the observed element from the DOM is itself a resize), and a
    // set() into disposed signals would panic and poison the wasm runtime.
    let refresh = move || {
        if let Some(el) = view_ref.get_untracked() {
            scroll_top.try_set(el.scroll_top() as f64);
            scroll_height.try_set(el.scroll_height().max(1) as f64);
            client_height.try_set(el.client_height().max(1) as f64);
        }
    };

    // Arrow buttons: nudge the scroll position by a step.
    let step_by = move |delta: i32| {
        if let Some(el) = view_ref.get_untracked() {
            el.set_scroll_top(el.scroll_top() + delta);
        }
        refresh();
    };

    // Held arrow autorepeat, classic Mac style: one step on press, a beat,
    // then steady ticking until release. The handles cancel on drop, so
    // clearing the stores (or component disposal) stops the repeat. Local
    // storage — Timeout/Interval are not Send.
    let repeat_delay: StoredValue<Option<Timeout>, LocalStorage> = StoredValue::new_local(None);
    let repeat_tick: StoredValue<Option<Interval>, LocalStorage> = StoredValue::new_local(None);
    let stop_repeat = move || {
        repeat_delay.set_value(None);
        repeat_tick.set_value(None);
    };
    let start_repeat = move |delta: i32| {
        step_by(delta);
        let t = Timeout::new(REPEAT_DELAY_MS, move || {
            repeat_tick.set_value(Some(Interval::new(REPEAT_EVERY_MS, move || step_by(delta))));
        });
        repeat_delay.set_value(Some(t));
    };

    // Thumb drag. We capture the starting pointer Y, the starting scrollTop, and
    // a factor mapping thumb-pixels → content-pixels, then follow the mouse via
    // window listeners (so the drag continues outside the thumb).
    let dragging = RwSignal::new(false);
    let drag = StoredValue::new((0.0_f64, 0.0_f64, 1.0_f64)); // (start_y, start_scroll, factor)

    let on_thumb_down = move |e: ev::MouseEvent| {
        e.prevent_default();
        let (Some(view), Some(track), Some(thumb)) = (
            view_ref.get_untracked(),
            track_ref.get_untracked(),
            thumb_ref.get_untracked(),
        ) else {
            return;
        };
        let scrollable = (view.scroll_height() - view.client_height()).max(1) as f64;
        let travel = (track.client_height() - thumb.offset_height()).max(1) as f64;
        drag.set_value((e.client_y() as f64, view.scroll_top() as f64, scrollable / travel));
        dragging.set(true);
    };

    // Attached once, gated on `dragging`, and cleaned up with the component.
    let mv = window_event_listener(ev::mousemove, move |e: ev::MouseEvent| {
        if !dragging.get_untracked() {
            return;
        }
        if let Some(view) = view_ref.get_untracked() {
            let (start_y, start_scroll, factor) = drag.get_value();
            let scrollable = (view.scroll_height() - view.client_height()).max(0) as f64;
            let next = (start_scroll + (e.client_y() as f64 - start_y) * factor).clamp(0.0, scrollable);
            view.set_scroll_top(next as i32);
            refresh();
        }
    });
    let up = window_event_listener(ev::mouseup, move |_| {
        if dragging.get_untracked() {
            dragging.set(false);
        }
        // Also ends a held arrow button, wherever the pointer is released.
        stop_repeat();
    });
    // Observer + its closure live here so unmount can tear them down — this
    // component used to be app-lifetime (the old whole-pane scroller) and got
    // away with leaking them; ScrollWell instances mount and unmount with
    // their page, so a leaked observer would fire into disposed signals.
    let observer: StoredValue<Option<(web_sys::ResizeObserver, Closure<dyn FnMut()>)>, LocalStorage> =
        StoredValue::new_local(None);

    let rz = window_event_listener(ev::resize, move |_| refresh());
    on_cleanup(move || {
        mv.remove();
        up.remove();
        rz.remove();
        observer.update_value(|o| {
            if let Some((obs, _cb)) = o.take() {
                obs.disconnect();
            }
        });
    });

    // Re-measure whenever the content's size changes — a route swap, async data
    // loading in AFTER navigation, or a reflow. A scroll event only fires when
    // you actually scroll, so without this the bar lags a page behind (a plain
    // route-change hook fires too early, before async content lands). The
    // observer also fires once on observe(), covering the initial measure.
    let observer_set = StoredValue::new(false);
    Effect::new(move |_| {
        let Some(content) = content_ref.get() else {
            return;
        };
        if observer_set.get_value() {
            return;
        }
        observer_set.set_value(true);
        let cb = Closure::<dyn FnMut()>::new(move || refresh());
        if let Ok(obs) = web_sys::ResizeObserver::new(cb.as_ref().unchecked_ref()) {
            obs.observe(&content);
            // Parked (with its closure) for the on_cleanup above to disconnect.
            observer.set_value(Some((obs, cb)));
        }
    });

    // Thumb size/position as a % of the track (content-proportional).
    let thumb_height = move || (client_height.get() / scroll_height.get() * 100.0).min(100.0);
    let thumb_top = move || scroll_top.get() / scroll_height.get() * 100.0;

    // Only show the bar when the content actually overflows — otherwise a short
    // page would carry a full-height, do-nothing scrollbar (native `auto` hides
    // it). Wheel/keys still scroll the view natively, and that scroll event
    // re-measures, so the bar appears the moment content grows past the pane.
    // (Welled mode instead keeps the bar always; a fitting page just shows a
    // full-track thumb, since thumb height is client/scroll = 100% there.)
    let scrollable = move || scroll_height.get() > client_height.get() + 1.0;

    // Which outline edges the arrows decline to draw depends on who the
    // neighbors are: in the app chrome the content panel provides the left
    // line and the header the up arrow's top line; inside a well the inset
    // ring provides top/right/bottom and the bar's own divider the left.
    let (up_seams, down_seams) = if welled {
        (
            "pl-seam-left pl-seam-top pl-seam-right",
            "pl-seam-left pl-seam-bottom pl-seam-right",
        )
    } else {
        ("pl-seam-left pl-seam-top", "pl-seam-left")
    };

    view! {
        <div class="pl-scroll">
            // Scrolling is fully native: wheel, keys, momentum, chaining,
            // latching, pinch-zoom and zoomed panning are all the browser's.
            // We only MIRROR the geometry (scroll event + ResizeObserver)
            // into the hand-drawn bar, and write scrollTop from its parts.
            <div class="pl-scroll-view" node_ref=view_ref on:scroll=move |_| refresh()>
                // Wrapper exists so the ResizeObserver has a single element whose
                // height tracks the content (the view's own height is fixed). It's
                // at least the pane height (min-h-full, resolvable since the view
                // is flex-stretched) and a flex column, so a page's content panel
                // can grow to fill the pane's bottom; it still grows past it to
                // scroll when the content is taller.
                <div node_ref=content_ref class="flex flex-col min-h-full">{children()}</div>
            </div>
            <div
                class=if welled { "pl-scrollbar pl-scrollbar-welled" } else { "pl-scrollbar" }
                class:pl-scrollbar-hidden=move || !welled && !scrollable()
                // Native scrollbars scroll the content when you wheel over
                // them; this bar is a separate element (a SIBLING of the
                // view, so it's not in the view's scroll chain) — forward
                // its vertical wheel to the view. preventDefault, or the
                // browser's default action would scroll an OUTER scroller
                // on top of the forward. Zoom/horizontal/zoomed-pan wheels
                // pass through untouched, same as anywhere else.
                on:wheel=move |e: ev::WheelEvent| {
                    if e.ctrl_key() || e.delta_x().abs() > e.delta_y().abs() || pinch_zoomed() {
                        return;
                    }
                    e.prevent_default();
                    if let Some(view) = view_ref.get_untracked() {
                        // Firefox reports line deltas (mode 1); normalize.
                        let dy = if e.delta_mode() == 1 { e.delta_y() * 33.0 } else { e.delta_y() };
                        view.set_scroll_top(view.scroll_top() + dy as i32);
                        refresh();
                    }
                }
            >
                <button
                    type="button"
                    class=format!("pl-scroll-arrow pl-scroll-up {up_seams}")
                    aria-label="Scroll up"
                    on:mousedown=move |_| start_repeat(-STEP)
                    on:mouseleave=move |_| stop_repeat()
                    // Mouse presses are handled (and repeated) via mousedown;
                    // click stays for keyboard activation only (detail == 0).
                    on:click=move |e: ev::MouseEvent| if e.detail() == 0 { step_by(-STEP) }
                ></button>
                <div class="pl-scroll-track" node_ref=track_ref>
                    <div
                        class="pl-scroll-thumb"
                        node_ref=thumb_ref
                        on:mousedown=on_thumb_down
                        style:height=move || format!("{}%", thumb_height())
                        style:top=move || format!("{}%", thumb_top())
                    ></div>
                </div>
                <button
                    type="button"
                    class=format!("pl-scroll-arrow pl-scroll-down {down_seams}")
                    aria-label="Scroll down"
                    on:mousedown=move |_| start_repeat(STEP)
                    on:mouseleave=move |_| stop_repeat()
                    on:click=move |e: ev::MouseEvent| if e.detail() == 0 { step_by(STEP) }
                ></button>
            </div>
        </div>
    }
}

/// A self-contained scrollable section: a well whose right-hand side is a
/// permanently visible Platinum scrollbar and whose left-hand side is the
/// scrolling content. Unlike the app-chrome [`PlatinumScroll`], the bar keeps
/// its place even when the content fits — the thumb simply fills the whole
/// track with nowhere to go. The well's 1px fitted padding seats both sides
/// just inside its inset outline ring.
///
/// The caller owns the size: `<ScrollWell class="h-40 w-72">…</ScrollWell>`.
/// Give the content its own padding.
#[component]
pub fn ScrollWell(
    children: Children,
    /// Sizing/extra classes for the well box (height is the caller's job).
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    view! {
        <div class=crate::classes("pl-well pl-well-fitted", &class)>
            <PlatinumScroll welled=true>{children()}</PlatinumScroll>
        </div>
    }
}

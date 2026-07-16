//! A custom, DOM-built Platinum scrollbar with real up/down arrow buttons.
//!
//! Why not CSS? Chrome on macOS refuses to render `::-webkit-scrollbar-button`
//! (the Mac scrollbar theme has no button parts), so the classic Mac scroll
//! arrows can't exist as pure CSS there. This wraps its children in a scroller
//! whose native scrollbar is hidden, and paints its own bar: a raised arrow
//! button at each end and a draggable purple thumb whose size/position track the
//! content. Arrows step the scroll; the thumb drags it; the wheel/keys still
//! scroll the inner element natively and we mirror that via its scroll event.

use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// Pixels scrolled per arrow-button click.
const STEP: i32 = 48;

#[component]
pub fn PlatinumScroll(children: Children) -> impl IntoView {
    let view_ref: NodeRef<html::Div> = NodeRef::new();
    let content_ref: NodeRef<html::Div> = NodeRef::new();
    let track_ref: NodeRef<html::Div> = NodeRef::new();
    let thumb_ref: NodeRef<html::Div> = NodeRef::new();

    // Live geometry of the inner scroller, mirrored into signals so the thumb
    // can react. All px, as f64.
    let scroll_top = RwSignal::new(0.0);
    let scroll_height = RwSignal::new(1.0);
    let client_height = RwSignal::new(1.0);

    // Pull the current geometry off the DOM into the signals.
    let refresh = move || {
        if let Some(el) = view_ref.get_untracked() {
            scroll_top.set(el.scroll_top() as f64);
            scroll_height.set(el.scroll_height().max(1) as f64);
            client_height.set(el.client_height().max(1) as f64);
        }
    };

    // Arrow buttons: nudge the scroll position by a step.
    let step_by = move |delta: i32| {
        if let Some(el) = view_ref.get_untracked() {
            el.set_scroll_top(el.scroll_top() + delta);
        }
        refresh();
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
    });
    let rz = window_event_listener(ev::resize, move |_| refresh());
    on_cleanup(move || {
        mv.remove();
        up.remove();
        rz.remove();
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
            std::mem::forget(obs); // app-lifetime scroller: keep the observer alive
        }
        cb.forget();
    });

    // Thumb size/position as a % of the track (content-proportional).
    let thumb_height = move || (client_height.get() / scroll_height.get() * 100.0).min(100.0);
    let thumb_top = move || scroll_top.get() / scroll_height.get() * 100.0;

    // Only show the bar when the content actually overflows — otherwise a short
    // page would carry a full-height, do-nothing scrollbar (native `auto` hides
    // it). Wheel/keys still scroll the view natively, and that scroll event
    // re-measures, so the bar appears the moment content grows past the pane.
    let scrollable = move || scroll_height.get() > client_height.get() + 1.0;

    view! {
        <div class="pl-scroll">
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
                class="pl-scrollbar"
                class:pl-scrollbar-hidden=move || !scrollable()
                // Native scrollbars scroll the content when you wheel over them;
                // this bar is a separate element, so forward its wheel to the view.
                on:wheel=move |e: ev::WheelEvent| {
                    if let Some(view) = view_ref.get_untracked() {
                        view.set_scroll_top(view.scroll_top() + e.delta_y() as i32);
                    }
                }
            >
                <button
                    type="button"
                    class="pl-scroll-arrow pl-scroll-up"
                    aria-label="Scroll up"
                    on:click=move |_| step_by(-STEP)
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
                    class="pl-scroll-arrow pl-scroll-down"
                    aria-label="Scroll down"
                    on:click=move |_| step_by(STEP)
                ></button>
            </div>
        </div>
    }
}

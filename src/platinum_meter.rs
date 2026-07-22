//! A recessed level indicator — the classic Mac gauge: a fitted well whose
//! fill width tracks a fraction. Doubles as a determinate progress bar.
//! Lifted from chdkpano's viewfinder battery meter.

use leptos::prelude::*;

use crate::classes;

/// A fitted well containing a proportional fill.
///
/// Not reactive on its own — rebuild it inside a reactive closure (the
/// library-wide pattern for display components). The caller MUST size the
/// track via `class` (e.g. `"w-6 h-2"`); like `ScrollWell`, the component
/// carries no default size to keep base class lists free of utility
/// conflicts.
#[component]
pub fn Meter(
    /// Fill fraction; clamped to 0.0–1.0.
    fraction: f64,
    /// At or below this fraction the fill switches to the destructive color
    /// (low battery, disk almost full, …).
    #[prop(optional)] danger_below: Option<f64>,
    /// Track sizing utilities (width/height). Required in practice.
    #[prop(into, optional)] class: String,
    /// Fill color utility; defaults to `bg-success`.
    #[prop(into, optional)] fill_class: String,
    /// Tooltip text.
    #[prop(into, optional)] title: String,
) -> impl IntoView {
    let frac = fraction.clamp(0.0, 1.0);
    let danger = danger_below.is_some_and(|d| frac <= d);
    let fill = if danger {
        "bg-destructive".to_string()
    } else if fill_class.is_empty() {
        "bg-success".to_string()
    } else {
        fill_class
    };
    view! {
        <div class="pl-well pl-well-fitted w-fit" data-name="Meter" title=title>
            <div class=classes("relative overflow-hidden", &class)>
                <div
                    class=classes("absolute inset-y-0 left-0", &fill)
                    style=format!("width:{:.1}%", frac * 100.0)
                ></div>
            </div>
        </div>
    }
}

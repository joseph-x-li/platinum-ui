//! Dark-palette switch: the `.dark` class on the document element opts the
//! whole page into platinum.css's dark tokens (see the `.dark` blocks there).

use leptos::prelude::*;

/// Returns a signal driving the dark palette: setting it true/false
/// adds/removes `.dark` on `<html>`, so EVERYTHING — including portals and
/// popups mounted outside the app root — recolors. Starts light; wire the
/// signal to a Button for a toggle:
///
/// ```ignore
/// let dark = use_dark_mode();
/// view! { <Button on:click=move |_| dark.update(|d| *d = !*d)>"Dark"</Button> }
/// ```
pub fn use_dark_mode() -> RwSignal<bool> {
    let dark = RwSignal::new(false);
    Effect::new(move |_| {
        let is_dark = dark.get();
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            let list = el.class_list();
            let _ = if is_dark {
                list.add_1("dark")
            } else {
                list.remove_1("dark")
            };
        }
    });
    dark
}

use leptos::prelude::*;

// Hand-written (no leptos_ui::variants! / tw_merge): the macro ran every class
// through tw_merge, which classifies ANY `pl-*` class as Tailwind padding-left
// (["pl", ..] with no value validation) and deletes it against the size
// classes' later `px-*` — so the design system's pl-* classes could never
// survive the trip through a Button. Plain string concatenation has no such
// opinions. The visual construction (face, outline border, mitered ::after
// bevel, pressed state) all lives in platinum.css keyed off data-name="Button",
// so the utility strings here carry only what the skin does NOT own: layout,
// typography, sizing, and interaction affordances.
// (No disabled:opacity-* here: fading the whole box would dim the outline and
// well bevel too. The skin's :disabled rules fade only the label + platform
// bevel — see platinum.css.)
// (Default icon sizing — svgs without a size-* class get 1rem — lives in
// platinum.css as a [data-name="Button"] rule: its old utility form,
// [&_svg:not([class*='size-'])]:size-4, is the one class encre-css can't
// parse, and every utility here must stay encre-parseable — see build.rs.)
const BASE: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium disabled:pointer-events-none [&_svg]:pointer-events-none shrink-0 [&_svg]:shrink-0 outline-none w-fit hover:cursor-pointer touch-manipulation [-webkit-tap-highlight-color:transparent] select-none [-webkit-touch-callout:none]";

/// The two constructions a button can wear. Default is the raised platform;
/// Plaque emits pl-plaque, swapping in the platform-seated-in-a-well composite
/// (the raised CSS group excludes it via :not(.pl-plaque), the well group
/// dresses it; pressing still inverts only the inner platform bevel). The old
/// Outline/Ghost names — shadcn semantics that all rendered identically under
/// this skin — were dropped when Plaque made the enum mean construction.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Plaque,
}

impl ButtonVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Plaque => "pl-plaque",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl ButtonSize {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
            Self::Sm => "h-8 gap-1.5 px-3 has-[>svg]:px-2.5",
            Self::Lg => "h-10 px-6 has-[>svg]:px-4",
        }
    }
}

#[component]
pub fn Button(
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    // try_get: the closure may be re-run by machinery that outlives the arena
    // signals these props wrap (e.g. a rebuilt parent reusing the node) — fall
    // back to defaults rather than panicking into a poisoned wasm runtime.
    let computed_class = move || {
        let variant = variant.try_get().unwrap_or_default();
        let size = size.try_get().unwrap_or_default();
        let class = class.try_get().unwrap_or_default();
        let mut s = String::from(BASE);
        for part in [variant.class(), size.class(), class.as_str()] {
            if !part.is_empty() {
                s.push(' ');
                s.push_str(part);
            }
        }
        s
    };

    view! {
        <button class=computed_class data-name="Button">
            {children()}
        </button>
    }
}

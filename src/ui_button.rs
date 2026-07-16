use leptos::prelude::*;
use leptos_ui::variants;

// The visual construction (face, outline border, mitered ::after bevel, pressed
// state) all lives in platinum.css keyed off the emitted data-name="Button" —
// so the utility strings here carry only what the skin does NOT own: layout,
// typography, sizing, and interaction affordances. Colors/borders/shadows/
// radii/transitions listed here would be dead on arrival (the skin !important-
// overrides them), so they're omitted.
variants! {
    Button {
        base: "inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none w-fit hover:cursor-pointer touch-manipulation [-webkit-tap-highlight-color:transparent] select-none [-webkit-touch-callout:none]", // Using hover:cursor-pointer as workaround for href_support.
        variants: {
            // Every variant renders identically under the Platinum skin; the
            // names are kept as call-site semantics (primary action vs.
            // secondary vs. quiet) in case the skin ever differentiates them.
            variant: {
                Default: "",
                Outline: "",
                Ghost: "",
            },
            size: {
                Default: "h-9 px-4 py-2 has-[>svg]:px-3",
                Sm: "h-8 gap-1.5 px-3 has-[>svg]:px-2.5",
                Lg: "h-10 px-6 has-[>svg]:px-4",
            }
        },
        component: {
            element: button,
            support_href: true,
            support_aria_current: true
        }
    }
}

//! The showcase sections, one component per sidebar category. Ported from
//! chdkpano's original /components page and regrouped: each large class of
//! controls (buttons, inputs, containers, …) gets its own page.

use leptos::ev;
use leptos::prelude::*;

use platinum_ui::debounce_button::{boxed, DebounceButton};
use platinum_ui::platinum_info::{Property, PropertyList, StatWell};
use platinum_ui::platinum_meter::Meter;
use platinum_ui::platinum_scroll::{PlatinumTextarea, ScrollWell};
use platinum_ui::platinum_select::PlatinumSelect;
use platinum_ui::ui_button;
use platinum_ui::ui_button::{Button, ButtonVariant};
use platinum_ui::ui_dialog::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};

/// Current value of the `<input>` an event fired on.
fn input_value(ev: &ev::Event) -> String {
    use wasm_bindgen::JsCast;
    let target = ev.target().expect("event target");
    let input: web_sys::HtmlInputElement = target.unchecked_into();
    input.value()
}

// ───────────────────────── Colors ─────────────────────────

#[component]
pub fn ColorsSection() -> impl IntoView {
    view! {
        <ComponentSection title="Light palette">
            <div class="flex flex-col gap-5">
                // Neutral surfaces + lines.
                <div class="flex flex-wrap gap-5">
                    <ColorSwatch label="Window" hex="#ffffff"/>
                    <ColorSwatch label="Face alt" hex="#cccccc"/>
                    <ColorSwatch label="Outline" hex="#555555"/>
                    <ColorSwatch label="Ink" hex="#000000"/>
                </div>
                // Neutral bevel family: face + its highlight/shadow edges + pressed.
                <div class="flex flex-wrap gap-5">
                    <ColorSwatch label="Face" hex="#dddddd"/>
                    <ColorSwatch label="Highlight" hex="#ffffff"/>
                    <ColorSwatch label="Shadow" hex="#808080"/>
                    <ColorSwatch label="Pressed" hex="#c6c6c2"/>
                </div>
            </div>
        </ComponentSection>

        <ComponentSection title="Dark palette (.dark)">
            <div class="flex flex-col gap-5">
                // The same roles relit for a dark room; ink inverts.
                <div class="flex flex-wrap gap-5">
                    <ColorSwatch label="Window" hex="#1e1e1e"/>
                    <ColorSwatch label="Face alt" hex="#333333"/>
                    <ColorSwatch label="Outline" hex="#000000"/>
                    <ColorSwatch label="Ink" hex="#e8e8e8"/>
                </div>
                <div class="flex flex-wrap gap-5">
                    <ColorSwatch label="Face" hex="#3e3e3e"/>
                    <ColorSwatch label="Highlight" hex="#6f6f6f"/>
                    <ColorSwatch label="Shadow" hex="#141414"/>
                    <ColorSwatch label="Pressed" hex="#2f2f2f"/>
                </div>
            </div>
        </ComponentSection>

        <ComponentSection title="Accent purple (shared by both palettes)">
            // The scroll-thumb family is deliberately identical in light and
            // dark — the one constant across the two palettes.
            <div class="flex flex-wrap gap-5">
                <ColorSwatch label="Purple" hex="#8e5ba6"/>
                <ColorSwatch label="Highlight" hex="#c1a5cf"/>
                <ColorSwatch label="Shadow" hex="#553764"/>
                <ColorSwatch label="Pressed" hex="#7a4f8f"/>
            </div>
        </ComponentSection>
    }
}

// ─────────────────────── Bevel styles ───────────────────────

#[component]
pub fn BevelsSection() -> impl IntoView {
    view! {
        <ComponentSection title="Bevel styles">
            <div class="flex flex-col gap-6">
                // Row 1: the button states.
                <div class="flex flex-wrap gap-6">
                    // Real library Button in its resting state, made inert with
                    // pointer-events:none so it stays in the raised bevel and can't
                    // be clicked/pressed.
                    <div class="flex flex-col items-center gap-2">
                        <Button
                            variant=ButtonVariant::Default
                            attr:style="width:4rem;height:4rem;pointer-events:none"
                        >{""}</Button>
                        <span class="text-xs text-muted-foreground">"Button Rest"</span>
                    </div>
                    // The same Button genuinely in its PRESSED state — aria-current is
                    // the pressed hook (inverts the ::after bevel + darkens the face),
                    // so this exercises the real state, not a hand-forced bevel. Inert.
                    <div class="flex flex-col items-center gap-2">
                        <Button
                            variant=ButtonVariant::Default
                            attr:aria-current="page"
                            attr:style="width:4rem;height:4rem;pointer-events:none"
                        >{""}</Button>
                        <span class="text-xs text-muted-foreground">"Button Pressed"</span>
                    </div>
                </div>
                // Row 2: the same two states in the plaque construction —
                // pressing inverts only the seated platform, the well stays put.
                <div class="flex flex-wrap gap-6">
                    <div class="flex flex-col items-center gap-2">
                        <Button
                            variant=ButtonVariant::Plaque
                            attr:style="width:4rem;height:4rem;pointer-events:none"
                        >{""}</Button>
                        <span class="text-xs text-muted-foreground">"Plaque Rest"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Button
                            variant=ButtonVariant::Plaque
                            attr:aria-current="page"
                            attr:style="width:4rem;height:4rem;pointer-events:none"
                        >{""}</Button>
                        <span class="text-xs text-muted-foreground">"Plaque Pressed"</span>
                    </div>
                </div>
                // Row 3: the container surfaces.
                <div class="flex flex-wrap gap-6">
                    // Raised surface + its recessed dual (both used for containers).
                    <BevelSwatch label="Platform" class="pl-platform"/>
                    <BevelSwatch label="Well" class="pl-well"/>
                    // Their composite: one element painting a platform seated in a
                    // well, the well's 1px inset ring being the single shared seam.
                    <BevelSwatch label="Plaque" class="pl-plaque"/>
                </div>
            </div>
        </ComponentSection>
    }
}

// ───────────────────────── Buttons ─────────────────────────

#[component]
pub fn ButtonsSection() -> impl IntoView {
    view! {
        <ComponentSection title="Buttons">
            <div class="flex flex-col gap-3">
                <div class="flex flex-wrap items-center gap-3">
                    <Button variant=ButtonVariant::Default size=ui_button::ButtonSize::Sm>"Small"</Button>
                    <Button variant=ButtonVariant::Default>"Default"</Button>
                    <Button variant=ButtonVariant::Default size=ui_button::ButtonSize::Lg>"Large"</Button>
                    <Button variant=ButtonVariant::Default attr:disabled=true>"Disabled"</Button>
                </div>
                // The alternate construction: a platform seated in a well.
                // Press one — the inner platform inverts, the well stays put.
                <div class="flex flex-wrap items-center gap-3">
                    <Button variant=ButtonVariant::Plaque size=ui_button::ButtonSize::Sm>"Small"</Button>
                    <Button variant=ButtonVariant::Plaque>"Default"</Button>
                    <Button variant=ButtonVariant::Plaque size=ui_button::ButtonSize::Lg>"Large"</Button>
                    <Button variant=ButtonVariant::Plaque attr:disabled=true>"Disabled"</Button>
                </div>
            </div>
        </ComponentSection>

        <ComponentSection title="Debounced button">
            <div class="flex flex-wrap items-center gap-3">
                // Default 0.2s hold — the instant action shows the minimum
                // pending time so a fast action doesn't flicker the label.
                <DebounceButton
                    label="Save"
                    pending="Saving…"
                    variant=ButtonVariant::Default
                    size=ui_button::ButtonSize::Sm
                    action=move || boxed(async {})
                />
                // Stays disabled until the (simulated 800ms) work finishes.
                <DebounceButton
                    label="Sync"
                    pending="Syncing…"
                    variant=ButtonVariant::Default
                    size=ui_button::ButtonSize::Sm
                    action=move || boxed(async {
                        gloo_timers::future::TimeoutFuture::new(800).await;
                    })
                />
            </div>
        </ComponentSection>

        <ComponentSection title="Button palette">
            <div class="flex flex-col items-start gap-4">
                <div class="flex flex-col items-center gap-2">
                    <div class="pl-palette pl-palette-icons">
                        <Button variant=ButtonVariant::Default size=ui_button::ButtonSize::Sm attr:title="Record">
                            <span class="block bg-destructive" style="width:10px;height:10px;border-radius:50%"></span>
                        </Button>
                        <Button variant=ButtonVariant::Default size=ui_button::ButtonSize::Sm attr:title="Play">
                            <span class="block" style="width:0;height:0;border-style:solid;border-width:5px 0 5px 8px;border-color:transparent transparent transparent currentColor"></span>
                        </Button>
                        <Button variant=ButtonVariant::Default size=ui_button::ButtonSize::Sm attr:title="Stop">
                            <span class="block bg-foreground" style="width:9px;height:9px"></span>
                        </Button>
                    </div>
                    <span class="text-xs text-muted-foreground">"Palette"</span>
                </div>
                // Same strip seated in a tight well: pl-palette-plaque swaps the
                // palette's outer frame ring for a well bevel + inset ring.
                <div class="flex flex-col items-center gap-2">
                    <div class="pl-palette pl-palette-icons pl-palette-plaque">
                        <Button variant=ButtonVariant::Default size=ui_button::ButtonSize::Sm attr:title="Record">
                            <span class="block bg-destructive" style="width:10px;height:10px;border-radius:50%"></span>
                        </Button>
                        <Button variant=ButtonVariant::Default size=ui_button::ButtonSize::Sm attr:title="Play">
                            <span class="block" style="width:0;height:0;border-style:solid;border-width:5px 0 5px 8px;border-color:transparent transparent transparent currentColor"></span>
                        </Button>
                        <Button variant=ButtonVariant::Default size=ui_button::ButtonSize::Sm attr:title="Stop">
                            <span class="block bg-foreground" style="width:9px;height:9px"></span>
                        </Button>
                    </div>
                    <span class="text-xs text-muted-foreground">"Plaque palette"</span>
                </div>
            </div>
        </ComponentSection>
    }
}

// ───────────────────────── Inputs ─────────────────────────

#[component]
pub fn InputsSection() -> impl IntoView {
    let (checked, set_checked) = signal(true);
    let (radio, set_radio) = signal(2);
    let (text, set_text) = signal(String::new());
    // Backing value for the popup-menu demo (tv96 shutter units).
    let demo_shutter = RwSignal::new(288);

    view! {
        <ComponentSection title="Dropdown (platinum popup menu)">
            <PlatinumSelect
                selected=demo_shutter
                options=vec![
                    ("1/8s".to_string(), 288),
                    ("1/60s".to_string(), 576),
                    ("1/250s".to_string(), 768),
                    ("1/500s".to_string(), 864),
                    ("1/1000s".to_string(), 960),
                ]
            />
        </ComponentSection>

        <ComponentSection title="Text fields">
            <div class="flex flex-col gap-3 max-w-sm">
                <input
                    class="text-sm px-3 py-2"
                    placeholder="text input"
                    prop:value=move || text.get()
                    on:input=move |ev| set_text.set(input_value(&ev))
                />
                // Plain textarea: scrolls natively, so its bar is the global
                // ::-webkit-scrollbar fallback (no arrows on macOS).
                <textarea
                    class="text-sm px-3 py-2"
                    rows="3"
                    placeholder="plain textarea (webkit-fallback bar)"
                ></textarea>
            </div>
        </ComponentSection>

        <ComponentSection title="Platinum textarea">
            <p class="text-xs text-muted-foreground mb-3 max-w-md">
                "The DOM-built bar on a text field: arrows (hold to autorepeat), "
                "draggable accent thumb, native typing and scrolling."
            </p>
            <PlatinumTextarea
                class="h-40 max-w-sm"
                placeholder="platinum textarea"
                value="Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\nLine 11\nLine 12"
            />
        </ComponentSection>

        <ComponentSection title="Checkbox">
            <label class="inline-flex items-center gap-2 cursor-pointer select-none text-sm">
                <input type="checkbox" prop:checked=move || checked.get()
                    on:change=move |_| set_checked.update(|v| *v = !*v)/>
                "Checkbox"
            </label>
        </ComponentSection>

        <ComponentSection title="Radio">
            <div class="flex flex-col items-start gap-2 text-sm">
                {[(1, "Option A"), (2, "Option B"), (3, "Option C")].into_iter().map(|(n, lbl)| view! {
                    <label class="inline-flex items-center gap-2 cursor-pointer select-none">
                        <input type="radio" name="demo-radio" prop:checked=move || radio.get() == n
                            on:change=move |_| set_radio.set(n)/>
                        {lbl}
                    </label>
                }).collect_view()}
            </div>
        </ComponentSection>
    }
}

// ──────────────────────── Containers ────────────────────────

#[component]
pub fn ContainersSection() -> impl IntoView {
    view! {
        <ComponentSection title="Framed card">
            // The plaque primitive: one element painting a raised platform
            // seated in a recessed well, sharing one 1px seam.
            <div class="pl-plaque p-4 max-w-sm">
                <h3 class="text-sm font-semibold mb-1">"Card title"</h3>
                <p class="text-xs text-muted-foreground">"A plaque: a raised platform seated in a recessed well — one shared 1px seam."</p>
            </div>
        </ComponentSection>

        <ComponentSection title="Separator">
            <div class="max-w-sm"><hr/></div>
        </ComponentSection>

        <ComponentSection title="Screen (content well)">
            <div class="pl-well pl-well-fitted w-fit">
                <div class="pl-screen bg-black relative w-40 aspect-[3/4] flex items-center justify-center">
                    <span class="text-xs text-muted-foreground">"screen well"</span>
                </div>
            </div>
        </ComponentSection>

        <ComponentSection title="Caution status">
            <div class="inline-flex flex-col items-center gap-0.5 bg-black px-8 py-5">
                <span class="text-base leading-none text-warning">"⚠"</span>
                <span class="text-xs font-medium text-warning">"No signal"</span>
                <span class="text-[10px] font-mono text-white/50">"12 failed"</span>
            </div>
        </ComponentSection>

        <ComponentSection title="Meter">
            <p class="text-xs text-muted-foreground mb-3 max-w-md">
                "A recessed level indicator; the fill drops to the destructive "
                "color at the danger threshold. Caller sizes the track."
            </p>
            <div class="flex items-center gap-6">
                <Meter fraction=0.85 danger_below=0.2 class="w-24 h-2" title="85%"/>
                <Meter fraction=0.45 danger_below=0.2 class="w-24 h-2" title="45%"/>
                <Meter fraction=0.12 danger_below=0.2 class="w-24 h-2" title="12% — danger"/>
                <Meter fraction=0.62 class="w-6 h-2" title="battery-sized"/>
            </div>
        </ComponentSection>

        <ComponentSection title="Stat wells">
            <div class="grid grid-cols-3 gap-3 max-w-md text-sm">
                <StatWell label="operations">{62}</StatWell>
                <StatWell label="events">{14}</StatWell>
                <StatWell label="props">{51}</StatWell>
            </div>
        </ComponentSection>

        <ComponentSection title="Property list">
            <div class="max-w-md">
                <PropertyList>
                    <Property label="firmware" value="1.01a".to_string() mono=true/>
                    <Property label="serial" value="C8A2FF01B4E29D77".to_string() mono=true/>
                    <Property label="model" value="Canon PowerShot ELPH 180".to_string()/>
                    <Property label="notes" value="".to_string()/>
                </PropertyList>
            </div>
        </ComponentSection>
    }
}

// ──────────────────────── Scrolling ────────────────────────

#[component]
pub fn ScrollingSection() -> impl IntoView {
    view! {
        <ComponentSection title="Scroll well">
            <div class="flex flex-wrap items-start gap-6">
                // Overflowing content: the thumb tracks it, arrows step (and
                // autorepeat when held), wheel scrolls it natively.
                <div class="flex flex-col items-center gap-2">
                    <ScrollWell class="h-40 w-80">
                        <div class="p-3 text-sm space-y-2">
                            <p class="font-semibold">"About Platinum Scrolling"</p>
                            <p>"The native scrollbar is hidden and this bar is drawn from real DOM: a raised arrow button at each end and a draggable accent thumb whose size tracks the content."</p>
                            <p>"Scrolling itself stays native — wheel, keyboard, momentum, chaining and zoom gestures all behave exactly as the platform intends."</p>
                            <p>"Hold an arrow to autorepeat, classic Mac style: one step, a beat, then a steady march."</p>
                            <p>"Content shorter than the well keeps the bar in place with a full-track thumb — see the neighbor."</p>
                        </div>
                    </ScrollWell>
                    <span class="text-xs text-muted-foreground">"Overflowing"</span>
                </div>
                // Short content: the bar keeps its place, the thumb fills the
                // whole track with nowhere to go.
                <div class="flex flex-col items-center gap-2">
                    <ScrollWell class="h-40 w-80">
                        <div class="p-3 text-sm">
                            <p>"Content fits — the thumb fills the track."</p>
                        </div>
                    </ScrollWell>
                    <span class="text-xs text-muted-foreground">"Content fits (bar stays)"</span>
                </div>
            </div>
        </ComponentSection>
    }
}

// ────────────────────────── Modal ──────────────────────────

#[component]
pub fn ModalSection() -> impl IntoView {
    view! {
        <ComponentSection title="Modal">
            <Dialog>
                <DialogTrigger variant=ButtonVariant::Default size=ui_button::ButtonSize::Sm>
                    "Open modal"
                </DialogTrigger>
                <DialogContent class="max-w-md text-left" show_close_button=false>
                    <DialogHeader>
                        <DialogTitle>"Example modal"</DialogTitle>
                        <DialogDescription>
                            "A modal window: raised bevel plus a hard-edged classic-Mac drop shadow."
                        </DialogDescription>
                    </DialogHeader>
                    <DialogFooter class="mt-6">
                        <DialogClose variant=ButtonVariant::Default size=ui_button::ButtonSize::Default>
                            "Close"
                        </DialogClose>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </ComponentSection>
    }
}

// ──────────────────────── Typography ────────────────────────

#[component]
pub fn TypographySection() -> impl IntoView {
    view! {
        <ComponentSection title="Font families">
            <div class="flex flex-col gap-5 max-w-2xl">
                <div>
                    <div class="text-xs text-muted-foreground font-mono mb-1">"--font-sans (Geneva)"</div>
                    <p class="text-base">"The quick brown fox jumps over the lazy dog — 0123456789"</p>
                </div>
                // Forced to the bundled face so the fallback is visible even on
                // a Mac, where the stack would otherwise resolve to Geneva.
                <div>
                    <div class="text-xs text-muted-foreground font-mono mb-1">"--font-sans fallback (Arimo, bundled)"</div>
                    <p class="text-base" style="font-family:Arimo">"The quick brown fox jumps over the lazy dog — 0123456789"</p>
                </div>
                <div>
                    <div class="text-xs text-muted-foreground font-mono mb-1">"--font-mono (Monaco)"</div>
                    <p class="text-base font-mono">"The quick brown fox jumps over the lazy dog — 0123456789"</p>
                </div>
                <div>
                    <div class="text-xs text-muted-foreground font-mono mb-1">"--font-mono fallback (Cousine, bundled)"</div>
                    <p class="text-base" style="font-family:Cousine">"The quick brown fox jumps over the lazy dog — 0123456789"</p>
                </div>
            </div>
        </ComponentSection>

        <ComponentSection title="Header sizes">
            <div class="flex flex-col gap-5">
                <div>
                    <div class="text-xs text-muted-foreground font-mono mb-1">"h1"</div>
                    <h1 class="text-lg font-semibold tracking-tight">"platinum-ui"</h1>
                </div>
                <div>
                    <div class="text-xs text-muted-foreground font-mono mb-1">"h2"</div>
                    <h2 class="text-base font-semibold">"Section heading"</h2>
                </div>
                <div>
                    <div class="text-xs text-muted-foreground font-mono mb-1">"h3"</div>
                    <h3 class="text-xs uppercase tracking-wide text-muted-foreground">"Bevel styles"</h3>
                </div>
            </div>
        </ComponentSection>

        <ComponentSection title="Body sizes">
            <div class="flex flex-col gap-5 max-w-2xl">
                <div>
                    <div class="text-xs text-muted-foreground font-mono mb-1">"text-sm"</div>
                    <p class="text-sm">"Point the rig, half-press to wake the cameras, then hit Capture."</p>
                </div>
                <div>
                    <div class="text-xs text-muted-foreground font-mono mb-1">"text-xs"</div>
                    <p class="text-xs">"Point the rig, half-press to wake the cameras, then hit Capture."</p>
                </div>
                <div>
                    <div class="text-xs text-muted-foreground font-mono mb-1">"text-[10px]"</div>
                    <p class="text-[10px] font-mono">"12 failed · skew 3.2 ms · 0/4 fired"</p>
                </div>
            </div>
        </ComponentSection>
    }
}

// ───────────────────────── Helpers ─────────────────────────

#[component]
pub fn ComponentSection(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="mb-8">
            <h3 class="text-xs text-muted-foreground uppercase tracking-wide mb-3">{title}</h3>
            {children()}
        </section>
    }
}

#[component]
pub fn BevelSwatch(label: &'static str, class: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-2">
            <div class=class style="width:4rem;height:4rem"></div>
            <span class="text-xs text-muted-foreground">{label}</span>
        </div>
    }
}

/// One entry in the Colors section: a black-framed swatch + role label + hex.
#[component]
pub fn ColorSwatch(label: &'static str, hex: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-1">
            <div style=format!(
                "width:3.5rem;height:3.5rem;background-color:{hex};border:1px solid #000"
            )></div>
            <span class="text-xs">{label}</span>
            <span class="text-[10px] text-muted-foreground font-mono">{hex}</span>
        </div>
    }
}


//! Switch — toggle switch with label, sizes sm/md/lg, disabled state.

use leptos::prelude::*;

/// Switch size variant.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SwitchSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// A styled toggle switch with optional label.
///
/// Uses DUI CSS classes: `.dm-switch`, `.dm-switch-track-*`, `.dm-switch-thumb-*`, `.dm-switch-checked`.
/// No Tailwind required.
#[component]
pub fn Switch(
    /// Reactive checked state (two-way binding).
    checked: RwSignal<bool>,
    /// Disabled state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Size of the switch.
    #[prop(default = SwitchSize::Md)]
    size: SwitchSize,
    /// Optional label displayed to the right of the switch.
    #[prop(optional)]
    label: Option<&'static str>,
    /// Extra CSS classes on the outer wrapper.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let toggle = move || {
        if !disabled.get() {
            checked.set(!checked.get());
        }
    };

    let track_class = match size {
        SwitchSize::Sm => "dm-switch-track-sm",
        SwitchSize::Md => "dm-switch-track-md",
        SwitchSize::Lg => "dm-switch-track-lg",
    };

    let thumb_class = match size {
        SwitchSize::Sm => "dm-switch-thumb dm-switch-thumb-sm",
        SwitchSize::Md => "dm-switch-thumb dm-switch-thumb-md",
        SwitchSize::Lg => "dm-switch-thumb dm-switch-thumb-lg",
    };

    view! {
        <div class=format!("dm-switch {}", class)>
            <button
                type="button"
                role="switch"
                aria-checked=move || if checked.get() { "true" } else { "false" }
                aria-disabled=move || if disabled.get() { "true" } else { "false" }
                aria-label=label.unwrap_or("Toggle")
                class=move || format!(
                    "dm-switch-track {} {} {}",
                    track_class,
                    if checked.get() { "dm-switch-checked" } else { "" },
                    if disabled.get() { "dm-opacity-40 dm-cursor-not-allowed" } else { "dm-cursor-pointer" },
                )
                disabled=disabled
                on:click=move |_| toggle()
                on:keydown=move |ev: web_sys::KeyboardEvent| {
                    let key = ev.key();
                    if key == " " || key == "Enter" {
                        ev.prevent_default();
                        toggle();
                    }
                }
            >
                <span class=move || format!(
                    "{} {}",
                    thumb_class,
                    if checked.get() { "dm-switch-checked" } else { "" },
                )></span>
            </button>

            {label.map(|l| view! {
                <span
                    class=move || format!(
                        "dm-text-sm dm-text-primary {}",
                        if disabled.get() { "dm-opacity-40" } else { "dm-cursor-pointer" },
                    )
                    on:click=move |_| toggle()
                >
                    {l}
                </span>
            })}
        </div>
    }
}

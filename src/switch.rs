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
/// Click or press Space/Enter to toggle. Supports three sizes and a disabled state.
///
/// # Example
/// ```rust
/// let enabled = RwSignal::new(false);
/// view! { <Switch checked=enabled label="Enable notifications" /> }
/// ```
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

    let (track_classes, thumb_size, thumb_translate) = match size {
        SwitchSize::Sm => ("w-8 h-4", "width:14px;height:14px;", "translateX(15px)"),
        SwitchSize::Md => ("w-10 h-5", "width:18px;height:18px;", "translateX(19px)"),
        SwitchSize::Lg => ("w-12 h-6", "width:22px;height:22px;", "translateX(23px)"),
    };

    view! {
        <div
            class=format!(
                "inline-flex items-center gap-2.5 select-none {}",
                class,
            )
        >
            // Track + Thumb
            <button
                type="button"
                role="switch"
                aria-checked=move || if checked.get() { "true" } else { "false" }
                aria-disabled=move || if disabled.get() { "true" } else { "false" }
                aria-label=label.unwrap_or("Toggle")
                class=move || format!(
                    "relative inline-flex items-center shrink-0 \
                     rounded-full transition-colors duration-150 \
                     dm-focus-ring cursor-pointer \
                     {} {} {}",
                    track_classes,
                    if checked.get() { "bg-dm-accent" } else { "bg-dm-elevated" },
                    if disabled.get() { "opacity-50 cursor-not-allowed" } else { "" },
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
                // Thumb circle
                <span
                    class="absolute left-[1px] top-1/2 rounded-full bg-white shadow-sm transition-transform duration-150"
                    style=move || format!(
                        "{} transform: {} translateY(-50%);",
                        thumb_size,
                        if checked.get() { thumb_translate } else { "translateX(0)" },
                    )
                ></span>
            </button>

            // Label
            {label.map(|l| view! {
                <span
                    class=move || format!(
                        "text-sm text-dm-text {}",
                        if disabled.get() { "opacity-50 cursor-not-allowed" } else { "cursor-pointer" },
                    )
                    on:click=move |_| toggle()
                >
                    {l}
                </span>
            })}
        </div>
    }
}

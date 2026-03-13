//! Checkbox — custom styled checkbox with label and description.

use leptos::prelude::*;

/// A custom-styled checkbox with optional label and description text.
///
/// Uses an SVG checkmark instead of the native checkbox for consistent styling.
/// Click anywhere on the row (checkbox, label, or description) to toggle.
///
/// # Example
/// ```rust
/// let agreed = RwSignal::new(false);
/// view! { <Checkbox checked=agreed label="I agree to the terms" /> }
/// ```
#[component]
pub fn Checkbox(
    /// Reactive checked state (two-way binding).
    checked: RwSignal<bool>,
    /// Disabled state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Label displayed to the right of the checkbox.
    #[prop(optional)]
    label: Option<&'static str>,
    /// Description displayed below the label in smaller muted text.
    #[prop(optional)]
    description: Option<&'static str>,
    /// Extra CSS classes on the outer wrapper.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let toggle = move || {
        if !disabled.get() {
            checked.set(!checked.get());
        }
    };

    let label_id = label
        .map(|l| format!("cb-{}", l.to_lowercase().replace(' ', "-")))
        .unwrap_or_else(|| "checkbox-label".to_string());
    let label_id_clone = label_id.clone();

    view! {
        <div
            class=format!(
                "inline-flex items-start gap-2.5 select-none {}",
                class,
            )
            on:click=move |_| toggle()
        >
            // Custom checkbox box
            <span
                role="checkbox"
                aria-checked=move || if checked.get() { "true" } else { "false" }
                aria-disabled=move || if disabled.get() { "true" } else { "false" }
                aria-labelledby=label_id
                tabindex="0"
                class=move || format!(
                    "shrink-0 inline-flex items-center justify-center \
                     w-[18px] h-[18px] rounded-[4px] border-2 \
                     transition-all duration-150 \
                     dm-focus-ring cursor-pointer \
                     {} {}",
                    if checked.get() {
                        "bg-dm-accent border-dm-accent"
                    } else {
                        "bg-dm-panel border-dm hover:border-dm-strong"
                    },
                    if disabled.get() { "opacity-50 cursor-not-allowed" } else { "" },
                )
                on:keydown=move |ev: web_sys::KeyboardEvent| {
                    if ev.key() == " " {
                        ev.prevent_default();
                        toggle();
                    }
                }
            >
                // Checkmark SVG — only visible when checked
                <Show when=move || checked.get()>
                    <svg
                        class="w-3 h-3 text-white"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="3"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M4.5 12.75l6 6 9-13.5"
                        />
                    </svg>
                </Show>
            </span>

            // Label + Description
            {(label.is_some() || description.is_some()).then(|| view! {
                <div class="flex flex-col gap-0.5">
                    {label.map(|l| view! {
                        <span
                            id=label_id_clone.clone()
                            class=move || format!(
                                "text-sm text-dm-text leading-tight {}",
                                if disabled.get() { "opacity-50" } else { "cursor-pointer" },
                            )
                        >
                            {l}
                        </span>
                    })}
                    {description.map(|d| view! {
                        <span
                            class=move || format!(
                                "text-xs text-dm-dim leading-snug {}",
                                if disabled.get() { "opacity-50" } else { "cursor-pointer" },
                            )
                        >
                            {d}
                        </span>
                    })}
                </div>
            })}
        </div>
    }
}

//! Checkbox — custom styled checkbox with label and description.

use leptos::prelude::*;

/// A custom-styled checkbox with optional label and description text.
///
/// Uses DUI CSS classes: `.dm-checkbox`, `.dm-checkbox-box`, `.dm-checkbox-checked`, etc.
/// No Tailwind required.
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
            class=move || format!(
                "dm-checkbox {} {}",
                if disabled.get() { "dm-checkbox-disabled" } else { "" },
                class,
            )
            on:click=move |_| toggle()
        >
            <span
                role="checkbox"
                aria-checked=move || if checked.get() { "true" } else { "false" }
                aria-disabled=move || if disabled.get() { "true" } else { "false" }
                aria-labelledby=label_id
                tabindex="0"
                class=move || format!(
                    "dm-checkbox-box {}",
                    if checked.get() { "dm-checkbox-checked" } else { "" },
                )
                on:keydown=move |ev: web_sys::KeyboardEvent| {
                    if ev.key() == " " {
                        ev.prevent_default();
                        toggle();
                    }
                }
            >
                <Show when=move || checked.get()>
                    <svg
                        style="width:12px;height:12px;color:#fff"
                        xmlns="http://www.w3.org/2000/svg" fill="none"
                        viewBox="0 0 24 24" stroke-width="3" stroke="currentColor"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                    </svg>
                </Show>
            </span>

            {(label.is_some() || description.is_some()).then(|| view! {
                <div>
                    {label.map(|l| view! {
                        <span id=label_id_clone.clone() class="dm-checkbox-label">{l}</span>
                    })}
                    {description.map(|d| view! {
                        <span class="dm-checkbox-desc">{d}</span>
                    })}
                </div>
            })}
        </div>
    }
}

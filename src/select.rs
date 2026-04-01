//! Select — dropdown with options and placeholder.

use leptos::prelude::*;

/// A single option in the dropdown.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

/// A styled dropdown select with optional placeholder.
///
/// Uses DUI CSS classes: `.dm-input` (shared with Input), `.dm-input-label`.
/// No Tailwind required.
#[component]
pub fn Select(
    /// Available options.
    options: Vec<SelectOption>,
    /// Currently selected value.
    #[prop(into)]
    value: RwSignal<String>,
    /// Placeholder shown when nothing is selected.
    #[prop(default = "Select an option")]
    placeholder: &'static str,
    /// Label above the select.
    #[prop(optional)]
    label: Option<&'static str>,
    /// Disabled state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div class="dm-flex dm-flex-col dm-gap-2 dm-w-full">
            {label.map(|l| view! {
                <label class="dm-input-label">{l}</label>
            })}
            <div class="dm-relative">
                <select
                    class=format!("dm-input dm-cursor-pointer {}", class)
                    style="appearance:none;padding-right:40px"
                    disabled=disabled
                    on:change=move |ev| { value.set(event_target_value(&ev)); }
                    prop:value=move || value.get()
                >
                    <option value="" disabled=true selected=move || value.get().is_empty()>
                        {placeholder}
                    </option>
                    {options.into_iter().map(|opt| {
                        let val = opt.value.clone();
                        let val2 = opt.value.clone();
                        view! {
                            <option value=val selected=move || value.get() == val2>
                                {opt.label}
                            </option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
                <svg
                    class="dm-input-icon dm-input-icon-right"
                    style="width:16px;height:16px"
                    xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                    stroke-width="2" stroke="currentColor"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
                </svg>
            </div>
        </div>
    }
}

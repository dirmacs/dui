//! Select — dropdown with options and placeholder.

use leptos::prelude::*;

/// A single option in the dropdown.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

/// A styled dropdown select with optional placeholder.
#[component]
pub fn Select(
    /// Available options.
    options: Vec<SelectOption>,
    /// Currently selected value (empty string = nothing selected).
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
        <div class="flex flex-col gap-1.5 w-full">
            {label.map(|l| view! {
                <label class="text-sm font-medium text-dm-muted">{l}</label>
            })}

            <div class="relative">
                <select
                    class=format!(
                        "w-full bg-dm-panel text-dm-text text-sm appearance-none \
                         border border-dm hover:border-dm-strong rounded-lg px-3 py-2.5 pr-10 \
                         transition-all duration-150 \
                         focus:outline-none focus:border-dm-accent \
                         focus:shadow-[0_0_0_3px_rgba(79,124,255,0.15)] \
                         disabled:opacity-50 disabled:cursor-not-allowed \
                         cursor-pointer {}",
                        class
                    )
                    disabled=disabled
                    on:change=move |ev| {
                        value.set(event_target_value(&ev));
                    }
                    prop:value=move || value.get()
                >
                    <option value="" disabled=true selected=move || value.get().is_empty()>
                        {placeholder}
                    </option>
                    {options.into_iter().map(|opt| {
                        let val = opt.value.clone();
                        let val2 = opt.value.clone();
                        view! {
                            <option
                                value=val
                                selected=move || value.get() == val2
                            >
                                {opt.label}
                            </option>
                        }
                    }).collect::<Vec<_>>()}
                </select>

                // Chevron icon
                <svg
                    class="absolute right-3 top-1/2 -translate-y-1/2 w-4 h-4 text-dm-dim pointer-events-none"
                    xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                    stroke-width="2" stroke="currentColor"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
                </svg>
            </div>
        </div>
    }
}

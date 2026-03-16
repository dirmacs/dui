//! Input — text / password / search with label, error message, focus ring.

use leptos::prelude::*;

/// Input field type.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum InputType {
 #[default]
 Text,
 Password,
 Search,
 Email, // NEW — adds type="email" for browser email validation
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Text     => "text",
            InputType::Password => "password",
            InputType::Search   => "search",
            InputType::Email    => "email",
        }
    }
}
}

/// A styled text input with optional label and error message.
///
/// Focus ring uses the accent color. Error state turns the border red.
#[component]
pub fn Input(
    /// Input type (text, password, search).
    #[prop(default = InputType::Text)]
    input_type: InputType,
    /// Placeholder text.
    #[prop(default = "")]
    placeholder: &'static str,
    /// Label displayed above the input.
    #[prop(optional)]
    label: Option<&'static str>,
    /// Reactive value binding.
    #[prop(into)]
    value: RwSignal<String>,
    /// Error message — when non-empty, shows error styling.
    #[prop(optional, into)]
    error: Signal<String>,
    /// Disabled state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Extra CSS classes on the input element.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let has_error = move || !error.get().is_empty();

    let search_icon = matches!(input_type, InputType::Search);

    view! {
        <div class="flex flex-col gap-1.5 w-full">
            // Label
            {label.map(|l| view! {
                <label class="text-sm font-medium text-dm-muted">{l}</label>
            })}

            // Input wrapper (for search icon)
            <div class="relative">
                {search_icon.then(|| view! {
                    <svg
                        class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-dm-dim pointer-events-none"
                        xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                        stroke-width="2" stroke="currentColor"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round"
                              d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
                    </svg>
                })}

                <input
                    type=input_type.as_str()
                    placeholder=placeholder
                    disabled=disabled
                    class=move || format!(
                        "w-full bg-dm-panel text-dm-text text-sm \
                         border rounded-lg px-3 py-2.5 \
                         placeholder:text-dm-dim \
                         transition-all duration-150 \
                         focus:outline-none focus:border-dm-accent \
                         focus:shadow-[0_0_0_3px_rgba(79,124,255,0.15)] \
                         disabled:opacity-50 disabled:cursor-not-allowed \
                         {} {} {}",
                        if has_error() { "border-dm-danger focus:border-dm-danger focus:shadow-[0_0_0_3px_rgba(248,113,113,0.15)]" }
                        else { "border-dm hover:border-dm-strong" },
                        if search_icon { "pl-10" } else { "" },
                        class,
                    )
                    prop:value=move || value.get()
                    on:input=move |ev| {
                        value.set(event_target_value(&ev));
                    }
                />
            </div>

            // Error message
            <Show when=has_error>
                <p class="text-xs text-dm-danger flex items-center gap-1">
                    <svg class="w-3.5 h-3.5 shrink-0" xmlns="http://www.w3.org/2000/svg" fill="none"
                         viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round"
                              d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z" />
                    </svg>
                    {move || error.get()}
                </p>
            </Show>
        </div>
    }
}

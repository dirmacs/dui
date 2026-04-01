//! Input — text / password / search / email with label, error message, focus ring.

use leptos::prelude::*;

/// Input field type.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Search,
    Email,
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

/// A styled text input with optional label and error message.
///
/// Uses DUI CSS classes: `.dm-input`, `.dm-input-label`, `.dm-input-error`, etc.
/// No Tailwind required.
#[component]
pub fn Input(
    /// Input type (text, password, search, email).
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
        <div class="dm-flex dm-flex-col dm-gap-2 dm-w-full">
            {label.map(|l| view! {
                <label class="dm-input-label">{l}</label>
            })}

            <div class="dm-input-wrapper">
                {search_icon.then(|| view! {
                    <svg
                        class="dm-input-icon dm-input-icon-left"
                        style="width:16px;height:16px"
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
                        "dm-input {} {} {}",
                        if has_error() { "dm-input-error" } else { "" },
                        if search_icon { "dm-input-with-icon-left" } else { "" },
                        if disabled.get() { "dm-input-disabled" } else { "" },
                    )
                    prop:value=move || value.get()
                    on:input=move |ev| {
                        value.set(event_target_value(&ev));
                    }
                />
            </div>

            <Show when=has_error>
                <p class="dm-input-error-text">
                    <svg style="width:14px;height:14px;flex-shrink:0" xmlns="http://www.w3.org/2000/svg" fill="none"
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

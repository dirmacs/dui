//! Textarea — multi-line text input with label, error, character count.

use leptos::prelude::*;

/// A styled multi-line text input with optional label, error message,
/// and character count.
///
/// Uses DUI CSS classes: `.dm-textarea`, `.dm-input-label`, `.dm-input-error-text`.
/// No Tailwind required.
#[component]
pub fn Textarea(
    /// Reactive value binding.
    #[prop(into)]
    value: RwSignal<String>,
    /// Placeholder text.
    #[prop(default = "")]
    placeholder: &'static str,
    /// Label displayed above the textarea.
    #[prop(optional)]
    label: Option<&'static str>,
    /// Error message — when non-empty, shows error styling.
    #[prop(optional, into)]
    error: Signal<String>,
    /// Disabled state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Number of visible text rows.
    #[prop(default = 4)]
    rows: u32,
    /// Maximum character length. When set, a counter is displayed.
    #[prop(optional)]
    max_length: Option<u32>,
    /// Whether vertical resizing is allowed.
    #[prop(default = true)]
    resize: bool,
    /// Extra CSS classes on the textarea element.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let has_error = move || !error.get().is_empty();
    let resize_style = if resize { "resize:vertical" } else { "resize:none" };

    let error_id: &'static str = match label {
        Some(l) => Box::leak(format!("{}-error", l.to_lowercase().replace(' ', "-")).into_boxed_str()),
        None    => "textarea-error",
    };

    view! {
        <div class="dm-flex dm-flex-col dm-gap-2 dm-w-full">
            {label.map(|l| view! {
                <label class="dm-input-label">{l}</label>
            })}

            <div class="dm-relative">
                <textarea
                    placeholder=placeholder
                    disabled=disabled
                    rows=rows
                    aria-invalid=move || if has_error() { "true" } else { "false" }
                    aria-describedby=move || if has_error() { error_id } else { "" }
                    class=move || format!(
                        "dm-textarea {} {} {}",
                        if has_error() { "dm-input-error" } else { "" },
                        if disabled.get() { "dm-input-disabled" } else { "" },
                        class,
                    )
                    style=resize_style
                    prop:value=move || value.get()
                    on:input=move |ev| {
                        let mut v = event_target_value(&ev);
                        if let Some(max) = max_length {
                            if v.len() > max as usize {
                                v.truncate(max as usize);
                            }
                        }
                        value.set(v);
                    }
                ></textarea>

                {max_length.map(|max| view! {
                    <span class="dm-textarea-counter">
                        {move || format!("{}/{}", value.get().len(), max)}
                    </span>
                })}
            </div>

            <Show when=has_error>
                <p id=error_id class="dm-input-error-text">
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

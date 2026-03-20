//! Textarea — multi-line text input with label, error, character count.

use leptos::prelude::*;

/// A styled multi-line text input with optional label, error message,
/// and character count.
///
/// Follows the same styling conventions as the [`Input`](crate::input::Input)
/// component. Supports resize control and max-length enforcement.
///
/// # Example
/// ```rust
/// let bio = RwSignal::new(String::new());
/// view! { <Textarea value=bio label="Bio" placeholder="Tell us about yourself" max_length=Some(500) /> }
/// ```
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
    let resize_class = if resize { "resize-y" } else { "resize-none" };

    // Leak a &'static str for the error id so it can be used in Fn closures
    let error_id: &'static str = match label {
        Some(l) => Box::leak(format!("{}-error", l.to_lowercase().replace(' ', "-")).into_boxed_str()),
        None    => "textarea-error",
    };

    view! {
        <div class="flex flex-col gap-1.5 w-full">
            // Label
            {label.map(|l| view! {
                <label class="font-mono text-[11px] font-medium uppercase tracking-[0.05em] text-[var(--dm-text-secondary)]">{l}</label>
            })}

            // Textarea
            <div class="relative">
                <textarea
                    placeholder=placeholder
                    disabled=disabled
                    rows=rows
                    aria-invalid=move || if has_error() { "true" } else { "false" }
                    aria-describedby=move || if has_error() { error_id } else { "" }
                    class=move || format!(
                        "w-full bg-[var(--dm-surface)] text-[var(--dm-text)] text-sm \
                         border-2 border-[var(--dm-border)] rounded-md px-3 py-2.5 \
                         placeholder:text-[var(--dm-text-dim)] \
                         transition-all duration-150 \
                         focus:outline-none focus:border-[var(--dm-accent)] \
                         focus:shadow-[0_0_0_3px_rgba(79,124,255,0.15)] \
                         disabled:opacity-50 disabled:cursor-not-allowed \
                         {} {} {}",
                        if has_error() {
                            "border-[var(--dm-unknown)] focus:border-[var(--dm-unknown)] focus:shadow-[0_0_0_3px_rgba(248,113,113,0.15)]"
                        } else {
                            "border-[var(--dm-border)] hover:border-[var(--dm-border-hover)]"
                        },
                        resize_class,
                        class,
                    )
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

                // Character count
                {max_length.map(|max| view! {
                    <span class="absolute bottom-2 right-3 text-xs text-[var(--dm-text-dim)] pointer-events-none select-none">
                        {move || format!("{}/{}", value.get().len(), max)}
                    </span>
                })}
            </div>

            // Error message
            <Show when=has_error>
                <p
                    id=error_id
                    class="text-xs text-[var(--dm-unknown-text)] flex items-center gap-1"
                >
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

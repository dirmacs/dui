use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Chat input with textarea and send button.
///
/// Enter to send, Shift+Enter for newline. Send button shows loading state.
#[component]
pub fn ChatInput(
    /// Reactive message value (two-way binding).
    value: RwSignal<String>,
    /// Called when user sends a message.
    on_send: Box<dyn Fn(String) + 'static>,
    /// Placeholder text.
    #[prop(default = "Type a message...")]
    placeholder: &'static str,
    /// Loading state (disables input during send).
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let on_send = std::rc::Rc::new(on_send);
    let on_send_click = on_send.clone();
    let on_send_key = on_send.clone();

    view! {
        <div class=format!("flex items-end gap-2 p-2 border-t border-[var(--dm-border)] {}", class)>
            <textarea
                rows="1"
                placeholder=placeholder
                disabled=move || loading.get()
                class="flex-1 resize-none rounded-md px-3 py-2 text-sm bg-[var(--dm-surface)] text-[var(--dm-text)] border-2 border-[var(--dm-border)] focus:outline-none focus:ring-2 focus:ring-[var(--dm-accent)] disabled:opacity-50"
                prop:value=move || value.get()
                on:input=move |ev| {
                    let target = ev.target().unwrap();
                    let input: web_sys::HtmlTextAreaElement = target.unchecked_into();
                    value.set(input.value());
                }
                on:keydown=move |ev: web_sys::KeyboardEvent| {
                    if ev.key() == "Enter" && !ev.shift_key() {
                        ev.prevent_default();
                        let msg = value.get().trim().to_string();
                        if !msg.is_empty() && !loading.get() {
                            (on_send_key)(msg);
                            value.set(String::new());
                        }
                    }
                }
            />
            <button
                type="button"
                disabled=move || loading.get() || value.get().trim().is_empty()
                class="px-4 py-2 rounded-lg text-sm font-medium bg-[var(--dm-accent)] text-white hover:opacity-90 transition-opacity disabled:opacity-50 disabled:cursor-not-allowed min-h-[40px]"
                on:click=move |_| {
                    let msg = value.get().trim().to_string();
                    if !msg.is_empty() && !loading.get() {
                        (on_send_click)(msg);
                        value.set(String::new());
                    }
                }
            >
                {move || if loading.get() { "..." } else { "Send" }}
            </button>
        </div>
    }
}

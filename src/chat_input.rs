//! ChatInput — chat input with textarea and send button.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Chat input with textarea and send button.
///
/// Uses DUI CSS classes: `.dm-textarea`, `.dm-btn`, `.dm-btn-primary`.
/// No Tailwind required.
#[component]
pub fn ChatInput(
    /// Reactive message value.
    value: RwSignal<String>,
    /// Called when user sends a message.
    on_send: Box<dyn Fn(String) + 'static>,
    /// Placeholder text.
    #[prop(default = "Type a message...")]
    placeholder: &'static str,
    /// Loading state.
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
        <div class=format!("dm-flex dm-items-end dm-gap-2 dm-p-2 dm-border-t {}", class)>
            <textarea
                rows="1"
                placeholder=placeholder
                disabled=move || loading.get()
                class="dm-textarea dm-flex-1"
                style="resize:none;min-height:40px"
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
                class="dm-btn dm-btn-primary"
                style="min-height:40px"
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

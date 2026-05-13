//! Modal — overlay dialog with backdrop, close-on-escape, and close-on-click-outside.

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::{prelude::*, JsCast};

/// A centered modal overlay with backdrop, title bar, and close triggers.
///
/// Uses DUI CSS classes: `.dm-modal-backdrop`, `.dm-modal`, `.dm-modal-header`, `.dm-modal-title`, `.dm-modal-close`.
/// No Tailwind required.
#[component]
pub fn Modal(
    /// Controls visibility.
    open: RwSignal<bool>,
    /// Header text (empty = no header).
    #[prop(default = "")]
    title: &'static str,
    /// Max width for the panel (CSS value, e.g. "32rem").
    #[prop(default = "32rem")]
    max_width: &'static str,
    /// Modal body content.
    children: Children,
) -> impl IntoView {
    Effect::new(move |_| {
        if !open.get() {
            return;
        }
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" { open.set(false); }
        });
        let _ = window.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        let window = SendWrapper::new(window);
        let cb = SendWrapper::new(cb);
        on_cleanup(move || {
            let _ = window.remove_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        });
    });

    let has_title = !title.is_empty();
    let body = children();

    view! {
        <div
            class=move || if open.get() { "dm-modal-backdrop" } else { "dm-hidden" }
            on:mousedown=move |ev| {
                use wasm_bindgen::JsCast;
                if let Some(target) = ev.target() {
                    if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                        // Close if clicking the backdrop itself
                        if el.class_list().contains("dm-modal-backdrop") {
                            open.set(false);
                        }
                    }
                }
            }
        >
            <div
                role="dialog"
                aria-modal="true"
                aria-label=title
                class="dm-modal"
                style=format!("max-width:{}", max_width)
            >
                {has_title.then(|| view! {
                    <div class="dm-modal-header">
                        <h2 class="dm-modal-title">{title}</h2>
                        <button
                            on:click=move |_| open.set(false)
                            aria-label="Close dialog"
                            class="dm-modal-close"
                        >
                            <svg style="width:18px;height:18px" xmlns="http://www.w3.org/2000/svg" fill="none"
                                 viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                })}
                <div class="dm-p-6">
                    {body}
                </div>
            </div>
        </div>
    }
}

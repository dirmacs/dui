//! Modal — overlay dialog with backdrop, close-on-escape, and close-on-click-outside.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// A centered modal overlay with backdrop, title bar, and close triggers.
///
/// Close triggers: X button, click outside the panel, Escape key.
/// Children are rendered once and visibility is toggled via CSS.
#[component]
pub fn Modal(
    /// Controls visibility (writable so the modal can close itself).
    open: RwSignal<bool>,
    /// Header text (empty = no header).
    #[prop(default = "")]
    title: &'static str,
    /// Tailwind max-width class for the panel.
    #[prop(default = "max-w-lg")]
    max_width: &'static str,
    /// Modal body content.
    children: Children,
) -> impl IntoView {
    // Escape key listener (runs once, no tracked signals)
    Effect::new(move |_| {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" {
                open.set(false);
            }
        });
        let _ = window.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        // Modal uses CSS visibility toggling, so the escape listener is harmless when hidden.
        // forget() keeps the closure alive for the page lifetime.
        cb.forget();
    });

    let has_title = !title.is_empty();
    let body = children();

    view! {
        <div
            class=move || {
                if open.get() {
                    "fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm"
                } else {
                    "hidden"
                }
            }
            style="background: rgba(0,0,0,0.4);"
            on:mousedown=move |ev| {
                use wasm_bindgen::JsCast;
                if let Some(target) = ev.target() {
                    if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                        if el.class_list().contains("fixed") {
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
                class=format!(
                    "bg-[var(--dm-surface)] border-2 border-[var(--dm-border)] rounded-lg shadow-[var(--dm-shadow-xl)] w-full {}",
                    max_width
                )
            >
                // Title bar
                {has_title.then(|| view! {
                    <div class="flex items-center justify-between px-6 py-4 border-b-2 border-[var(--dm-border)]">
                        <h2 class="text-lg font-semibold text-[var(--dm-text)]">{title}</h2>
                        <button
                            on:click=move |_| open.set(false)
                            aria-label="Close dialog"
                            class="p-1 rounded-md text-[var(--dm-text-secondary)] hover:text-[var(--dm-text)] hover:bg-[var(--dm-surface-hover)] transition-colors dm-focus-ring"
                        >
                            <svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" fill="none"
                                 viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                })}

                // Body
                <div class="px-6 py-5">
                    {body}
                </div>
            </div>
        </div>
    }
}

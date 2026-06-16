//! Sheet — a slide-out panel from any edge.

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::{prelude::*, JsCast};

/// Which edge the sheet slides in from.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SheetSide {
    #[default]
    Right,
    Left,
}

/// A slide-out panel that overlays the page from a specified edge.
///
/// Uses DUI CSS classes: `.dm-sheet-backdrop`, `.dm-sheet`, `.dm-sheet-right/left`, `.dm-sheet-header`.
/// No Tailwind required.
#[component]
pub fn Sheet(
    /// Controls visibility.
    open: RwSignal<bool>,
    /// Which side the panel slides in from.
    #[prop(default = SheetSide::Right)]
    side: SheetSide,
    /// Optional title.
    #[prop(default = "")]
    title: &'static str,
    /// Panel width (CSS value, e.g. "400px").
    #[prop(default = "400px")]
    width: &'static str,
    /// Panel body content.
    children: Children,
) -> impl IntoView {
    Effect::new(move |_| {
        if !open.try_get().unwrap_or(false) {
            return;
        }
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let cb =
            Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
                if ev.key() == "Escape" {
                    open.set(false);
                }
            });
        let _ = window.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        let window = SendWrapper::new(window);
        let cb = SendWrapper::new(cb);
        on_cleanup(move || {
            let _ =
                window.remove_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        });
    });

    let has_title = !title.is_empty();

    let side_class = match side {
        SheetSide::Right => "dm-sheet-right",
        SheetSide::Left => "dm-sheet-left",
    };

    view! {
        // Backdrop
        <div
            class=move || if open.try_get().unwrap_or(false) { "dm-sheet-backdrop" } else { "dm-hidden" }
            on:click=move |_| open.set(false)
        />
        // Panel
        <div
            class=move || format!("dm-sheet {} {}", side_class, if open.try_get().unwrap_or(false) { "" } else { "dm-hidden" })
            style=format!("width:{};max-width:90vw", width)
        >
            {has_title.then(|| view! {
                <div class="dm-sheet-header">
                    <span class="dm-font-mono dm-font-semibold dm-text-primary" style="font-size:16px">{title}</span>
                    <button
                        class="dm-modal-close"
                        on:click=move |_| open.set(false)
                        aria-label="Close"
                    >
                        <svg style="width:18px;height:18px" xmlns="http://www.w3.org/2000/svg" fill="none"
                             viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
            })}
            <div class="dm-sheet-body">
                {children()}
            </div>
        </div>
    }
}

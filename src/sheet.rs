//! Sheet — a slide-out panel from any edge (like Shadcn Sheet).

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Which edge the sheet slides in from.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SheetSide {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}

/// A slide-out panel that overlays the page from a specified edge.
///
/// Similar to a modal but anchored to one side of the viewport.
/// Close triggers: X button, Escape key, click backdrop.
/// Uses CSS visibility toggling — children are rendered once.
#[component]
pub fn Sheet(
    /// Controls visibility (writable so the sheet can close itself).
    open: RwSignal<bool>,
    /// Which side the panel slides in from.
    #[prop(default = SheetSide::Right)]
    side: SheetSide,
    /// Optional title shown in the header.
    #[prop(default = "")]
    title: &'static str,
    /// Width class for Left/Right or height class for Top/Bottom.
    /// Default: "max-w-md" for left/right, "max-h-96" for top/bottom.
    #[prop(default = "")]
    width: &'static str,
    /// Panel body content.
    children: Children,
) -> impl IntoView {
    // Escape key listener
    Effect::new(move |_| {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
            move |ev: web_sys::KeyboardEvent| {
                if ev.key() == "Escape" {
                    open.set(false);
                }
            },
        );
        let _ = window.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    let has_title = !title.is_empty();
    let body = children();

    // Determine sizing default
    let size_class = if !width.is_empty() {
        width
    } else {
        match side {
            SheetSide::Left | SheetSide::Right => "max-w-md",
            SheetSide::Top | SheetSide::Bottom => "max-h-96",
        }
    };

    // Panel positioning (fixed to edge)
    let panel_position = match side {
        SheetSide::Right => "top-0 right-0 h-full",
        SheetSide::Left => "top-0 left-0 h-full",
        SheetSide::Top => "top-0 left-0 w-full",
        SheetSide::Bottom => "bottom-0 left-0 w-full",
    };

    // Panel width/height dimension
    let panel_dimension = match side {
        SheetSide::Left | SheetSide::Right => "w-full",
        SheetSide::Top | SheetSide::Bottom => "h-auto",
    };

    // Border on the inner edge
    let panel_border = match side {
        SheetSide::Right => "border-l",
        SheetSide::Left => "border-r",
        SheetSide::Top => "border-b",
        SheetSide::Bottom => "border-t",
    };

    // CSS transform values: closed → open
    let transform_closed = match side {
        SheetSide::Right => "translateX(100%)",
        SheetSide::Left => "translateX(-100%)",
        SheetSide::Top => "translateY(-100%)",
        SheetSide::Bottom => "translateY(100%)",
    };

    view! {
        // Backdrop + container
        <div
            class=move || {
                if open.get() {
                    "fixed inset-0 z-50"
                } else {
                    "fixed inset-0 z-50 pointer-events-none"
                }
            }
            role="dialog"
            aria-modal="true"
            aria-label=if has_title { title } else { "Panel" }
        >
            // Backdrop overlay
            <div
                class="absolute inset-0 transition-opacity duration-300"
                style=move || {
                    if open.get() {
                        "background: rgba(0,0,0,0.6); opacity: 1;"
                    } else {
                        "background: rgba(0,0,0,0.6); opacity: 0; pointer-events: none;"
                    }
                }
                on:mousedown=move |_| open.set(false)
            ></div>

            // Panel
            <div
                class=format!(
                    "fixed {} {} {} {} bg-dm-panel border-dm shadow-2xl \
                     flex flex-col transition-transform duration-300 ease-out",
                    panel_position, panel_dimension, size_class, panel_border
                )
                style=move || {
                    if open.get() {
                        "transform: translate(0);".to_string()
                    } else {
                        format!("transform: {};", transform_closed)
                    }
                }
            >
                // Header
                <div class="flex items-center justify-between px-5 py-4 border-b border-dm shrink-0">
                    {has_title.then(|| view! {
                        <h2 class="text-lg font-semibold text-dm-text">{title}</h2>
                    })}
                    {(!has_title).then(|| view! { <div></div> })}
                    <button
                        on:click=move |_| open.set(false)
                        class="p-1 rounded-md text-dm-muted hover:text-dm-text \
                               hover:bg-dm-hover transition-colors"
                        aria-label="Close panel"
                    >
                        <svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" fill="none"
                             viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M6 18 18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                // Body
                <div class="flex-1 overflow-y-auto px-5 py-4">
                    {body}
                </div>
            </div>
        </div>
    }
}

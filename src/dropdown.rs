//! Dropdown — a menu of actions/items triggered by a button or element.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Alignment of the dropdown menu relative to the trigger.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DropdownAlign {
    #[default]
    Start,
    End,
}

/// A single actionable item in the dropdown.
#[derive(Debug, Clone)]
pub struct DropdownItem {
    /// Unique key returned on selection.
    pub key: String,
    /// Display label.
    pub label: String,
    /// Optional SVG path data for a 16x16 icon (viewBox 0 0 24 24).
    pub icon: Option<String>,
    /// Render with danger (red) styling.
    pub danger: bool,
    /// Greyed-out, non-interactive.
    pub disabled: bool,
}

/// An entry in the dropdown — either an item, a visual separator, or a group label.
#[derive(Debug, Clone)]
pub enum DropdownEntry {
    Item(DropdownItem),
    Separator,
    Label(String),
}

/// A dropdown menu component (similar to Radix DropdownMenu).
///
/// The trigger element toggles the menu. Items fire `on_select` with their key.
/// Closes on item click, Escape, or click outside.
/// Uses CSS visibility toggling — menu is rendered once and shown/hidden via classes.
#[component]
pub fn Dropdown(
    /// Trigger element that opens/closes the menu.
    trigger: Children,
    /// Menu entries (items, separators, labels).
    items: Vec<DropdownEntry>,
    /// Called with the selected item's key.
    on_select: Box<dyn Fn(String)>,
    /// Horizontal alignment of the menu relative to the trigger.
    #[prop(default = DropdownAlign::Start)]
    align: DropdownAlign,
    /// Extra CSS classes on the outer wrapper.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let focused_index = RwSignal::new(-1_i32);

    // Store callback in a signal so closures can reference it without Rc
    let selected_key: RwSignal<Option<String>> = RwSignal::new(None);

    // Watch for selection and invoke callback
    let on_select = std::rc::Rc::new(on_select);
    {
        let on_select = on_select.clone();
        Effect::new(move |_| {
            if let Some(key) = selected_key.get() {
                on_select(key);
                selected_key.set(None);
            }
        });
    }

    // Collect selectable indices for keyboard navigation
    let selectable_indices: Vec<usize> = items
        .iter()
        .enumerate()
        .filter_map(|(i, entry)| match entry {
            DropdownEntry::Item(item) if !item.disabled => Some(i),
            _ => None,
        })
        .collect();
    let selectable_indices = std::rc::Rc::new(selectable_indices);

    // Collect item keys for selection by index
    let item_keys: Vec<Option<String>> = items
        .iter()
        .map(|entry| match entry {
            DropdownEntry::Item(item) => Some(item.key.clone()),
            _ => None,
        })
        .collect();
    let item_keys = std::rc::Rc::new(item_keys);

    // Close on Escape + keyboard navigation
    {
        let selectable_indices = selectable_indices.clone();
        let item_keys = item_keys.clone();
        Effect::new(move |_| {
            let window = match web_sys::window() {
                Some(w) => w,
                None => return,
            };
            let selectable = selectable_indices.clone();
            let keys = item_keys.clone();
            let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
                move |ev: web_sys::KeyboardEvent| {
                    if !open.get() {
                        return;
                    }
                    match ev.key().as_str() {
                        "Escape" => {
                            ev.prevent_default();
                            open.set(false);
                            focused_index.set(-1);
                        }
                        "ArrowDown" => {
                            ev.prevent_default();
                            if selectable.is_empty() {
                                return;
                            }
                            let current = focused_index.get();
                            let next = selectable
                                .iter()
                                .find(|&&i| i as i32 > current)
                                .or(selectable.first())
                                .copied()
                                .unwrap_or(0);
                            focused_index.set(next as i32);
                        }
                        "ArrowUp" => {
                            ev.prevent_default();
                            if selectable.is_empty() {
                                return;
                            }
                            let current = focused_index.get();
                            let prev = selectable
                                .iter()
                                .rev()
                                .find(|&&i| (i as i32) < current)
                                .or(selectable.last())
                                .copied()
                                .unwrap_or(0);
                            focused_index.set(prev as i32);
                        }
                        "Enter" => {
                            ev.prevent_default();
                            let idx = focused_index.get();
                            if idx >= 0 {
                                if let Some(Some(key)) = keys.get(idx as usize) {
                                    selected_key.set(Some(key.clone()));
                                    open.set(false);
                                    focused_index.set(-1);
                                }
                            }
                        }
                        _ => {}
                    }
                },
            );
            let _ = window
                .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            cb.forget();
        });
    }

    // Close on click outside
    Effect::new(move |_| {
        let document = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d,
            None => return,
        };
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
            use wasm_bindgen::JsCast;
            if !open.get() {
                return;
            }
            if let Some(target) = ev.target() {
                if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                    if el.closest("[data-dm-dropdown]").ok().flatten().is_some() {
                        return;
                    }
                }
            }
            open.set(false);
            focused_index.set(-1);
        });
        let _ =
            document.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    let align_class = match align {
        DropdownAlign::Start => "left-0",
        DropdownAlign::End => "right-0",
    };

    // Pre-render menu items (rendered once, visibility toggled via CSS)
    let menu_items = items
        .iter()
        .enumerate()
        .map(|(idx, entry)| match entry {
            DropdownEntry::Separator => view! {
                <div class="h-px bg-dm-border my-1 mx-1" role="separator"></div>
            }
            .into_any(),
            DropdownEntry::Label(text) => view! {
                <div
                    class="px-2 py-1 text-xs font-semibold text-dm-muted \
                           uppercase tracking-wider"
                    role="presentation"
                >
                    {text.clone()}
                </div>
            }
            .into_any(),
            DropdownEntry::Item(item) => {
                let key = item.key.clone();
                let label = item.label.clone();
                let icon = item.icon.clone();
                let danger = item.danger;
                let disabled = item.disabled;

                let base_class = if danger {
                    "text-red-400 hover:bg-red-500/10"
                } else {
                    "text-dm-text hover:bg-dm-hover"
                };

                let disabled_class = if disabled {
                    "opacity-50 pointer-events-none"
                } else {
                    "cursor-pointer"
                };

                view! {
                    <div
                        class=move || format!(
                            "flex items-center gap-2 px-2 py-1.5 text-sm rounded-md \
                             transition-colors {} {} {}",
                            base_class,
                            disabled_class,
                            if focused_index.get() == idx as i32 {
                                "bg-dm-hover"
                            } else {
                                ""
                            }
                        )
                        role="menuitem"
                        aria-disabled=if disabled { "true" } else { "false" }
                        tabindex=if disabled { "-1" } else { "0" }
                        on:click={
                            let key = key.clone();
                            move |_| {
                                if !disabled {
                                    selected_key.set(Some(key.clone()));
                                    open.set(false);
                                    focused_index.set(-1);
                                }
                            }
                        }
                    >
                        {icon.as_ref().map(|path| view! {
                            <svg class="w-4 h-4 shrink-0" xmlns="http://www.w3.org/2000/svg"
                                 fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                 stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                      d=path.clone() />
                            </svg>
                        })}
                        {label.clone()}
                    </div>
                }
                .into_any()
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class=format!("relative inline-block {}", class) data-dm-dropdown="">
            // Trigger
            <div
                on:click=move |_| {
                    open.update(|v| *v = !*v);
                    focused_index.set(-1);
                }
                aria-haspopup="true"
                aria-expanded=move || if open.get() { "true" } else { "false" }
            >
                {trigger()}
            </div>

            // Menu — visibility toggled via CSS class (not Show)
            <div
                class=move || {
                    if open.get() {
                        format!(
                            "absolute top-full mt-1 {} z-50 \
                             bg-dm-elevated border border-dm rounded-lg shadow-lg \
                             p-1 min-w-[180px] animate-dm-fade-in",
                            align_class
                        )
                    } else {
                        "hidden".to_string()
                    }
                }
                role="menu"
                aria-orientation="vertical"
            >
                {menu_items}
            </div>
        </div>
    }
}

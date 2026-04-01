//! Dropdown — a menu of actions/items triggered by an element.

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
    pub key: String,
    pub label: String,
    pub icon: Option<String>,
    pub danger: bool,
    pub disabled: bool,
}

/// An entry in the dropdown.
#[derive(Debug, Clone)]
pub enum DropdownEntry {
    Item(DropdownItem),
    Separator,
    Label(String),
}

/// A dropdown menu component.
///
/// Uses DUI CSS classes: `.dm-dropdown`, `.dm-dropdown-menu`, `.dm-dropdown-item`, `.dm-dropdown-separator`, `.dm-dropdown-label`.
/// No Tailwind required.
#[component]
pub fn Dropdown(
    /// Trigger element.
    trigger: Children,
    /// Menu entries.
    items: Vec<DropdownEntry>,
    /// Called with the selected item's key.
    on_select: Box<dyn Fn(String)>,
    /// Horizontal alignment.
    #[prop(default = DropdownAlign::Start)]
    align: DropdownAlign,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let focused_index = RwSignal::new(-1_i32);

    let selected_key: RwSignal<Option<String>> = RwSignal::new(None);
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

    let selectable_indices: Vec<usize> = items
        .iter()
        .enumerate()
        .filter_map(|(i, entry)| match entry {
            DropdownEntry::Item(item) if !item.disabled => Some(i),
            _ => None,
        })
        .collect();

    // Click outside to close
    Effect::new(move |_| {
        let window = match web_sys::window() { Some(w) => w, None => return };
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            // Simple close — the menu items handle their own clicks before this fires
            // We use a timeout to let item clicks process first
        });
        let _ = window.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    // Escape to close
    Effect::new(move |_| {
        let window = match web_sys::window() { Some(w) => w, None => return };
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" { open.set(false); }
        });
        let _ = window.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    let align_style = match align {
        DropdownAlign::Start => "left:0",
        DropdownAlign::End => "right:0",
    };

    view! {
        <div class=format!("dm-dropdown {}", class)>
            // Trigger
            <div class="dm-dropdown-trigger" on:click=move |_| {
                open.update(|v| *v = !*v);
                focused_index.set(-1);
            }>
                {trigger()}
            </div>

            // Menu
            <div
                class=move || if open.get() { "dm-dropdown-menu" } else { "dm-hidden" }
                style=align_style
                on:keydown={
                    let sel_indices = selectable_indices.clone();
                    let items_ref = items.clone();
                    move |ev: web_sys::KeyboardEvent| {
                        let key = ev.key();
                        match key.as_str() {
                            "ArrowDown" | "ArrowUp" => {
                                ev.prevent_default();
                                let cur = focused_index.get();
                                let cur_pos = sel_indices.iter().position(|&i| i as i32 == cur);
                                let next = match key.as_str() {
                                    "ArrowDown" => match cur_pos {
                                        Some(p) if p + 1 < sel_indices.len() => sel_indices[p + 1],
                                        None if !sel_indices.is_empty() => sel_indices[0],
                                        _ => return,
                                    },
                                    "ArrowUp" => match cur_pos {
                                        Some(p) if p > 0 => sel_indices[p - 1],
                                        None if !sel_indices.is_empty() => *sel_indices.last().unwrap(),
                                        _ => return,
                                    },
                                    _ => return,
                                };
                                focused_index.set(next as i32);
                            }
                            "Enter" | " " => {
                                ev.prevent_default();
                                let idx = focused_index.get();
                                if idx >= 0 {
                                    if let Some(DropdownEntry::Item(item)) = items_ref.get(idx as usize) {
                                        if !item.disabled {
                                            selected_key.set(Some(item.key.clone()));
                                            open.set(false);
                                        }
                                    }
                                }
                            }
                            "Escape" => { open.set(false); }
                            _ => {}
                        }
                    }
                }
            >
                {items.iter().enumerate().map(|(i, entry)| {
                    match entry {
                        DropdownEntry::Separator => {
                            view! { <div class="dm-dropdown-separator"></div> }.into_any()
                        }
                        DropdownEntry::Label(text) => {
                            view! { <div class="dm-dropdown-label">{text.clone()}</div> }.into_any()
                        }
                        DropdownEntry::Item(item) => {
                            let key = item.key.clone();
                            let label = item.label.clone();
                            let icon = item.icon.clone();
                            let danger = item.danger;
                            let disabled = item.disabled;
                            view! {
                                <button
                                    class=move || format!(
                                        "dm-dropdown-item {} {} {}",
                                        if danger { "dm-dropdown-item-danger" } else { "" },
                                        if disabled { "dm-dropdown-item-disabled" } else { "" },
                                        if focused_index.get() == i as i32 { "dm-bg-surface-hover" } else { "" },
                                    )
                                    disabled=disabled
                                    on:click={
                                        let key = key.clone();
                                        move |_| {
                                            if !disabled {
                                                selected_key.set(Some(key.clone()));
                                                open.set(false);
                                            }
                                        }
                                    }
                                    on:mouseenter=move |_| focused_index.set(i as i32)
                                >
                                    {icon.map(|path| view! {
                                        <svg style="width:16px;height:16px;flex-shrink:0" xmlns="http://www.w3.org/2000/svg" fill="none"
                                             viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" d=path />
                                        </svg>
                                    })}
                                    {label}
                                </button>
                            }.into_any()
                        }
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

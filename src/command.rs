//! CommandPalette — Cmd+K style command menu with fuzzy search, keyboard
//! navigation, grouped results, and full ARIA support.

use leptos::prelude::*;
use leptos::callback::{Callback, Callable};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

/// A single actionable item in the command palette.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandItem {
    /// Unique identifier — passed to `on_select` when chosen.
    pub id: String,
    /// Primary display text.
    pub label: String,
    /// Optional secondary description shown beneath / beside the label.
    pub description: Option<String>,
    /// Optional SVG `<path d="...">` data for a 20x20 viewBox icon.
    pub icon: Option<String>,
    /// Optional keyboard shortcut hint (e.g. `"⌘ K"`).
    /// Multiple keys separated by spaces each get their own keycap.
    pub shortcut: Option<String>,
    /// Optional group heading. Items sharing the same group value are rendered
    /// together under a single heading.
    pub group: Option<String>,
    /// Extra search terms that do not appear in the UI but improve findability.
    pub keywords: Vec<String>,
}

// ---------------------------------------------------------------------------
// Kbd inline styling (avoids cross-component import issues)
// ---------------------------------------------------------------------------

/// Inline keycap styling string, matching `kbd.rs`.
const KBD_CLASS: &str = "inline-flex items-center justify-center \
    min-w-[20px] h-5 px-1.5 text-[11px] font-mono font-medium leading-none \
    rounded border bg-dm-elevated text-dm-muted border-dm \
    shadow-[0_1px_0_1px_var(--dm-bg)] select-none";

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A full-screen command palette overlay with search, keyboard navigation,
/// and grouped results.
///
/// Uses the same CSS-visibility-toggle pattern as `Modal` — children are
/// rendered once, and the palette is shown/hidden via class swaps.
///
/// # Features
/// - **Search**: filters items by label, description, and keywords (case-insensitive).
/// - **Keyboard navigation**: Arrow Up/Down, Enter to select, Escape to close.
/// - **Grouping**: items with a `group` field are rendered under headings.
/// - **ARIA**: `role="dialog"`, combobox, listbox, option, and group roles.
///
/// # Example
/// ```rust
/// let open = RwSignal::new(false);
/// let items = Signal::derive(|| vec![
///     CommandItem {
///         id: "save".into(),
///         label: "Save file".into(),
///         description: Some("Save the current document".into()),
///         icon: None,
///         shortcut: Some("⌘ S".into()),
///         group: Some("File".into()),
///         keywords: vec!["write".into(), "persist".into()],
///     },
/// ]);
/// view! {
///     <CommandPalette
///         open=open
///         items=items
///         on_select=Callback::new(move |id: String| { /* handle */ })
///     />
/// }
/// ```
#[component]
pub fn CommandPalette(
    /// Controls visibility (writable so the palette can close itself).
    open: RwSignal<bool>,
    /// The full set of command items (filtering happens internally).
    #[prop(into)]
    items: Signal<Vec<CommandItem>>,
    /// Called with the selected item's `id` when the user picks one.
    on_select: Callback<String>,
    /// Placeholder text for the search input.
    #[prop(default = "Type a command or search\u{2026}")]
    placeholder: &'static str,
) -> impl IntoView {
    // -- Local state ---------------------------------------------------------
    let query = RwSignal::new(String::new());
    let active_index = RwSignal::new(0usize);

    // Unique ids for ARIA linkage.
    let input_id = "dm-cmd-input";
    let listbox_id = "dm-cmd-listbox";

    // -- Derived: filtered items ---------------------------------------------
    let filtered = Memo::new(move |_| {
        let q = query.get().to_lowercase();
        let all = items.get();
        if q.is_empty() {
            return all;
        }
        all.into_iter()
            .filter(|item| {
                item.label.to_lowercase().contains(&q)
                    || item
                        .description
                        .as_ref()
                        .map_or(false, |d| d.to_lowercase().contains(&q))
                    || item.keywords.iter().any(|k| k.to_lowercase().contains(&q))
            })
            .collect::<Vec<_>>()
    });

    // -- Helpers -------------------------------------------------------------
    // Clamp active_index whenever filtered list changes.
    Effect::new(move |_| {
        let len = filtered.get().len();
        if len == 0 {
            active_index.set(0);
        } else if active_index.get() >= len {
            active_index.set(len - 1);
        }
    });

    // Reset state when palette opens and focus the search input.
    Effect::new(move |_| {
        if open.get() {
            query.set(String::new());
            active_index.set(0);

            // Focus after DOM settles.
            request_animation_frame(move || {
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    if let Some(el) = doc.get_element_by_id(input_id) {
                        if let Some(html) = el.dyn_ref::<web_sys::HtmlElement>() {
                            let _ = html.focus();
                        }
                    }
                }
            });
        }
    });

    // Scroll the active item into view.
    let scroll_active_into_view = move || {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let selector = format!("[data-dm-cmd-idx=\"{}\"]", active_index.get_untracked());
            if let Ok(Some(el)) = doc.query_selector(&selector) {
                el.scroll_into_view();
            }
        }
    };

    // Close the palette.
    let close = move || {
        open.set(false);
    };

    // Fire on_select for the currently active item, then close.
    let do_select = move || {
        let list = filtered.get_untracked();
        let idx = active_index.get_untracked();
        if let Some(item) = list.get(idx) {
            on_select.run(item.id.clone());
        }
        close();
    };

    // -- Global Escape key listener (same pattern as Modal) ------------------
    Effect::new(move |_| {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" && open.get_untracked() {
                open.set(false);
            }
        });
        let _ = window.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    // -- View ----------------------------------------------------------------
    view! {
        <div
            class=move || {
                if open.get() {
                    "fixed inset-0 z-50 flex items-center justify-center animate-dm-fade-in"
                } else {
                    "hidden"
                }
            }
            style="background: rgba(0,0,0,0.60);"
            role="dialog"
            aria-modal="true"
            aria-label="Command palette"
            on:mousedown=move |ev| {
                // Close on backdrop click (not on panel)
                if let Some(target) = ev.target() {
                    if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                        if el.class_list().contains("fixed") {
                            close();
                        }
                    }
                }
            }
        >
            // Panel
            <div class="bg-dm-panel border border-dm rounded-xl shadow-2xl \
                        w-full max-w-lg mx-4 flex flex-col overflow-hidden \
                        animate-dm-scale-in">

                // ---- Search input section ----
                <div class="flex items-center gap-3 px-4 py-3 border-b border-dm">
                    // Magnifying glass icon
                    <svg
                        class="w-5 h-5 text-dm-muted shrink-0"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="2"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"
                        />
                    </svg>

                    <input
                        id=input_id
                        type="text"
                        placeholder=placeholder
                        autocomplete="off"
                        spellcheck="false"
                        role="combobox"
                        aria-expanded="true"
                        aria-controls=listbox_id
                        aria-autocomplete="list"
                        aria-activedescendant=move || format!("dm-cmd-opt-{}", active_index.get())
                        class="flex-1 bg-transparent text-dm-text text-base \
                               placeholder:text-dm-dim outline-none"
                        prop:value=move || query.get()
                        on:input=move |ev| {
                            query.set(event_target_value(&ev));
                            active_index.set(0);
                        }
                        on:keydown=move |ev: web_sys::KeyboardEvent| {
                            let key = ev.key();
                            match key.as_str() {
                                "ArrowDown" => {
                                    ev.prevent_default();
                                    let len = filtered.get_untracked().len();
                                    if len > 0 {
                                        active_index.update(|i| *i = (*i + 1) % len);
                                        scroll_active_into_view();
                                    }
                                }
                                "ArrowUp" => {
                                    ev.prevent_default();
                                    let len = filtered.get_untracked().len();
                                    if len > 0 {
                                        active_index.update(|i| {
                                            *i = if *i == 0 { len - 1 } else { *i - 1 };
                                        });
                                        scroll_active_into_view();
                                    }
                                }
                                "Enter" => {
                                    ev.prevent_default();
                                    do_select();
                                }
                                "Escape" => {
                                    ev.prevent_default();
                                    close();
                                }
                                _ => {}
                            }
                        }
                    />
                </div>

                // ---- Results list ----
                <div
                    id=listbox_id
                    role="listbox"
                    aria-label="Commands"
                    class="overflow-y-auto overscroll-contain py-2 px-2"
                    style="max-height: 300px;"
                >
                    {move || {
                        let list = filtered.get();

                        if list.is_empty() {
                            return view! {
                                <div class="px-4 py-8 text-center text-sm text-dm-muted select-none">
                                    "No results found."
                                </div>
                            }.into_any();
                        }

                        // Group items: collect (group_name, Vec<(global_idx, item)>)
                        let mut groups: Vec<(Option<String>, Vec<(usize, CommandItem)>)> = Vec::new();
                        for (idx, item) in list.into_iter().enumerate() {
                            let group_key = item.group.clone();
                            if let Some(last) = groups.last_mut() {
                                if last.0 == group_key {
                                    last.1.push((idx, item));
                                    continue;
                                }
                            }
                            groups.push((group_key, vec![(idx, item)]));
                        }

                        view! {
                            <div>
                                {groups.into_iter().map(|(group_name, members)| {
                                    let group_heading_id = group_name
                                        .as_ref()
                                        .map(|g| format!("dm-cmd-grp-{}", g.to_lowercase().replace(' ', "-")));
                                    let heading_id_attr = group_heading_id.clone().unwrap_or_default();

                                    view! {
                                        <div
                                            role="group"
                                            aria-labelledby=heading_id_attr.clone()
                                        >
                                            // Group heading
                                            {group_name.map(|name| {
                                                let gid = group_heading_id.clone().unwrap_or_default();
                                                view! {
                                                    <div
                                                        id=gid
                                                        class="text-xs font-semibold text-dm-muted \
                                                               uppercase tracking-wider px-2 py-1.5 \
                                                               select-none"
                                                    >
                                                        {name}
                                                    </div>
                                                }
                                            })}

                                            // Items in this group
                                            {members.into_iter().map(|(idx, item)| {
                                                let option_dom_id = format!("dm-cmd-opt-{}", idx);
                                                let item_id_click = item.id.clone();

                                                view! {
                                                    <div
                                                        id=option_dom_id
                                                        role="option"
                                                        aria-selected=move || {
                                                            if active_index.get() == idx { "true" } else { "false" }
                                                        }
                                                        data-dm-cmd-idx=idx.to_string()
                                                        class=move || format!(
                                                            "px-2 py-2 flex items-center gap-3 rounded-md \
                                                             cursor-pointer text-sm transition-colors duration-75 {}",
                                                            if active_index.get() == idx {
                                                                "bg-dm-hover"
                                                            } else {
                                                                ""
                                                            }
                                                        )
                                                        on:mouseenter=move |_| {
                                                            active_index.set(idx);
                                                        }
                                                        on:click={
                                                            let id = item_id_click.clone();
                                                            move |_| {
                                                                on_select.run(id.clone());
                                                                close();
                                                            }
                                                        }
                                                    >
                                                        // Icon
                                                        {item.icon.as_ref().map(|path_d| {
                                                            let d = path_d.clone();
                                                            view! {
                                                                <svg
                                                                    class="w-5 h-5 text-dm-muted shrink-0"
                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                    fill="none"
                                                                    viewBox="0 0 20 20"
                                                                    stroke-width="1.5"
                                                                    stroke="currentColor"
                                                                >
                                                                    <path
                                                                        stroke-linecap="round"
                                                                        stroke-linejoin="round"
                                                                        d=d
                                                                    />
                                                                </svg>
                                                            }
                                                        })}

                                                        // Label + description
                                                        <div class="flex-1 min-w-0">
                                                            <div class="text-dm-text truncate">
                                                                {item.label.clone()}
                                                            </div>
                                                            {item.description.as_ref().map(|desc| {
                                                                let d = desc.clone();
                                                                view! {
                                                                    <div class="text-xs text-dm-dim truncate mt-0.5">
                                                                        {d}
                                                                    </div>
                                                                }
                                                            })}
                                                        </div>

                                                        // Shortcut badge(s)
                                                        {item.shortcut.as_ref().map(|sc| {
                                                            let parts: Vec<String> = sc
                                                                .split_whitespace()
                                                                .map(|s| s.to_string())
                                                                .collect();
                                                            view! {
                                                                <span class="inline-flex items-center gap-0.5 shrink-0 ml-auto">
                                                                    {parts.into_iter().map(|k| {
                                                                        view! {
                                                                            <kbd class=KBD_CLASS>{k}</kbd>
                                                                        }
                                                                    }).collect::<Vec<_>>()}
                                                                </span>
                                                            }
                                                        })}
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }}
                </div>

                // ---- Footer: keyboard hints ----
                <div class="flex items-center gap-4 px-4 py-2.5 border-t border-dm \
                            text-xs text-dm-dim select-none">
                    <span class="inline-flex items-center gap-1">
                        <kbd class=KBD_CLASS>{"\u{2191}\u{2193}"}</kbd>
                        " Navigate"
                    </span>
                    <span class="inline-flex items-center gap-1">
                        <kbd class=KBD_CLASS>{"\u{21B5}"}</kbd>
                        " Select"
                    </span>
                    <span class="inline-flex items-center gap-1">
                        <kbd class=KBD_CLASS>{"Esc"}</kbd>
                        " Close"
                    </span>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Utility: requestAnimationFrame helper
// ---------------------------------------------------------------------------

fn request_animation_frame(f: impl FnOnce() + 'static) {
    let closure = Closure::once_into_js(f);
    if let Some(window) = web_sys::window() {
        let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
    }
}

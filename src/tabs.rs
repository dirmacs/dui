//! Tabs — horizontal tab navigation with active indicator.

use leptos::prelude::*;

/// A single tab definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TabItem {
    /// Unique key.
    pub key: String,
    /// Display label.
    pub label: String,
    /// Optional count badge (e.g. number of items).
    pub count: Option<usize>,
}

/// Horizontal tab navigation bar with an animated active indicator.
///
/// The consumer renders the appropriate tab content based on `active_tab`.
#[component]
pub fn Tabs(
    /// Tab definitions.
    items: Vec<TabItem>,
    /// Currently active tab key.
    #[prop(into)]
    active_tab: RwSignal<String>,
    /// Called when a tab is clicked.
    #[prop(optional)]
    on_change: Option<Box<dyn Fn(String)>>,
    /// Extra classes on the tab bar wrapper.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let on_change = std::rc::Rc::new(on_change);

    view! {
        <div class=format!(
            "flex items-center border-b border-dm gap-1 {}",
            class
        )>
            {items.into_iter().map(|tab| {
                let key = tab.key.clone();
                let key2 = tab.key.clone();
                let key3 = tab.key.clone();
                let on_change = on_change.clone();
                view! {
                    <button
                        class=move || format!(
                            "relative px-4 py-2.5 text-sm font-medium transition-colors duration-150 \
                             rounded-t-lg -mb-px {}",
                            if active_tab.get() == key {
                                "text-dm-accent"
                            } else {
                                "text-dm-muted hover:text-dm-text"
                            }
                        )
                        on:click={
                            let key2 = key2.clone();
                            let on_change = on_change.clone();
                            move |_| {
                                active_tab.set(key2.clone());
                                if let Some(ref cb) = *on_change {
                                    cb(key2.clone());
                                }
                            }
                        }
                    >
                        <span class="flex items-center gap-2">
                            {tab.label.clone()}
                            {tab.count.map(|c| view! {
                                <span class="text-xs bg-dm-elevated text-dm-dim px-1.5 py-0.5 rounded-md">
                                    {c.to_string()}
                                </span>
                            })}
                        </span>

                        // Active indicator line
                        <Show when={
                            let key3 = key3.clone();
                            move || active_tab.get() == key3
                        }>
                            <span class="absolute bottom-0 left-2 right-2 h-0.5 bg-dm-accent rounded-full"></span>
                        </Show>
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

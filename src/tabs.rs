//! Tabs — horizontal tab navigation with active indicator.

use leptos::prelude::*;

/// A single tab definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TabItem {
    pub key: String,
    pub label: String,
    pub count: Option<usize>,
}

/// Horizontal tab navigation bar with an animated active indicator.
///
/// Uses DUI CSS classes: `.dm-tabs`, `.dm-tab`, `.dm-tab-active`, `.dm-tab-count`.
/// No Tailwind required.
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
        <div role="tablist" class=format!("dm-tabs {}", class)>
            {items.into_iter().map(|tab| {
                let key_aria = tab.key.clone();
                let key_tab = tab.key.clone();
                let key_class = tab.key.clone();
                let key2 = tab.key.clone();
                let on_change = on_change.clone();
                view! {
                    <button
                        role="tab"
                        aria-selected=move || (active_tab.get() == key_aria).to_string()
                        tabindex=move || if active_tab.get() == key_tab { "0" } else { "-1" }
                        class=move || format!(
                            "dm-tab {}",
                            if active_tab.get() == key_class { "dm-tab-active" } else { "" },
                        )
                        on:click={
                            let on_change = on_change.clone();
                            move |_| {
                                active_tab.set(key2.clone());
                                if let Some(ref cb) = *on_change {
                                    cb(key2.clone());
                                }
                            }
                        }
                    >
                        {tab.label.clone()}
                        {tab.count.map(|c| view! {
                            <span class="dm-tab-count">{c}</span>
                        })}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

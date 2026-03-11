//! Sidebar — collapsible nav with icon+label items, active highlight, user avatar.

use leptos::prelude::*;

/// A single navigation item in the sidebar.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    /// Unique key for active-state matching.
    pub key: String,
    /// Display label.
    pub label: String,
    /// SVG path data for the icon (24x24 viewBox, single path).
    pub icon_path: String,
}

/// User info displayed at the bottom of the sidebar.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarUser {
    pub name: String,
    pub email: String,
    /// URL or empty for initials fallback.
    pub avatar_url: String,
}

/// A collapsible sidebar navigation panel.
///
/// Features:
/// - Collapse to icon-only mode (48px) or expand to full (256px).
/// - Active item has an accent left-border indicator.
/// - User avatar at the bottom with name/email (hidden when collapsed).
#[component]
pub fn Sidebar(
    /// List of nav items.
    items: Vec<SidebarItem>,
    /// Currently active item key.
    #[prop(into)]
    active: RwSignal<String>,
    /// Whether the sidebar is collapsed (icon only).
    #[prop(into)]
    collapsed: RwSignal<bool>,
    /// Called when a nav item is clicked.
    on_navigate: Box<dyn Fn(String)>,
    /// Optional user info for the bottom section.
    #[prop(optional)]
    user: Option<SidebarUser>,
    /// Optional brand label (shown expanded only).
    #[prop(default = "Dirmacs")]
    brand: &'static str,
) -> impl IntoView {
    let on_nav = std::rc::Rc::new(on_navigate);

    view! {
        <aside class=move || format!(
            "h-screen bg-dm-panel border-r border-dm flex flex-col \
             transition-all duration-300 ease-out shrink-0 {}",
            if collapsed.get() { "w-16" } else { "w-64" }
        )>
            // Brand header
            <div class="flex items-center gap-3 px-4 h-14 border-b border-dm shrink-0">
                // Logo mark
                <div class="w-8 h-8 rounded-lg bg-dm-accent/20 flex items-center justify-center \
                            text-dm-accent font-bold text-sm shrink-0">
                    "D"
                </div>
                <Show when=move || !collapsed.get()>
                    <span class="text-dm-text font-semibold text-sm truncate">{brand}</span>
                </Show>

                // Collapse toggle
                <button
                    class="ml-auto p-1.5 rounded-md text-dm-dim hover:text-dm-text \
                           hover:bg-dm-hover transition-colors duration-150"
                    on:click=move |_| collapsed.update(|c| *c = !*c)
                >
                    <svg class=move || format!(
                        "w-4 h-4 transition-transform duration-300 {}",
                        if collapsed.get() { "rotate-180" } else { "" }
                    ) xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                       stroke-width="2" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round"
                              d="M15.75 19.5 8.25 12l7.5-7.5" />
                    </svg>
                </button>
            </div>

            // Nav items
            <nav class="flex-1 overflow-y-auto py-3 px-2 space-y-1 dm-scrollbar">
                {items.into_iter().map(|item| {
                    let key = item.key.clone();
                    let key2 = item.key.clone();
                    let on_nav = on_nav.clone();
                    view! {
                        <button
                            class=move || {
                                let is_active = active.get() == key;
                                format!(
                                    "w-full flex items-center gap-3 rounded-lg \
                                     transition-all duration-150 relative group {} {}",
                                    if collapsed.get() { "px-3 py-2.5 justify-center" }
                                    else { "px-3 py-2.5" },
                                    if is_active {
                                        "bg-dm-accent/10 text-dm-accent"
                                    } else {
                                        "text-dm-muted hover:text-dm-text hover:bg-dm-hover"
                                    }
                                )
                            }
                            on:click={
                                let key2 = key2.clone();
                                let on_nav = on_nav.clone();
                                move |_| {
                                    active.set(key2.clone());
                                    on_nav(key2.clone());
                                }
                            }
                        >
                            // Active indicator bar
                            <Show when={
                                let key = item.key.clone();
                                move || active.get() == key
                            }>
                                <span class="absolute left-0 top-1/2 -translate-y-1/2 w-[3px] h-5 \
                                             bg-dm-accent rounded-r-full"></span>
                            </Show>

                            // Icon
                            <svg class="w-5 h-5 shrink-0" xmlns="http://www.w3.org/2000/svg"
                                 fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" d=item.icon_path.clone() />
                            </svg>

                            // Label (hidden when collapsed)
                            <Show when=move || !collapsed.get()>
                                <span class="text-sm font-medium truncate">{item.label.clone()}</span>
                            </Show>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </nav>

            // User section at bottom
            {user.map(|u| {
                let initials = u.name.split_whitespace()
                    .filter_map(|w| w.chars().next())
                    .take(2)
                    .collect::<String>()
                    .to_uppercase();
                let has_avatar = !u.avatar_url.is_empty();
                let avatar_url = u.avatar_url.clone();

                view! {
                    <div class="border-t border-dm px-3 py-3 shrink-0">
                        <div class=move || format!(
                            "flex items-center gap-3 rounded-lg p-2 \
                             hover:bg-dm-hover transition-colors duration-150 cursor-pointer {}",
                            if collapsed.get() { "justify-center" } else { "" }
                        )>
                            // Avatar
                            <div class="w-8 h-8 rounded-full bg-dm-accent/20 flex items-center justify-center \
                                        text-dm-accent text-xs font-semibold shrink-0 overflow-hidden">
                                {if has_avatar {
                                    view! { <img src=avatar_url.clone() class="w-full h-full object-cover" /> }.into_any()
                                } else {
                                    view! { <span>{initials.clone()}</span> }.into_any()
                                }}
                            </div>

                            <Show when=move || !collapsed.get()>
                                <div class="flex-1 min-w-0">
                                    <div class="text-sm font-medium text-dm-text truncate">{u.name.clone()}</div>
                                    <div class="text-xs text-dm-dim truncate">{u.email.clone()}</div>
                                </div>
                            </Show>
                        </div>
                    </div>
                }
            })}
        </aside>
    }
}

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
/// - Collapse to icon-only mode (64px) or expand to full (256px).
/// - Active item uses the `nav-item active` CSS class (animated left-border,
///   gradient background, accent text color).
/// - User avatar at the bottom with gradient circle + initials.
/// - Labels fade smoothly on collapse/expand via CSS transitions.
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
            "h-screen bg-[var(--dm-surface)] border-r border-[var(--dm-border)] flex flex-col \
             shrink-0 overflow-hidden {}",
            if collapsed.get() { "w-16" } else { "w-64" }
        )
        style="transition: width var(--dm-duration-slow, 350ms) cubic-bezier(0.4, 0, 0.2, 1)"
        >
            // Brand header
            <div class=move || format!(
                "flex items-center h-14 border-b border-[var(--dm-border)] shrink-0 {}",
                if collapsed.get() { "justify-center px-2" } else { "gap-3 px-4" }
            )>
                <div class="w-8 h-8 rounded-lg flex items-center justify-center \
                            text-white font-bold text-sm shrink-0"
                     style="background: linear-gradient(135deg, #2563eb, #3b82f6)">
                    "D"
                </div>
                <Show when=move || !collapsed.get()>
                    <span class="font-semibold text-sm tracking-wide text-gray-100 truncate">{brand}</span>
                    <button
                        class="ml-auto p-1.5 rounded-md text-[var(--dm-text-dim)] hover:text-[var(--dm-text)] \
                               hover:bg-[var(--dm-surface-hover)]"
                        on:click=move |_| collapsed.set(true)
                    >
                        <svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                           stroke-width="2" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M15.75 19.5 8.25 12l7.5-7.5" />
                        </svg>
                    </button>
                </Show>
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
                                     relative nav-item text-[var(--dm-text-secondary)] {} {}",
                                    if collapsed.get() { "px-3 py-2.5 justify-center" }
                                    else { "px-3 py-2.5" },
                                    if is_active { "active" } else { "" }
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
                            // Icon
                            <svg class="w-5 h-5 shrink-0" xmlns="http://www.w3.org/2000/svg"
                                 fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" d=item.icon_path.clone() />
                            </svg>

                            // Label — always rendered, fades via CSS
                            <span class=move || format!(
                                "text-sm font-medium truncate dm-sidebar-label {}",
                                if collapsed.get() { "dm-sidebar-label-hidden" } else { "dm-sidebar-label-visible" }
                            )>{item.label.clone()}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </nav>

            // Expand button — only when collapsed, sits below nav items
            <Show when=move || collapsed.get()>
                <div class="px-2 pb-2 shrink-0">
                    <button
                        class="w-full flex items-center justify-center py-2 rounded-lg \
                               text-[var(--dm-text-dim)] hover:text-[var(--dm-text)] \
                               hover:bg-[var(--dm-surface-hover)]"
                        on:click=move |_| collapsed.set(false)
                        title="Expand sidebar"
                    >
                        <svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                           stroke-width="1.5" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15" />
                        </svg>
                    </button>
                </div>
            </Show>

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
                    <div class="border-t border-[var(--dm-border)] px-3 py-3 shrink-0">
                        <div class=move || format!(
                            "flex items-center gap-3 rounded-lg p-2 \
                             hover:bg-[var(--dm-surface-hover)] cursor-pointer {}",
                            if collapsed.get() { "justify-center" } else { "" }
                        )>
                            // Avatar — gradient circle with initials
                            <div class="w-9 h-9 rounded-full flex items-center justify-center \
                                        text-white text-sm font-semibold shrink-0 overflow-hidden"
                                 style="background: linear-gradient(135deg, #2563eb, #7c3aed)">
                                {if has_avatar {
                                    view! { <img src=avatar_url.clone() class="w-full h-full object-cover" /> }.into_any()
                                } else {
                                    view! { <span>{initials.clone()}</span> }.into_any()
                                }}
                            </div>

                            <div class=move || format!(
                                "flex-1 min-w-0 dm-sidebar-label {}",
                                if collapsed.get() { "dm-sidebar-label-hidden" } else { "dm-sidebar-label-visible" }
                            )>
                                <div class="text-sm font-medium text-gray-200 truncate">{u.name.clone()}</div>
                                <div class="text-xs text-gray-500 truncate">{u.email.clone()}</div>
                            </div>
                        </div>
                    </div>
                }
            })}
        </aside>
    }
}

//! Sidebar — collapsible nav with icon+label items, active highlight, user avatar.

use leptos::prelude::*;

/// A single navigation item in the sidebar.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    pub key: String,
    pub label: String,
    pub icon_path: String,
}

/// User info displayed at the bottom of the sidebar.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarUser {
    pub name: String,
    pub email: String,
    pub avatar_url: String,
}

/// A collapsible sidebar navigation panel.
///
/// Uses DUI CSS classes: `.dm-sidebar`, `.dm-sidebar-collapsed/expanded`, `.dm-sidebar-item`, `.dm-sidebar-item-active`, etc.
/// No Tailwind required.
#[component]
pub fn Sidebar(
    /// List of nav items.
    items: Vec<SidebarItem>,
    /// Currently active item key.
    #[prop(into)]
    active: RwSignal<String>,
    /// Whether the sidebar is collapsed.
    #[prop(into)]
    collapsed: RwSignal<bool>,
    /// Called when a nav item is clicked.
    on_navigate: Box<dyn Fn(String)>,
    /// Optional user info.
    #[prop(optional)]
    user: Option<SidebarUser>,
    /// Optional brand label.
    #[prop(default = "Dirmacs")]
    brand: &'static str,
) -> impl IntoView {
    let on_nav = std::rc::Rc::new(on_navigate);

    view! {
        <aside class=move || format!(
            "dm-sidebar {}",
            if collapsed.get() { "dm-sidebar-collapsed" } else { "dm-sidebar-expanded" }
        )>
            // Brand header
            <div class="dm-sidebar-header">
                <div class="dm-sidebar-brand-icon">"D"</div>
                <Show when=move || !collapsed.get()>
                    <span class="dm-sidebar-brand-text">{brand}</span>
                    <button
                        class="dm-modal-close"
                        style="margin-left:auto"
                        on:click=move |_| collapsed.set(true)
                    >
                        <svg style="width:16px;height:16px" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                             stroke-width="2" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5 8.25 12l7.5-7.5" />
                        </svg>
                    </button>
                </Show>
            </div>

            // Nav items
            <nav class="dm-sidebar-nav dm-scrollbar">
                {items.into_iter().map(|item| {
                    let key = item.key.clone();
                    let key2 = item.key.clone();
                    let on_nav = on_nav.clone();
                    view! {
                        <button
                            class=move || format!(
                                "dm-sidebar-item {}",
                                if active.get() == key { "dm-sidebar-item-active" } else { "" },
                            )
                            on:click={
                                let key2 = key2.clone();
                                let on_nav = on_nav.clone();
                                move |_| { active.set(key2.clone()); on_nav(key2.clone()); }
                            }
                        >
                            <svg class="dm-sidebar-item-icon" xmlns="http://www.w3.org/2000/svg"
                                 fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" d=item.icon_path.clone() />
                            </svg>
                            <span class="dm-sidebar-item-label">{item.label.clone()}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </nav>

            // Expand button when collapsed
            <Show when=move || collapsed.get()>
                <div class="dm-p-2">
                    <button
                        class="dm-sidebar-item dm-justify-center"
                        on:click=move |_| collapsed.set(false)
                        title="Expand sidebar"
                    >
                        <svg style="width:20px;height:20px" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                             stroke-width="1.5" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15" />
                        </svg>
                    </button>
                </div>
            </Show>

            // User section
            {user.map(|u| {
                let initials = u.name.split_whitespace()
                    .filter_map(|w| w.chars().next()).take(2)
                    .collect::<String>().to_uppercase();
                let has_avatar = !u.avatar_url.is_empty();
                let avatar_url = u.avatar_url.clone();

                view! {
                    <div class="dm-sidebar-user">
                        <div class="dm-avatar dm-avatar-md">
                            {if has_avatar {
                                view! { <img src=avatar_url.clone() style="width:100%;height:100%;object-fit:cover" /> }.into_any()
                            } else {
                                view! { <span>{initials.clone()}</span> }.into_any()
                            }}
                        </div>
                        <div class="dm-sidebar-user-info">
                            <div class="dm-sidebar-user-name">{u.name.clone()}</div>
                            <div class="dm-sidebar-user-email">{u.email.clone()}</div>
                        </div>
                    </div>
                }
            })}
        </aside>
    }
}

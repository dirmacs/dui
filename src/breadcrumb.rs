//! Breadcrumb — navigation trail showing the current page hierarchy.

use leptos::prelude::*;

/// A single item in the breadcrumb trail.
#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

/// A horizontal breadcrumb navigation bar.
///
/// Uses DUI CSS utility classes. No Tailwind required.
#[component]
pub fn Breadcrumb(
    /// Breadcrumb items in order (first = root, last = current page).
    items: Vec<BreadcrumbItem>,
    /// Separator character between items (default "/").
    #[prop(default = "/")]
    separator: &'static str,
    /// Extra CSS classes on the nav wrapper.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let total = items.len();

    view! {
        <nav aria-label="Breadcrumb" class=class>
            <ol role="list" class="dm-flex dm-items-center dm-gap-2 dm-text-sm">
                {items.into_iter().enumerate().map(|(i, item)| {
                    let is_last = i == total - 1;
                    let sep = separator;

                    view! {
                        <li class="dm-flex dm-items-center dm-gap-2">
                            {(i > 0).then(|| view! {
                                <span class="dm-text-dim dm-select-none" aria-hidden="true">{sep}</span>
                            })}
                            {if is_last {
                                view! {
                                    <span class="dm-text-primary dm-font-medium" aria-current="page">
                                        {item.label.clone()}
                                    </span>
                                }.into_any()
                            } else if let Some(href) = &item.href {
                                view! {
                                    <a href=href.clone() class="dm-text-muted dm-no-underline dm-transition-colors dm-cursor-pointer" style="hover:color:var(--dm-accent)">
                                        {item.label.clone()}
                                    </a>
                                }.into_any()
                            } else {
                                view! {
                                    <span class="dm-text-muted">{item.label.clone()}</span>
                                }.into_any()
                            }}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ol>
        </nav>
    }
}

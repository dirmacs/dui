//! Breadcrumb — navigation trail showing the current page hierarchy.

use leptos::prelude::*;

/// A single item in the breadcrumb trail.
#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    /// Display label.
    pub label: String,
    /// Optional navigation href. The last item typically has no href (current page).
    pub href: Option<String>,
}

/// A horizontal breadcrumb navigation bar.
///
/// Renders items as `<a>` tags (or plain `<span>` for the last item).
/// The last item is styled as the current page with `aria-current="page"`.
///
/// ```rust
/// view! {
///     <Breadcrumb items=vec![
///         BreadcrumbItem { label: "Home".into(), href: Some("/".into()) },
///         BreadcrumbItem { label: "Tenants".into(), href: Some("/tenants".into()) },
///         BreadcrumbItem { label: "Kasino".into(), href: None },
///     ] />
/// }
/// ```
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
        <nav
            aria-label="Breadcrumb"
            class=format!("{}", class)
        >
            <ol
                role="list"
                class="flex items-center gap-1.5 text-sm"
            >
                {items.into_iter().enumerate().map(|(i, item)| {
                    let is_last = i == total - 1;
                    let sep = separator;

                    view! {
                        <li class="flex items-center gap-1.5">
                            // Separator (before every item except the first)
                            {(i > 0).then(|| view! {
                                <span
                                    class="text-dm-dim select-none"
                                    aria-hidden="true"
                                >
                                    {sep}
                                </span>
                            })}

                            // Item
                            {if is_last {
                                // Current page — plain text, not a link
                                view! {
                                    <span
                                        class="text-dm-text font-medium"
                                        aria-current="page"
                                    >
                                        {item.label.clone()}
                                    </span>
                                }.into_any()
                            } else if let Some(href) = &item.href {
                                // Linked ancestor
                                view! {
                                    <a
                                        href=href.clone()
                                        class="text-dm-muted hover:text-dm-accent \
                                               transition-colors cursor-pointer"
                                    >
                                        {item.label.clone()}
                                    </a>
                                }.into_any()
                            } else {
                                // Non-linked ancestor (no href)
                                view! {
                                    <span class="text-dm-muted">
                                        {item.label.clone()}
                                    </span>
                                }.into_any()
                            }}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ol>
        </nav>
    }
}

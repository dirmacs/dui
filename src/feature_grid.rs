//! FeatureGrid — responsive grid of feature/product cards for landing pages.
//! Uses DUI Card component internally.

use leptos::prelude::*;

/// A feature item for the grid.
#[derive(Debug, Clone)]
pub struct FeatureItem {
    pub icon: String,
    pub title: String,
    pub description: String,
    pub href: Option<String>,
}

impl FeatureItem {
    pub fn new(icon: &str, title: &str, description: &str) -> Self {
        Self {
            icon: icon.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            href: None,
        }
    }

    pub fn with_link(mut self, href: &str) -> Self {
        self.href = Some(href.to_string());
        self
    }
}

/// Number of columns in the grid.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum GridColumns {
    Two,
    #[default]
    Three,
    Four,
}

/// Responsive feature card grid.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <FeatureGrid
///         items=vec![
///             FeatureItem::new("*", "Fast", "Built in Rust for speed"),
///             FeatureItem::new("*", "Safe", "Memory-safe by default"),
///             FeatureItem::new("*", "Open", "MIT licensed"),
///         ]
///     />
/// }
/// ```
#[component]
pub fn FeatureGrid(
    /// Feature items to display.
    items: Vec<FeatureItem>,
    /// Number of columns (default: 3).
    #[prop(default = GridColumns::Three)]
    columns: GridColumns,
    /// Extra CSS classes on the grid container.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let grid_class = match columns {
        GridColumns::Two => "grid grid-cols-1 md:grid-cols-2 gap-4",
        GridColumns::Three => "grid grid-cols-1 md:grid-cols-3 gap-4",
        GridColumns::Four => "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4",
    };

    view! {
        <div class=format!("{} {}", grid_class, class)>
            {items.iter().map(|item| {
                let icon = item.icon.clone();
                let title = item.title.clone();
                let description = item.description.clone();
                let href = item.href.clone();

                let inner = view! {
                    <div class="bg-[var(--dm-surface)] border-2 border-[var(--dm-border)] rounded-lg p-6 transition-all duration-200 hover:border-[var(--dm-border-hover)] hover:-translate-y-0.5">
                        <div class="text-2xl mb-3 opacity-60">{icon}</div>
                        <h3 class="font-mono text-sm font-bold uppercase tracking-[0.04em] text-[var(--dm-text)] mb-2">{title}</h3>
                        <p class="text-[0.8rem] text-[var(--dm-text-secondary)] leading-relaxed" style="font-family: var(--dm-font-body);">{description}</p>
                    </div>
                };

                if let Some(url) = href {
                    view! {
                        <a href=url class="no-underline text-inherit block">{inner}</a>
                    }.into_any()
                } else {
                    inner.into_any()
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

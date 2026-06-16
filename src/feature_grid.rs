//! FeatureGrid — responsive grid of feature/product cards.

use leptos::prelude::*;

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

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum GridColumns {
    Two,
    #[default]
    Three,
    Four,
}

/// Responsive feature card grid. Uses DUI CSS: `.dm-card`, `.dm-grid-*`. No Tailwind.
#[component]
pub fn FeatureGrid(
    items: Vec<FeatureItem>,
    #[prop(default = GridColumns::Three)] columns: GridColumns,
    #[prop(default = "")] class: &'static str,
) -> impl IntoView {
    let grid_class = match columns {
        GridColumns::Two => "dm-grid-2",
        GridColumns::Three => "dm-grid-3",
        GridColumns::Four => "dm-grid-4",
    };

    view! {
        <div class=format!("{} {}", grid_class, class)>
            {items.iter().map(|item| {
                let icon = item.icon.clone();
                let title = item.title.clone();
                let description = item.description.clone();
                let href = item.href.clone();

                let inner = view! {
                    <div class="dm-card">
                        <div class="dm-text-2xl dm-mb-3 dm-opacity-60">{icon}</div>
                        <h3 class="dm-font-mono dm-text-sm dm-font-bold dm-uppercase dm-tracking-wide dm-text-primary dm-mb-2">{title}</h3>
                        <p class="dm-text-xs dm-text-secondary dm-leading-relaxed dm-font-sans">{description}</p>
                    </div>
                };

                if let Some(url) = href {
                    view! { <a href=url class="dm-no-underline dm-block">{inner}</a> }.into_any()
                } else {
                    inner.into_any()
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

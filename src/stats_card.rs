//! StatsCard — compact metric display with title, value, optional subtitle and icon.

use leptos::prelude::*;

/// A compact stats card for displaying a single metric.
///
/// Uses DUI CSS classes: `.dm-card`, `.dm-card-accent`.
/// No Tailwind required.
#[component]
pub fn StatsCard(
    /// Metric label (e.g. "Total Tenants").
    title: String,
    /// Metric value (e.g. "42").
    value: String,
    /// Optional secondary text below the value.
    #[prop(optional)]
    subtitle: Option<String>,
    /// Optional icon text/emoji displayed beside the title.
    #[prop(optional)]
    icon: Option<String>,
    /// Extra CSS class on the value text.
    #[prop(default = "")]
    value_class: &'static str,
) -> impl IntoView {
    view! {
        <div class="dm-card dm-card-accent">
            <div class="dm-flex dm-items-center dm-justify-between dm-mb-2">
                <span class="dm-label">{title}</span>
                {icon.map(|i| view! { <span class="dm-text-accent">{i}</span> })}
            </div>
            <div class=format!("dm-font-mono dm-font-bold dm-text-primary {}", value_class) style="font-size:28px">
                {value}
            </div>
            {subtitle.map(|s| view! { <div class="dm-text-xs dm-text-secondary dm-mt-1">{s}</div> })}
        </div>
    }
}

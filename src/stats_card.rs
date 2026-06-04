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
        <div class="dm-card dm-card-accent" style="padding:0.8rem 0.95rem">
            <div class="dm-flex dm-items-center dm-justify-between" style="margin-bottom:0.3rem">
                <span class="dm-label" style="font-size:11px;letter-spacing:0.04em">{title}</span>
                {icon.map(|i| view! { <span class="dm-text-accent" style="font-size:12px;opacity:0.7">{i}</span> })}
            </div>
            <div class=format!("dm-font-mono dm-font-bold dm-text-primary {}", value_class) style="font-size:20px;line-height:1.15">
                {value}
            </div>
            {subtitle.map(|s| view! { <div class="dm-text-xs dm-text-secondary dm-mt-1">{s}</div> })}
        </div>
    }
}

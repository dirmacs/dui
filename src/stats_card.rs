//! StatsCard — compact metric display with title, value, optional subtitle and icon.

use leptos::prelude::*;

/// A compact stats card for displaying a single metric.
///
/// Used in dashboard grids to show counts, totals, and KPIs.
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
    /// Extra CSS class on the value text (e.g. "text-dm-success").
    #[prop(default = "")]
    value_class: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-[var(--dm-surface)] border-2 border-[var(--dm-border)] border-l-4 border-l-[var(--dm-accent)] rounded-lg p-6">
            <div class="flex items-center justify-between mb-2">
                <span class="font-mono text-[11px] font-medium uppercase tracking-[0.05em] text-[var(--dm-text-secondary)]">{title}</span>
                {icon.map(|i| view! { <span class="text-[var(--dm-accent)]">{i}</span> })}
            </div>
            <div class=format!("font-mono text-[28px] font-bold text-[var(--dm-text)] {}", value_class)>{value}</div>
            {subtitle.map(|s| view! { <div class="text-xs text-[var(--dm-text-secondary)] mt-1">{s}</div> })}
        </div>
    }
}

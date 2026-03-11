//! EmptyState — centered placeholder message with icon for empty lists/tables.

use leptos::prelude::*;

/// A centered empty-state placeholder with icon, title, description, and optional action.
///
/// Use this when a list, table, or section has no data yet. The icon should convey
/// the type of content that is missing.
#[component]
pub fn EmptyState(
    /// Title text (e.g. "No agents yet").
    title: String,
    /// Descriptive subtitle.
    #[prop(default = "")]
    description: &'static str,
    /// SVG path data for the illustration icon (24x24 viewBox).
    #[prop(default = "M2.25 13.5a8.25 8.25 0 0 1 8.25-8.25.75.75 0 0 1 .75.75v6.75H18a.75.75 0 0 1 .75.75 8.25 8.25 0 0 1-16.5 0Z")]
    icon_path: &'static str,
    /// Optional action slot (e.g. a "Create" button).
    #[prop(optional)]
    action: Option<Children>,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!(
            "flex flex-col items-center justify-center text-center py-16 px-8 \
             animate-dm-fade-in {}",
            class
        )>
            // Icon circle
            <div class="w-16 h-16 rounded-2xl bg-dm-accent/10 flex items-center justify-center mb-5">
                <svg class="w-8 h-8 text-dm-accent/60" xmlns="http://www.w3.org/2000/svg"
                     fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                </svg>
            </div>

            // Title
            <h3 class="text-lg font-semibold text-dm-text mb-1.5">{title}</h3>

            // Description
            {(!description.is_empty()).then(|| view! {
                <p class="text-sm text-dm-muted max-w-sm mb-6">{description}</p>
            })}

            // Action slot
            {action.map(|a| view! {
                <div class="mt-2">{a()}</div>
            })}
        </div>
    }
}

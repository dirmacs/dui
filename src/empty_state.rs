//! EmptyState — centered placeholder message with icon for empty lists/tables.

use leptos::prelude::*;

/// A centered empty-state placeholder with icon, title, description, and optional action.
///
/// Uses DUI CSS classes: `.dm-empty-state`, `.dm-empty-icon`, `.dm-empty-title`, `.dm-empty-desc`.
/// No Tailwind required.
#[component]
pub fn EmptyState(
    /// Title text (e.g. "No agents yet").
    title: String,
    /// Descriptive subtitle.
    #[prop(default = "")]
    description: &'static str,
    /// SVG path data for the illustration icon (24x24 viewBox).
    #[prop(
        default = "M2.25 13.5a8.25 8.25 0 0 1 8.25-8.25.75.75 0 0 1 .75.75v6.75H18a.75.75 0 0 1 .75.75 8.25 8.25 0 0 1-16.5 0Z"
    )]
    icon_path: &'static str,
    /// Optional action slot (e.g. a "Create" button).
    #[prop(optional)]
    action: Option<Children>,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("dm-empty-state dm-animate-fade-in {}", class)>
            // Icon
            <div class="dm-empty-icon">
                <svg style="width:32px;height:32px;color:var(--dm-accent);opacity:0.6" xmlns="http://www.w3.org/2000/svg"
                     fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                </svg>
            </div>

            // Title
            <h3 class="dm-empty-title">{title}</h3>

            // Description
            {(!description.is_empty()).then(|| view! {
                <p class="dm-empty-desc">{description}</p>
            })}

            // Action slot
            {action.map(|a| view! {
                <div class="dm-mt-4">{a()}</div>
            })}
        </div>
    }
}

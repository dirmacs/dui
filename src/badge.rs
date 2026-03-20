//! Badge — small colored tag in gray/blue/green/yellow/red/purple.

use leptos::prelude::*;

/// Badge color variant.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BadgeColor {
    #[default]
    Gray,
    Blue,
    Green,
    Yellow,
    Red,
    Purple,
}

/// A small inline tag/label with a colored background.
///
/// Useful for statuses, counts, categories, tags.
#[component]
pub fn Badge(
    /// Color variant.
    #[prop(default = BadgeColor::Gray)]
    color: BadgeColor,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
    /// Badge text content.
    children: Children,
) -> impl IntoView {
    let color_classes = match color {
        BadgeColor::Gray   => "bg-[var(--dm-surface)] text-[var(--dm-text-secondary)] border-2 border-[var(--dm-border)]",
        BadgeColor::Blue   => "bg-[var(--dm-accent-muted)] text-[var(--dm-accent)] border-2 border-[var(--dm-accent-border)]",
        BadgeColor::Green  => "bg-[var(--dm-confirmed-muted)] text-[var(--dm-confirmed-text)] border-2 border-[var(--dm-confirmed-border)]",
        BadgeColor::Yellow => "bg-[var(--dm-inferred-muted)] text-[var(--dm-inferred-text)] border-2 border-[var(--dm-inferred-border)]",
        BadgeColor::Red    => "bg-[var(--dm-unknown-muted)] text-[var(--dm-unknown-text)] border-2 border-[var(--dm-unknown-border)]",
        BadgeColor::Purple => "bg-[var(--dm-purple-muted)] text-[var(--dm-purple)] border-2 border-[var(--dm-purple-border)]",
    };

    view! {
        <span class=format!(
            "inline-flex items-center px-2 py-0.5 text-xs font-medium \
             rounded-md border {} {}",
            color_classes, class
        )>
            {children()}
        </span>
    }
}

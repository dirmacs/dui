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
        BadgeColor::Gray   => "bg-dm-elevated text-dm-muted border-dm-strong",
        BadgeColor::Blue   => "bg-blue-500/10 text-blue-400 border-blue-400/20",
        BadgeColor::Green  => "bg-emerald-500/10 text-emerald-400 border-emerald-400/20",
        BadgeColor::Yellow => "bg-yellow-500/10 text-yellow-400 border-yellow-400/20",
        BadgeColor::Red    => "bg-red-500/10 text-red-400 border-red-400/20",
        BadgeColor::Purple => "bg-purple-500/10 text-purple-400 border-purple-400/20",
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

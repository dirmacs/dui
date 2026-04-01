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
/// Uses DUI CSS classes: `.dm-badge`, `.dm-badge-gray`, etc.
/// No Tailwind required.
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
    let color_class = match color {
        BadgeColor::Gray   => "dm-badge-gray",
        BadgeColor::Blue   => "dm-badge-blue",
        BadgeColor::Green  => "dm-badge-green",
        BadgeColor::Yellow => "dm-badge-yellow",
        BadgeColor::Red    => "dm-badge-red",
        BadgeColor::Purple => "dm-badge-purple",
    };

    view! {
        <span class=format!("dm-badge {} {}", color_class, class)>
            {children()}
        </span>
    }
}

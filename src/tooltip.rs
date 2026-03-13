//! Tooltip — hover popup with arrow, positioned above the target.

use leptos::prelude::*;

/// Tooltip position relative to the trigger element.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// A hover-triggered tooltip that displays a small text popup near its children.
///
/// Wraps children in a `<div>` with relative positioning. On hover, the tooltip
/// appears with a fade-in animation and a small arrow pointing to the trigger.
#[component]
pub fn Tooltip(
    /// Tooltip text.
    text: String,
    /// Position relative to the trigger.
    #[prop(default = TooltipPosition::Top)]
    position: TooltipPosition,
    /// Trigger element(s).
    children: Children,
) -> impl IntoView {
    let hovered = RwSignal::new(false);

    let (tooltip_pos, arrow_pos) = match position {
        TooltipPosition::Top => (
            "bottom-full left-1/2 -translate-x-1/2 mb-2",
            "top-full left-1/2 -translate-x-1/2 border-l-4 border-r-4 border-t-4 \
             border-l-transparent border-r-transparent border-t-dm-elevated",
        ),
        TooltipPosition::Bottom => (
            "top-full left-1/2 -translate-x-1/2 mt-2",
            "bottom-full left-1/2 -translate-x-1/2 border-l-4 border-r-4 border-b-4 \
             border-l-transparent border-r-transparent border-b-dm-elevated",
        ),
        TooltipPosition::Left => (
            "right-full top-1/2 -translate-y-1/2 mr-2",
            "left-full top-1/2 -translate-y-1/2 border-t-4 border-b-4 border-l-4 \
             border-t-transparent border-b-transparent border-l-dm-elevated",
        ),
        TooltipPosition::Right => (
            "left-full top-1/2 -translate-y-1/2 ml-2",
            "right-full top-1/2 -translate-y-1/2 border-t-4 border-b-4 border-r-4 \
             border-t-transparent border-b-transparent border-r-dm-elevated",
        ),
    };

    view! {
        <div
            class="relative inline-flex"
            on:mouseenter=move |_| hovered.set(true)
            on:mouseleave=move |_| hovered.set(false)
        >
            {children()}

            <Show when=move || hovered.get()>
                <div
                    role="tooltip"
                    class=format!(
                        "absolute {} px-2.5 py-1.5 text-xs font-medium text-dm-text \
                         bg-dm-elevated border border-dm rounded-md shadow-lg \
                         whitespace-nowrap pointer-events-none animate-dm-fade-in",
                        tooltip_pos
                    )
                    style=format!("z-index: var(--dm-z-tooltip);")
                >
                    {text.clone()}
                    // Arrow
                    <span class=format!("absolute w-0 h-0 {}", arrow_pos)></span>
                </div>
            </Show>
        </div>
    }
}

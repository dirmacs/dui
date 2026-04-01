//! Tooltip — hover popup positioned near the target.

use leptos::prelude::*;

/// Tooltip position relative to the trigger element.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
}

/// A hover-triggered tooltip that displays a small text popup near its children.
///
/// Uses DUI CSS classes: `.dm-tooltip-wrapper`, `.dm-tooltip`, `.dm-tooltip-top/bottom`.
/// No Tailwind required.
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

    let pos_class = match position {
        TooltipPosition::Top => "dm-tooltip-top",
        TooltipPosition::Bottom => "dm-tooltip-bottom",
    };

    view! {
        <div
            class="dm-tooltip-wrapper"
            on:mouseenter=move |_| hovered.set(true)
            on:mouseleave=move |_| hovered.set(false)
        >
            {children()}
            <Show when=move || hovered.get()>
                <div class=format!("dm-tooltip {}", pos_class)>
                    {text.clone()}
                </div>
            </Show>
        </div>
    }
}

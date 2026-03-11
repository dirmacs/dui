//! Skeleton — loading placeholder with shimmer animation.

use leptos::prelude::*;

/// A shimmer-animated placeholder block for content that is still loading.
///
/// Use Tailwind dimension classes to control size: `"h-4 w-48"`, `"h-8 w-full"`, etc.
/// Multiple skeletons can be stacked to simulate text paragraphs or card layouts.
#[component]
pub fn Skeleton(
    /// Tailwind classes controlling dimensions and shape.
    /// Examples: "h-4 w-3/4", "h-10 w-full rounded-full", "h-32 w-full"
    #[prop(default = "h-4 w-full")]
    class: &'static str,
    /// Whether to use a circular shape (for avatars).
    #[prop(default = false)]
    circle: bool,
) -> impl IntoView {
    view! {
        <div class=format!(
            "dm-skeleton {}{}",
            if circle { "rounded-full " } else { "" },
            class
        )></div>
    }
}

/// A pre-built skeleton group that mimics a text block (e.g. a card body loading).
#[component]
pub fn SkeletonText(
    /// Number of lines.
    #[prop(default = 3)]
    lines: usize,
) -> impl IntoView {
    view! {
        <div class="space-y-3">
            {(0..lines).map(|i| {
                let width = match i % 3 {
                    0 => "w-full",
                    1 => "w-5/6",
                    _ => "w-2/3",
                };
                view! { <div class=format!("dm-skeleton h-4 {}", width)></div> }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// A pre-built skeleton that mimics a card (image + text).
#[component]
pub fn SkeletonCard() -> impl IntoView {
    view! {
        <div class="bg-dm-panel border border-dm rounded-xl overflow-hidden">
            <div class="dm-skeleton h-40 w-full rounded-none"></div>
            <div class="p-4 space-y-3">
                <div class="dm-skeleton h-5 w-2/3"></div>
                <div class="dm-skeleton h-4 w-full"></div>
                <div class="dm-skeleton h-4 w-4/5"></div>
            </div>
        </div>
    }
}

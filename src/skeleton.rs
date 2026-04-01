//! Skeleton — loading placeholder with shimmer animation.

use leptos::prelude::*;

/// A shimmer-animated placeholder block for content that is still loading.
///
/// Uses DUI CSS class: `.dm-skeleton`. Control size via `style` prop or extra classes.
/// No Tailwind required.
#[component]
pub fn Skeleton(
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
    /// Inline style for dimensions (e.g. "height:16px;width:100%").
    #[prop(default = "height:16px;width:100%")]
    style: &'static str,
    /// Whether to use a circular shape (for avatars).
    #[prop(default = false)]
    circle: bool,
) -> impl IntoView {
    view! {
        <div
            class=format!("dm-skeleton {} {}", if circle { "dm-rounded-full" } else { "" }, class)
            style=style
        ></div>
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
        <div class="dm-flex dm-flex-col dm-gap-3">
            {(0..lines).map(|i| {
                let width = match i % 3 {
                    0 => "100%",
                    1 => "83%",
                    _ => "66%",
                };
                view! { <div class="dm-skeleton" style=format!("height:16px;width:{}", width)></div> }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// A pre-built skeleton that mimics a card (image + text).
#[component]
pub fn SkeletonCard() -> impl IntoView {
    view! {
        <div class="dm-card dm-overflow-hidden">
            <div class="dm-skeleton" style="height:160px;width:100%;border-radius:0"></div>
            <div class="dm-p-4 dm-flex dm-flex-col dm-gap-3">
                <div class="dm-skeleton" style="height:20px;width:66%"></div>
                <div class="dm-skeleton" style="height:16px;width:100%"></div>
                <div class="dm-skeleton" style="height:16px;width:80%"></div>
            </div>
        </div>
    }
}

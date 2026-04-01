//! Divider — horizontal or vertical separator line with optional label.

use leptos::prelude::*;

/// Divider orientation.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// A visual separator line (horizontal or vertical) with an optional text label.
///
/// Uses DUI CSS classes: `.dm-divider`, `.dm-divider-vertical`.
/// No Tailwind required.
#[component]
pub fn Divider(
    /// Horizontal (default) or vertical orientation.
    #[prop(default = DividerOrientation::Horizontal)]
    orientation: DividerOrientation,
    /// Optional label text centered on the divider (horizontal only).
    #[prop(optional, into)]
    label: Option<String>,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    match orientation {
        DividerOrientation::Vertical => {
            view! {
                <div
                    class=format!("dm-divider-vertical {}", class)
                    role="separator"
                    aria-orientation="vertical"
                ></div>
            }
            .into_any()
        }
        DividerOrientation::Horizontal => {
            match label {
                Some(text) => {
                    view! {
                        <div
                            class=format!("dm-flex dm-items-center dm-w-full {}", class)
                            role="separator"
                            aria-orientation="horizontal"
                        >
                            <div class="dm-divider dm-grow"></div>
                            <span class="dm-text-xs dm-text-muted dm-px-3 dm-shrink-0">{text}</span>
                            <div class="dm-divider dm-grow"></div>
                        </div>
                    }
                    .into_any()
                }
                None => {
                    view! {
                        <div
                            class=format!("dm-divider dm-w-full {}", class)
                            role="separator"
                            aria-orientation="horizontal"
                        ></div>
                    }
                    .into_any()
                }
            }
        }
    }
}

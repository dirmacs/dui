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
/// When a label is provided (horizontal only), the divider renders as two lines
/// with centered text between them: `——— Label ———`.
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
                    class=format!("w-px bg-dm-border self-stretch {}", class)
                    role="separator"
                    aria-orientation="vertical"
                ></div>
            }
            .into_any()
        }
        DividerOrientation::Horizontal => {
            match label {
                Some(text) => {
                    // Line + label + line pattern
                    view! {
                        <div
                            class=format!("flex items-center w-full {}", class)
                            role="separator"
                            aria-orientation="horizontal"
                        >
                            <div class="flex-1 h-px bg-dm-border"></div>
                            <span class="text-xs text-dm-muted px-3 shrink-0">{text}</span>
                            <div class="flex-1 h-px bg-dm-border"></div>
                        </div>
                    }
                    .into_any()
                }
                None => {
                    view! {
                        <div
                            class=format!("h-px w-full bg-dm-border {}", class)
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

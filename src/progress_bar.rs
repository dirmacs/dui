//! ProgressBar — horizontal fill bar with label and percentage.

use leptos::prelude::*;

/// A horizontal progress bar with animated fill and optional label.
///
/// Uses DUI CSS classes: `.dm-progress`, `.dm-progress-fill`.
/// No Tailwind required.
#[component]
pub fn ProgressBar(
    /// Progress value from 0.0 to 100.0.
    #[prop(into)]
    value: Signal<f64>,
    /// Optional label displayed above the bar (e.g. "Uploading...").
    #[prop(optional)]
    label: Option<String>,
    /// Whether to show the percentage text.
    #[prop(default = true)]
    show_percentage: bool,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let clamped = move || value.get().clamp(0.0, 100.0);

    let fill_color = move || {
        let v = clamped();
        if v >= 80.0 {
            "var(--dm-confirmed)"
        } else if v >= 50.0 {
            "var(--dm-accent-hover)"
        } else {
            "var(--dm-accent)"
        }
    };

    view! {
        <div class=format!("dm-w-full {}", class)>
            // Label + percentage row
            {(label.is_some() || show_percentage).then(|| view! {
                <div class="dm-flex dm-items-center dm-justify-between dm-mb-2">
                    {label.clone().map(|l| view! {
                        <span class="dm-text-sm dm-font-medium dm-text-secondary">{l}</span>
                    })}
                    {show_percentage.then(|| view! {
                        <span class="dm-text-xs dm-font-mono dm-text-dim">
                            {move || format!("{:.0}%", clamped())}
                        </span>
                    })}
                </div>
            })}

            // Track
            <div
                role="progressbar"
                aria-valuenow=move || format!("{:.0}", clamped())
                aria-valuemin="0"
                aria-valuemax="100"
                class="dm-progress"
            >
                <div
                    class="dm-progress-fill"
                    style=move || format!("width:{}%;background:{}", clamped(), fill_color())
                ></div>
            </div>
        </div>
    }
}

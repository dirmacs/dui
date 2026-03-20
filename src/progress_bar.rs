//! ProgressBar — horizontal fill bar with label and percentage.

use leptos::prelude::*;

/// A horizontal progress bar with animated fill and optional label.
///
/// The fill color changes as the bar approaches 100%:
/// - 0-50%: accent blue
/// - 50-80%: accent-2 (lighter blue)
/// - 80-100%: success green
///
/// This gives an organic "system warming up" feel.
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
    /// Height class for the bar track (e.g. "h-2", "h-3").
    #[prop(default = "h-2")]
    height: &'static str,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let clamped = move || value.get().clamp(0.0, 100.0);

    let fill_color = move || {
        let v = clamped();
        if v >= 80.0 {
            "bg-[var(--dm-confirmed)]"
        } else if v >= 50.0 {
            "bg-[var(--dm-accent-muted)]"
        } else {
            "bg-[var(--dm-accent)]"
        }
    };

    view! {
        <div class=format!("w-full {}", class)>
            // Label + percentage row
            {(label.is_some() || show_percentage).then(|| view! {
                <div class="flex items-center justify-between mb-1.5">
                    {label.clone().map(|l| view! {
                        <span class="text-sm font-medium text-[var(--dm-text-secondary)]">{l}</span>
                    })}
                    {show_percentage.then(|| view! {
                        <span class="text-xs font-mono text-[var(--dm-text-dim)]">
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
                class=format!(
                    "w-full bg-[var(--dm-bg)] border border-[var(--dm-border)] rounded-full overflow-hidden {}",
                    height
                )
            >
                // Fill
                <div
                    class=move || format!(
                        "h-full rounded-full transition-all duration-500 ease-out {}",
                        fill_color()
                    )
                    style=move || format!("width: {}%", clamped())
                ></div>
            </div>
        </div>
    }
}

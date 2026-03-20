use leptos::prelude::*;

/// Animated circular score ring using SVG.
///
/// Color-coded: green (>70), yellow (40-70), red (<40).
/// Uses SVG viewBox for clean mobile scaling.
///
/// # Example
/// ```rust
/// view! { <ScoreRing score=68 size=140 label="Overall" /> }
/// ```
#[component]
pub fn ScoreRing(
    /// Score value (0-100).
    score: u32,
    /// Size in pixels (width and height).
    #[prop(default = 120)]
    size: u32,
    /// Label displayed below the score.
    #[prop(optional)]
    label: Option<String>,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let clamped = score.min(100);
    let radius = 45.0_f64;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let offset = circumference * (1.0 - clamped as f64 / 100.0);
    let color = if clamped > 70 {
        "var(--dm-confirmed)"
    } else if clamped >= 40 {
        "var(--dm-inferred)"
    } else {
        "var(--dm-unknown)"
    };
    let size_str = format!("{}px", size);
    view! {
        <div class=format!("inline-flex flex-col items-center gap-1 {}", class) role="img" aria-label=format!("Score: {} out of 100{}", clamped, label.as_ref().map(|l| format!(" for {}", l)).unwrap_or_default()) >
            <svg viewBox="0 0 100 100" style=format!("width: {}; height: {};", size_str, size_str) >
                <circle cx="50" cy="50" r=format!("{}", radius) fill="none" stroke="var(--dm-border)" stroke-width="8" opacity="0.3" />
                <circle cx="50" cy="50" r=format!("{}", radius) fill="none" stroke=color stroke-width="8" stroke-linecap="round" stroke-dasharray=format!("{}", circumference) stroke-dashoffset=format!("{}", offset) transform="rotate(-90 50 50)" style="transition: stroke-dashoffset 0.8s ease-out;" />
                <text x="50" y="50" text-anchor="middle" dominant-baseline="central" fill="var(--dm-text)" font-size="24" font-weight="bold" >
                    {format!("{}", clamped)}
                </text>
            </svg>
            {label.as_ref().map(|l| view! {
                <span class="font-mono uppercase text-xs font-medium" style="color: var(--dm-text-secondary);" >
                    {l.clone()}
                </span>
            })}
        </div>
    }
}
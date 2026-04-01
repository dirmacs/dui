//! StatusBadge — healthy / degraded / down / unknown with animated indicator dot.

use leptos::prelude::*;

/// System status level.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Status {
    Healthy,
    Degraded,
    Down,
    #[default]
    Unknown,
}

impl Status {
    /// Human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            Status::Healthy  => "Healthy",
            Status::Degraded => "Degraded",
            Status::Down     => "Down",
            Status::Unknown  => "Unknown",
        }
    }
}

/// A status indicator with a pulsing/static colored dot and label.
///
/// Uses DUI CSS: inline styles with DUI CSS variables. No Tailwind required.
#[component]
pub fn StatusBadge(
    /// The status to display.
    status: Signal<Status>,
    /// Optional override label (defaults to status name).
    #[prop(optional)]
    label: Option<String>,
) -> impl IntoView {
    let dot_style = move || match status.get() {
        Status::Healthy  => "background:var(--dm-confirmed);animation:dm-pulse 2s cubic-bezier(0.4,0,0.6,1) infinite",
        Status::Degraded => "background:var(--dm-inferred)",
        Status::Down     => "background:var(--dm-unknown)",
        Status::Unknown  => "background:var(--dm-text-dim)",
    };

    let text_style = move || match status.get() {
        Status::Healthy  => "color:var(--dm-confirmed-text)",
        Status::Degraded => "color:var(--dm-inferred-text)",
        Status::Down     => "color:var(--dm-unknown-text)",
        Status::Unknown  => "color:var(--dm-text-muted)",
    };

    let badge_style = move || match status.get() {
        Status::Healthy  => "background:var(--dm-confirmed-muted);border-color:var(--dm-confirmed-border)",
        Status::Degraded => "background:var(--dm-inferred-muted);border-color:var(--dm-inferred-border)",
        Status::Down     => "background:var(--dm-unknown-muted);border-color:var(--dm-unknown-border)",
        Status::Unknown  => "background:var(--dm-elevated);border-color:var(--dm-border)",
    };

    view! {
        <span
            class="dm-badge dm-rounded-full"
            style=badge_style
        >
            <span
                style=move || format!("width:8px;height:8px;border-radius:50%;flex-shrink:0;{}", dot_style())
            ></span>
            <span style=text_style>
                {move || label.clone().unwrap_or_else(|| status.get().label().to_string())}
            </span>
        </span>
    }
}

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
/// Healthy = green pulsing dot (the system breathes).
/// Degraded = yellow static dot.
/// Down = red static dot.
/// Unknown = gray static dot.
#[component]
pub fn StatusBadge(
    /// The status to display.
    status: Signal<Status>,
    /// Optional override label (defaults to status name).
    #[prop(optional)]
    label: Option<String>,
) -> impl IntoView {
    let dot_class = move || match status.get() {
        Status::Healthy  => "bg-dm-success animate-dm-pulse",
        Status::Degraded => "bg-dm-warning",
        Status::Down     => "bg-dm-danger",
        Status::Unknown  => "bg-dm-dim",
    };

    let text_class = move || match status.get() {
        Status::Healthy  => "text-dm-success",
        Status::Degraded => "text-dm-warning",
        Status::Down     => "text-dm-danger",
        Status::Unknown  => "text-dm-muted",
    };

    let bg_class = move || match status.get() {
        Status::Healthy  => "bg-dm-success/10 border-dm-success/20",
        Status::Degraded => "bg-dm-warning/10 border-dm-warning/20",
        Status::Down     => "bg-dm-danger/10 border-dm-danger/20",
        Status::Unknown  => "bg-dm-elevated border-dm",
    };

    view! {
        <span class=move || format!(
            "inline-flex items-center gap-2 px-2.5 py-1 text-xs font-medium \
             rounded-full border {}",
            bg_class()
        )>
            <span class=move || format!("w-2 h-2 rounded-full shrink-0 {}", dot_class())></span>
            <span class=text_class>
                {move || label.clone().unwrap_or_else(|| status.get().label().to_string())}
            </span>
        </span>
    }
}

//! AlertBanner — info / warning / error / success with icon and dismiss button.

use leptos::prelude::*;

/// Alert severity level.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AlertLevel {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// A horizontal alert banner with icon, message, and optional dismiss button.
#[component]
pub fn AlertBanner(
    /// Severity level — controls color and icon.
    #[prop(default = AlertLevel::Info)]
    level: AlertLevel,
    /// The alert message.
    message: String,
    /// Whether the banner can be dismissed.
    #[prop(default = true)]
    dismissible: bool,
    /// Controls visibility — set to false to hide.
    #[prop(into)]
    visible: RwSignal<bool>,
) -> impl IntoView {
    let (bg, border, text, icon_path) = match level {
        AlertLevel::Info => (
            "bg-dm-info/10",
            "border-dm-info/20",
            "text-dm-info",
            "m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z",
        ),
        AlertLevel::Success => (
            "bg-dm-success/10",
            "border-dm-success/20",
            "text-dm-success",
            "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        ),
        AlertLevel::Warning => (
            "bg-dm-warning/10",
            "border-dm-warning/20",
            "text-dm-warning",
            "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z",
        ),
        AlertLevel::Error => (
            "bg-dm-danger/10",
            "border-dm-danger/20",
            "text-dm-danger",
            "M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z",
        ),
    };

    view! {
        <Show when=move || visible.get()>
            <div class=format!(
                "flex items-center gap-3 px-4 py-3 rounded-lg border \
                 animate-dm-fade-in-up {} {}",
                bg, border
            )>
                // Icon
                <svg class=format!("w-5 h-5 shrink-0 {}", text)
                     xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                     stroke-width="1.5" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                </svg>

                // Message
                <p class=format!("flex-1 text-sm {}", text)>{message.clone()}</p>

                // Dismiss
                {dismissible.then(|| view! {
                    <button
                        class=format!(
                            "p-1 rounded-md hover:bg-white/5 transition-colors {} shrink-0",
                            text
                        )
                        on:click=move |_| visible.set(false)
                    >
                        <svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" fill="none"
                             viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                        </svg>
                    </button>
                })}
            </div>
        </Show>
    }
}

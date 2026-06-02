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
///
/// Uses DUI CSS classes: `.dm-alert`, `.dm-alert-info/success/warning/error`, `.dm-alert-dismiss`.
/// No Tailwind required.
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
    let (level_class, icon_path) = match level {
        AlertLevel::Info    => ("dm-alert-info", "m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z"),
        AlertLevel::Success => ("dm-alert-success", "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"),
        AlertLevel::Warning => ("dm-alert-warning", "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z"),
        AlertLevel::Error   => ("dm-alert-error", "M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"),
    };

    view! {
        <Show when=move || visible.try_get().unwrap_or(false)>
            <div class=format!("dm-alert {}", level_class) role="alert">
                <svg style="width:18px;height:18px;flex-shrink:0" xmlns="http://www.w3.org/2000/svg"
                     fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                </svg>
                <span class="dm-text-sm" style="flex:1;line-height:1.4">{message.clone()}</span>
                {dismissible.then(|| view! {
                    <button
                        class="dm-alert-dismiss"
                        on:click=move |_| visible.set(false)
                        aria-label="Dismiss"
                    >
                        <svg style="width:16px;height:16px" xmlns="http://www.w3.org/2000/svg" fill="none"
                             viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                        </svg>
                    </button>
                })}
            </div>
        </Show>
    }
}

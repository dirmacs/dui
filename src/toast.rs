//! Toast — auto-dismissing notification system.

use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct ToastData {
    pub id: u64,
    pub level: ToastLevel,
    pub message: String,
    pub duration_ms: u32,
}

#[derive(Clone)]
pub struct ToastState {
    pub items: RwSignal<Vec<ToastData>>,
    next_id: RwSignal<u64>,
}

impl ToastState {
    pub fn new() -> Self {
        Self {
            items: RwSignal::new(Vec::new()),
            next_id: RwSignal::new(1),
        }
    }

    pub fn push(&self, level: ToastLevel, message: impl Into<String>) {
        self.push_with_duration(level, message, 4000);
    }

    pub fn push_with_duration(
        &self,
        level: ToastLevel,
        message: impl Into<String>,
        duration_ms: u32,
    ) {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        let toast = ToastData {
            id,
            level,
            message: message.into(),
            duration_ms,
        };
        self.items.update(|list| list.push(toast));
        let items = self.items;
        gloo_timers::callback::Timeout::new(duration_ms, move || {
            items.update(|list| list.retain(|t| t.id != id));
        })
        .forget();
    }

    pub fn dismiss(&self, id: u64) {
        self.items.update(|list| list.retain(|t| t.id != id));
    }
}

/// Creates a ToastState, provides it as context, and returns it.
pub fn provide_toast() -> ToastState {
    let state = ToastState::new();
    provide_context(state.clone());
    state
}

/// Renders all active toasts. Place once in your app root.
///
/// Uses DUI CSS classes: `.dm-toast-container`, `.dm-toast`, `.dm-toast-info/success/warning/error`.
/// No Tailwind required.
#[component]
pub fn ToastContainer() -> impl IntoView {
    let state = use_context::<ToastState>();

    view! {
        {move || {
            let state = match &state {
                Some(s) => s.clone(),
                None => return view! { <div></div> }.into_any(),
            };
            let toasts = state.items.get();
            if toasts.is_empty() {
                return view! { <div></div> }.into_any();
            }
            view! {
                <div class="dm-toast-container">
                    {toasts.iter().map(|toast| {
                        let id = toast.id;
                        let items = state.items;
                        let (level_class, icon_path) = match toast.level {
                            ToastLevel::Info    => ("dm-toast-info", "m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z"),
                            ToastLevel::Success => ("dm-toast-success", "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"),
                            ToastLevel::Warning => ("dm-toast-warning", "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z"),
                            ToastLevel::Error   => ("dm-toast-error", "M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"),
                        };
                        view! {
                            <div class=format!("dm-toast {}", level_class)>
                                <svg class="dm-toast-icon" xmlns="http://www.w3.org/2000/svg"
                                     fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                                </svg>
                                <p class="dm-toast-message">{toast.message.clone()}</p>
                                <button
                                    class="dm-toast-dismiss"
                                    on:click=move |_| { items.update(|list| list.retain(|t| t.id != id)); }
                                >
                                    <svg style="width:16px;height:16px" xmlns="http://www.w3.org/2000/svg" fill="none"
                                         viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                                    </svg>
                                </button>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }.into_any()
        }}
    }
}

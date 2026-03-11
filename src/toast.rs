//! Toast — auto-dismissing notification.

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
}

impl ToastState {
    pub fn new() -> Self {
        Self {
            items: RwSignal::new(Vec::new()),
        }
    }

    pub fn push(&self, _level: ToastLevel, _message: impl Into<String>) {}

    pub fn push_with_duration(
        &self,
        _level: ToastLevel,
        _message: impl Into<String>,
        _duration_ms: u32,
    ) {
    }

    pub fn dismiss(&self, _id: u64) {}
}

#[component]
pub fn ToastContainer() -> impl IntoView {
    view! {
        <div></div>
    }
}

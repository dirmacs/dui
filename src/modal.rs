//! Modal — overlay dialog.

use leptos::prelude::*;

#[component]
pub fn Modal(
    #[prop(into)] _open: Signal<bool>,
    #[prop(default = "")] _title: &'static str,
    #[prop(default = "max-w-lg")] _max_width: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        {children()}
    }
}

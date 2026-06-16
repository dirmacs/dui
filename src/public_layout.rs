//! PublicLayout — page shell for public-facing (unauthenticated) pages.

use leptos::prelude::*;

/// Public page shell: wraps content with proper spacing for fixed navbar.
/// Use with Navbar + Footer outside. No Tailwind required.
#[component]
pub fn PublicLayout(
    children: Children,
    #[prop(default = "")] class: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("dm-min-h-screen dm-bg dm-text-primary {}", class)>
            <main>{children()}</main>
        </div>
    }
}

//! Auth guard component — wraps a DIRMACS frontend with authentication.
//!
//! If the user has a valid JWT in localStorage, renders children.
//! Otherwise, renders the fallback (typically a LoginPage).

use leptos::prelude::*;
use super::state::{is_authenticated, AuthConfig};
use super::login::LoginPage;

/// Auth guard that gates access to a DIRMACS frontend.
///
/// Checks localStorage for a JWT token. If present, renders children.
/// If absent, renders a login page.
///
/// After successful login, the page reloads and the guard passes.
///
/// # Example
///
/// ```rust,ignore
/// <AuthGuard config=my_config>
///     <MyDashboard />
/// </AuthGuard>
/// ```
#[component]
pub fn AuthGuard(
    config: AuthConfig,
    children: Children,
) -> impl IntoView {
    let (authed, set_authed) = signal(is_authenticated());

    let on_login = Callback::new(move |_: ()| {
        set_authed.set(true);
    });

    let config_clone = config.clone();

    view! {
        {move || {
            if authed.get() {
                children().into_any()
            } else {
                let cfg = config_clone.clone();
                view! {
                    <LoginPage config=cfg on_success=Some(on_login) />
                }.into_any()
            }
        }}
    }
}

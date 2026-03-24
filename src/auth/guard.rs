//! Auth guard component — wraps a DIRMACS frontend with authentication.

use leptos::prelude::*;
use super::state::{is_authenticated, AuthConfig};
use super::login::LoginPage;

/// Auth guard that gates access to a DIRMACS frontend.
///
/// Checks localStorage for a JWT. If present, renders children.
/// If absent, renders a login page.
#[component]
pub fn AuthGuard(
    config: AuthConfig,
    children: Children,
) -> impl IntoView {
    let (authed, set_authed) = signal(is_authenticated());

    if authed.get_untracked() {
        children().into_any()
    } else {
        view! {
            <LoginPage config=config on_success=set_authed />
        }.into_any()
    }
}

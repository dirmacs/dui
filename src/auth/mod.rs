//! Shared authentication for all DIRMACS frontends.
//!
//! - `AuthGuard` — wraps your app, shows login if no JWT
//! - `LoginPage` — email/password login against Eruka
//! - `auth_state()` / `logout()` — JWT management

mod guard;
mod login;
mod state;

pub use guard::AuthGuard;
pub use login::LoginPage;
pub use state::{
    get_auth_state, get_token, get_user, is_authenticated, logout, store_auth, AuthConfig,
    AuthState, UserInfo,
};

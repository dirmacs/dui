//! Shared authentication for all DIRMACS frontends.
//!
//! Provides:
//! - `AuthGuard` — wraps your app, redirects to login if no valid JWT
//! - `LoginPage` — email/password login form that authenticates against Eruka
//! - `auth_state()` / `logout()` — JWT management in localStorage
//!
//! # Usage
//!
//! ```rust,ignore
//! use dui::auth::{AuthGuard, LoginPage, AuthConfig};
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let config = AuthConfig {
//!         eruka_url: "https://eruka.dirmacs.com".to_string(),
//!         product_name: "DolTARES".to_string(),
//!         product_subtitle: "Orchestration Dashboard".to_string(),
//!     };
//!
//!     view! {
//!         <AuthGuard config=config.clone() fallback=move || view! { <LoginPage config=config.clone() /> }>
//!             <MyDashboard />
//!         </AuthGuard>
//!     }
//! }
//! ```

mod guard;
mod login;
mod state;

pub use guard::AuthGuard;
pub use login::LoginPage;
pub use state::{
    AuthConfig, AuthState, UserInfo,
    get_auth_state, is_authenticated, get_token, get_user, logout, store_auth,
};

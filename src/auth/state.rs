//! Auth state management — JWT storage in localStorage.

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const TOKEN_KEY: &str = "dirmacs_jwt";
const USER_KEY: &str = "dirmacs_user";

/// Configuration for the auth system.
#[derive(Clone, Debug)]
pub struct AuthConfig {
    /// Eruka API base URL (e.g., "https://eruka.dirmacs.com")
    pub eruka_url: String,
    /// Product name shown on login page (e.g., "DolTARES")
    pub product_name: String,
    /// Product subtitle (e.g., "Orchestration Dashboard")
    pub product_subtitle: String,
}

/// Stored user info from login response.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
}

/// Complete auth state.
#[derive(Clone, Debug)]
pub struct AuthState {
    pub token: Option<String>,
    pub user: Option<UserInfo>,
}

/// Get the current auth state from localStorage.
pub fn get_auth_state() -> AuthState {
    AuthState {
        token: get_token(),
        user: get_user(),
    }
}

/// Check if a valid JWT exists in localStorage.
/// Does NOT check expiry (that requires decoding which needs the secret).
/// For client-side, we trust the token until the server rejects it.
pub fn is_authenticated() -> bool {
    get_token().is_some()
}

/// Get the stored JWT token.
pub fn get_token() -> Option<String> {
    LocalStorage::get::<String>(TOKEN_KEY).ok()
}

/// Get the stored user info.
pub fn get_user() -> Option<UserInfo> {
    LocalStorage::get::<UserInfo>(USER_KEY).ok()
}

/// Store auth credentials after successful login.
pub fn store_auth(token: &str, user: &UserInfo) {
    let _ = LocalStorage::set(TOKEN_KEY, token.to_string());
    let _ = LocalStorage::set(USER_KEY, user.clone());
}

/// Clear all auth state (logout).
pub fn logout() {
    let _ = LocalStorage::delete(TOKEN_KEY);
    let _ = LocalStorage::delete(USER_KEY);
}

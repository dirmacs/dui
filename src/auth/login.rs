//! Login page component — authenticates against Eruka.
//!
//! Reusable across all DIRMACS frontends. Uses DUI components
//! (Card, Input, Button, AlertBanner) for consistent styling.

use leptos::prelude::*;
use super::state::{AuthConfig, UserInfo, store_auth};

/// Response from Eruka login endpoint.
#[derive(Clone, Debug, serde::Deserialize)]
struct LoginResponse {
    token: String,
    user: LoginUser,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct LoginUser {
    id: String,
    email: String,
    name: Option<String>,
}

/// Shared login page for all DIRMACS products.
///
/// Authenticates against Eruka `/api/v1/auth/login`.
/// On success, stores JWT + user info in localStorage and calls `on_success`.
#[component]
pub fn LoginPage(
    config: AuthConfig,
    #[prop(optional)] on_success: Option<Callback<()>>,
) -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());
    let loading = RwSignal::new(false);
    let eruka_url = config.eruka_url.clone();

    let handle_login = move |_: web_sys::MouseEvent| {
        let em = email.get();
        let pw = password.get();
        if em.is_empty() || pw.is_empty() {
            error.set("Please enter email and password".to_string());
            return;
        }
        if loading.get() { return; }
        loading.set(true);
        error.set(String::new());

        let url = eruka_url.clone();
        let on_success = on_success.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let login_url = format!("{}/api/v1/auth/login", url);
            let body = serde_json::json!({"email": em, "password": pw});

            match gloo_net::http::Request::post(&login_url)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&body).unwrap_or_default())
                .unwrap()
                .send()
                .await
            {
                Ok(resp) if resp.ok() => {
                    match resp.json::<LoginResponse>().await {
                        Ok(data) => {
                            let user = UserInfo {
                                id: data.user.id,
                                email: data.user.email,
                                name: data.user.name,
                            };
                            store_auth(&data.token, &user);
                            if let Some(cb) = on_success {
                                cb.call(());
                            } else {
                                // Reload page to trigger AuthGuard
                                if let Some(window) = web_sys::window() {
                                    let _ = window.location().reload();
                                }
                            }
                        }
                        Err(_) => {
                            error.set("Failed to parse login response".to_string());
                            loading.set(false);
                        }
                    }
                }
                Ok(resp) => {
                    let status = resp.status();
                    let body = resp.text().await.unwrap_or_default();
                    if status == 401 {
                        error.set("Invalid email or password".to_string());
                    } else {
                        error.set(format!("Login failed ({}): {}", status, body));
                    }
                    loading.set(false);
                }
                Err(e) => {
                    error.set(format!("Network error: {}", e));
                    loading.set(false);
                }
            }
        });
    };

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            handle_login(web_sys::MouseEvent::new("click").unwrap());
        }
    };

    view! {
        <div class="dui-auth-login">
            <div class="dui-auth-login-container">
                <div class="dui-auth-login-header">
                    <h1 class="dui-auth-login-title">{config.product_name.clone()}</h1>
                    <p class="dui-auth-login-subtitle">{config.product_subtitle.clone()}</p>
                </div>

                <div class="dui-auth-login-card">
                    <div class="dui-auth-login-fields">
                        <div class="dui-auth-field">
                            <label class="dui-auth-label">"Email"</label>
                            <input
                                type="email"
                                class="dui-auth-input"
                                placeholder="you@dirmacs.com"
                                prop:value=move || email.get()
                                on:input=move |ev| email.set(event_target_value(&ev))
                                on:keydown=handle_keydown.clone()
                            />
                        </div>
                        <div class="dui-auth-field">
                            <label class="dui-auth-label">"Password"</label>
                            <input
                                type="password"
                                class="dui-auth-input"
                                placeholder="Password"
                                prop:value=move || password.get()
                                on:input=move |ev| password.set(event_target_value(&ev))
                                on:keydown=handle_keydown.clone()
                            />
                        </div>

                        {move || {
                            let err = error.get();
                            if err.is_empty() {
                                None
                            } else {
                                Some(view! {
                                    <div class="dui-auth-error">{err}</div>
                                })
                            }
                        }}

                        <button
                            class="dui-auth-button"
                            on:click=handle_login.clone()
                            prop:disabled=move || loading.get()
                        >
                            {move || if loading.get() { "Signing in..." } else { "Sign in" }}
                        </button>
                    </div>
                </div>

                <p class="dui-auth-footer">
                    "Powered by "
                    <a href="https://dirmacs.com" target="_blank" class="dui-auth-link">"DIRMACS"</a>
                </p>
            </div>
        </div>
    }
}

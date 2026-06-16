//! Login page component — authenticates against Eruka.

use super::state::{store_auth, AuthConfig, UserInfo};
use leptos::prelude::*;

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
#[component]
pub fn LoginPage(
    config: AuthConfig,
    #[prop(optional)] on_success: Option<WriteSignal<bool>>,
) -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());
    let loading = RwSignal::new(false);
    let eruka_url = StoredValue::new(config.eruka_url.clone());

    let do_login = move || {
        let em = email.get();
        let pw = password.get();
        if em.is_empty() || pw.is_empty() {
            error.set("Please enter email and password".to_string());
            return;
        }
        if loading.get() {
            return;
        }
        loading.set(true);
        error.set(String::new());

        let url = eruka_url.get_value();
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
                Ok(resp) if resp.ok() => match resp.json::<LoginResponse>().await {
                    Ok(data) => {
                        let user = UserInfo {
                            id: data.user.id,
                            email: data.user.email,
                            name: data.user.name,
                        };
                        store_auth(&data.token, &user);
                        if let Some(setter) = on_success {
                            setter.set(true);
                        } else {
                            if let Some(window) = web_sys::window() {
                                let _ = window.location().reload();
                            }
                        }
                    }
                    Err(_) => {
                        error.set("Failed to parse login response".to_string());
                        loading.set(false);
                    }
                },
                Ok(resp) => {
                    let status = resp.status();
                    if status == 401 {
                        error.set("Invalid email or password".to_string());
                    } else {
                        error.set(format!("Login failed ({})", status));
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

    let do_login_click = do_login.clone();
    let do_login_key = do_login.clone();

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
                                on:keydown=move |ev: web_sys::KeyboardEvent| {
                                    if ev.key() == "Enter" { do_login_key(); }
                                }
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
                                on:keydown=move |ev: web_sys::KeyboardEvent| {
                                    if ev.key() == "Enter" { do_login(); }
                                }
                            />
                        </div>

                        {move || {
                            let err = error.get();
                            if err.is_empty() { None } else {
                                Some(view! { <div class="dui-auth-error">{err}</div> })
                            }
                        }}

                        <button
                            class="dui-auth-button"
                            on:click=move |_| do_login_click()
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

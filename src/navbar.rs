//! Navbar — horizontal top navigation for public-facing pages.
//! Fixed position, transparent → glass blur on scroll, mobile hamburger, dropdown support.
//!
//! Dropdowns are controlled via Leptos signals (not CSS :hover) so they never
//! flash during route transitions or component remounts.

use leptos::prelude::*;

/// A navigation link item.
#[derive(Debug, Clone)]
pub struct NavItem {
    pub label: String,
    pub href: Option<String>,
    pub children: Vec<NavDropdownItem>,
}

impl NavItem {
    pub fn link(label: &str, href: &str) -> Self {
        Self {
            label: label.to_string(),
            href: Some(href.to_string()),
            children: vec![],
        }
    }
    pub fn dropdown(label: &str, children: Vec<NavDropdownItem>) -> Self {
        Self {
            label: label.to_string(),
            href: None,
            children,
        }
    }
    pub fn dropdown_with_link(label: &str, href: &str, children: Vec<NavDropdownItem>) -> Self {
        Self {
            label: label.to_string(),
            href: Some(href.to_string()),
            children,
        }
    }
    pub fn is_dropdown(&self) -> bool {
        !self.children.is_empty()
    }
}

/// A child item inside a dropdown menu.
#[derive(Debug, Clone)]
pub struct NavDropdownItem {
    pub label: String,
    pub description: String,
    pub href: String,
}

impl NavDropdownItem {
    pub fn new(label: &str, description: &str, href: &str) -> Self {
        Self {
            label: label.to_string(),
            description: description.to_string(),
            href: href.to_string(),
        }
    }
}

/// Call-to-action button in the navbar.
#[derive(Debug, Clone)]
pub struct NavCta {
    pub label: String,
    pub href: String,
}

impl NavCta {
    pub fn new(label: &str, href: &str) -> Self {
        Self {
            label: label.to_string(),
            href: href.to_string(),
        }
    }
}

/// Horizontal top navigation bar for public-facing pages.
///
/// Uses DUI CSS classes: `.dm-nav`, `.dm-nav-scrolled`, `.dm-nav-inner`, `.dm-nav-brand`, `.dm-nav-links`, `.dm-nav-cta`, `.dm-nav-mobile`.
/// No Tailwind required.
#[component]
pub fn Navbar(
    brand_name: &'static str,
    #[prop(optional)] brand_logo_url: Option<&'static str>,
    items: Vec<NavItem>,
    #[prop(optional, into)] cta: Option<NavCta>,
    #[prop(default = "")] class: &'static str,
) -> impl IntoView {
    let scrolled = RwSignal::new(false);
    let nav_hidden = RwSignal::new(false);
    let mobile_open = RwSignal::new(false);
    // Non-reactive: tracks previous scroll Y to detect direction.
    let last_y: StoredValue<f64> = StoredValue::new(0.0);

    Effect::new(move |_| {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        let win = web_sys::window().unwrap();
        let cb = Closure::<dyn Fn()>::new(move || {
            let y = web_sys::window().unwrap().scroll_y().unwrap_or(0.0);
            let prev = last_y.get_value();
            scrolled.set(y > 10.0);
            // Always show near the top; hide when scrolling down, show when scrolling up.
            // 4px deadband prevents jitter from scroll bounce.
            if y <= 64.0 {
                nav_hidden.set(false);
            } else if y > prev + 4.0 {
                nav_hidden.set(true);
            } else if y < prev - 4.0 {
                nav_hidden.set(false);
            }
            last_y.set_value(y);
        });
        win.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref())
            .ok();
        cb.forget();
    });

    let items_mobile = items.clone();
    let cta_mobile = cta.clone();

    view! {
        <nav class=move || format!(
            "dm-nav {} {} {}",
            if scrolled.get() { "dm-nav-scrolled" } else { "" },
            if nav_hidden.get() { "dm-nav-hidden" } else { "" },
            class,
        )>
            <div class="dm-nav-inner">
                <a href="/" class="dm-nav-brand">
                    {brand_logo_url.map(|url| view! {
                        <img src=url alt="" style="width:28px;height:28px;border-radius:var(--dm-radius)" />
                    })}
                    <span class="dm-nav-brand-text">{brand_name}</span>
                </a>

                <div class="dm-nav-links dm-md-hidden-up" style="display:none">
                    // Hidden on mobile via CSS
                </div>
                <div class="dm-nav-links">
                    {items.iter().map(|item| {
                        if item.is_dropdown() {
                            let children = item.children.clone();
                            let label = item.label.clone();

                            // Signal-based open state: never driven by CSS :hover so the
                            // dropdown cannot flash during route transitions or remounts.
                            let open = RwSignal::new(false);
                            // Store the raw setTimeout handle (i32 is Send+Sync; -1 = no pending).
                            // gloo_timers::Timeout is !Send so we manage the timer ID directly.
                            let pending_id: StoredValue<i32> = StoredValue::new(-1);

                view! {
                    <div
                        class="dm-nav-dropdown"
                        style="display:inline-block;position:relative"
                        on:mouseenter=move |_| {
                            use wasm_bindgen::closure::Closure;
                            use wasm_bindgen::JsCast;
                            let win = web_sys::window().unwrap();
                            // Cancel any previous pending timer before scheduling a new one.
                            let old = pending_id.get_value();
                            if old >= 0 { win.clear_timeout_with_handle(old); }
                            // Schedule open after 120ms — prevents drive-by cursor flash.
                            // Closure::forget() is safe for one-shot short-lived timers:
                            // the browser holds the ref, fires it once, then GC's it.
                            let cb = Closure::<dyn Fn()>::new(move || open.set(true));
                            let id = win
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    cb.as_ref().unchecked_ref(),
                                    120,
                                )
                                .unwrap_or(-1);
                            cb.forget();
                            pending_id.set_value(id);
                        }
                        on:mouseleave=move |_| {
                            // Cancel pending open timer if still counting down.
                            let id = pending_id.get_value();
                            if id >= 0 {
                                web_sys::window().unwrap().clear_timeout_with_handle(id);
                                pending_id.set_value(-1);
                            }
                            open.set(false);
                        }
                    >
                        {if item.href.is_some() {
                            let href = item.href.clone().unwrap();
                            let label_for_link = label.clone();
                            let toggle_aria = format!("Toggle {} submenu", label);
                            view! {
                                <div style="display:flex;align-items:center;gap:2px;color:var(--dm-text-secondary)">
                                    <a
                                        href=href
                                        class="dm-nav-links dm-cursor-pointer dm-no-underline"
                                        style="display:flex;align-items:center;color:var(--dm-text-secondary);padding-right:2px"
                                    >
                                        {label_for_link}
                                    </a>
                                    <button
                                        type="button"
                                        class="dm-nav-dropdown-toggle"
                                        style="background:none;border:none;cursor:pointer;padding:6px 8px;display:flex;align-items:center;justify-content:center;color:var(--dm-text-secondary)"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            ev.stop_propagation();
                                            open.update(|v| *v = !*v);
                                        }
                                        aria-label=toggle_aria
                                        attr:aria-expanded=move || if open.get() { "true" } else { "false" }
                                        aria-haspopup="menu"
                                    >
                                        <svg style="width:12px;height:12px;opacity:0.5" viewBox="0 0 12 12" fill="currentColor">
                                            <path d="M2.5 4.5L6 8l3.5-3.5" stroke="currentColor" stroke-width="1.5" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
                                        </svg>
                                    </button>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <span class="dm-nav-links dm-cursor-pointer" style="display:flex;align-items:center;gap:4px;color:var(--dm-text-secondary)">
                                    {label}
                                    <svg style="width:12px;height:12px;opacity:0.5" viewBox="0 0 12 12" fill="currentColor">
                                        <path d="M2.5 4.5L6 8l3.5-3.5" stroke="currentColor" stroke-width="1.5" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
                                    </svg>
                                </span>
                            }.into_any()
                        }}
                                    <div
                                        class="dm-dropdown-menu"
                                        style=move || if open.get() {
                                            // Open: 200ms ease-in. animation:none suppresses the
                                            // dm-slide-down CSS class animation which has higher
                                            // cascade priority than inline styles and would flash
                                            // the menu visible on every page mount otherwise.
                                            "position:absolute;top:100%;left:-16px;min-width:280px;padding-top:12px;\
                                             opacity:1;pointer-events:auto;transform:translateY(0);\
                                             transition:opacity 0.2s ease,transform 0.2s ease;animation:none"
                                        } else {
                                            // Closed: 80ms fast-out. animation:none is the critical
                                            // line — without it the CSS animation overrides opacity:0
                                            // and flashes the dropdown on every page load/remount.
                                            "position:absolute;top:100%;left:-16px;min-width:280px;padding-top:12px;\
                                             opacity:0;pointer-events:none;transform:translateY(-8px);\
                                             transition:opacity 0.08s ease,transform 0.08s ease;animation:none"
                                        }
                                    >
                                        {children.iter().map(|child| {
                                            let href = child.href.clone();
                                            let label = child.label.clone();
                                            let desc = child.description.clone();
                                            view! {
                                                <a href=href class="dm-dropdown-item" style="display:block;padding:10px 14px">
                                                    <div class="dm-font-mono dm-font-bold dm-text-xs dm-text-primary" style="letter-spacing:0.04em">{label}</div>
                                                    <div class="dm-text-xs dm-text-muted" style="margin-top:2px">{desc}</div>
                                                </a>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            let href = item.href.clone().unwrap_or_default();
                            let label = item.label.clone();
                            view! {
                                <a href=href class="dm-no-underline dm-text-secondary dm-transition-colors" style="font-family:var(--dm-font-body);font-size:14px;font-weight:500">
                                    {label}
                                </a>
                            }.into_any()
                        }
                    }).collect::<Vec<_>>()}
                </div>

                <div class="dm-nav-right">
                    {cta.map(|c| {
                        let href = c.href.clone();
                        let label = c.label.clone();
                        view! { <a href=href class="dm-nav-cta">{label}</a> }
                    })}
                </div>

                <button
                    class="dm-nav-mobile-toggle"
                    on:click=move |_| mobile_open.update(|v| *v = !*v)
                    aria-label="Menu"
                >
                    <svg style="width:24px;height:24px" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 12h18M3 6h18M3 18h18"/></svg>
                </button>
            </div>
        </nav>

        // Mobile menu
        <Show when=move || mobile_open.get()>
            <div class="dm-nav-mobile open">
                // Header: brand left, X button right
                <div class="dm-nav-mobile-header">
                    <a href="/" class="dm-nav-brand" on:click=move |_| mobile_open.set(false)>
                        {brand_logo_url.map(|url| view! {
                            <img src=url alt="" style="width:24px;height:24px;border-radius:var(--dm-radius)" />
                        })}
                        <span class="dm-nav-brand-text">{brand_name}</span>
                    </a>
                    <button
                        class="dm-nav-mobile-close"
                        on:click=move |_| mobile_open.set(false)
                        aria-label="Close menu"
                    >
                        <svg style="width:24px;height:24px" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            <path d="M18 6L6 18M6 6l12 12" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                    </button>
                </div>
                <div class="dm-nav-mobile-sep"></div>

                // Nav items with expand/collapse for dropdowns
                <div class="dm-nav-mobile-body">
                    {items_mobile.iter().map(|item| {
                        if item.is_dropdown() {
                            let children = item.children.clone();
                            let label = item.label.clone();
                            let item_href = item.href.clone();
                            let expanded = RwSignal::new(false);

                            view! {
                                <div>
                                    <div class="dm-nav-mobile-item">
                                        {if let Some(page_href) = item_href {
                                            view! {
                                                <a href=page_href
                                                    style="flex:1;color:inherit;text-decoration:none;font:inherit"
                                                    on:click=move |_| mobile_open.set(false)
                                                >{label}</a>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <span style="flex:1">{label}</span>
                                            }.into_any()
                                        }}
                                        <button
                                            style="background:none;border:none;cursor:pointer;padding:8px 10px;min-width:40px;display:flex;align-items:center;justify-content:center;color:var(--dm-text-secondary)"
                                            on:click=move |_| expanded.update(|v| *v = !*v)
                                            aria-label="Toggle submenu"
                                        >
                                            {move || if expanded.get() {
                                                view! {
                                                    <svg style="width:18px;height:18px" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                                        <path d="M6 9l6 6 6-6" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <svg style="width:18px;height:18px" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                                        <path d="M9 18l6-6-6-6" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                }.into_any()
                                            }}
                                        </button>
                                    </div>
                                    <Show when=move || expanded.get()>
                                        <div class="dm-nav-mobile-submenu">
                                            {children.iter().map(|child| {
                                                let href = child.href.clone();
                                                let label = child.label.clone();
                                                view! {
                                                    <a href=href class="dm-nav-mobile-subitem" on:click=move |_| mobile_open.set(false)>{label}</a>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </Show>
                                </div>
                            }.into_any()
                        } else {
                            let href = item.href.clone().unwrap_or_default();
                            let label = item.label.clone();
                            view! {
                                <a href=href class="dm-nav-mobile-item" on:click=move |_| mobile_open.set(false)>{label}</a>
                            }.into_any()
                        }
                    }).collect::<Vec<_>>()}
                </div>

                // CTA at bottom
                {cta_mobile.as_ref().map(|c| {
                    let href = c.href.clone();
                    let label = c.label.clone();
                    view! {
                        <div class="dm-nav-mobile-cta-wrap">
                            <a href=href class="dm-nav-mobile-cta-btn" on:click=move |_| mobile_open.set(false)>{label}</a>
                        </div>
                    }
                })}
            </div>
        </Show>
    }
}

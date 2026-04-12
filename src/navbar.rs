//! Navbar — horizontal top navigation for public-facing pages.
//! Fixed position, transparent → glass blur on scroll, mobile hamburger, dropdown support.

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
        Self { label: label.to_string(), href: Some(href.to_string()), children: vec![] }
    }
    pub fn dropdown(label: &str, children: Vec<NavDropdownItem>) -> Self {
        Self { label: label.to_string(), href: None, children }
    }
    pub fn is_dropdown(&self) -> bool { !self.children.is_empty() }
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
        Self { label: label.to_string(), description: description.to_string(), href: href.to_string() }
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
        Self { label: label.to_string(), href: href.to_string() }
    }
}

/// Horizontal top navigation bar for public-facing pages.
///
/// Uses DUI CSS classes: `.dm-nav`, `.dm-nav-scrolled`, `.dm-nav-inner`, `.dm-nav-brand`, `.dm-nav-links`, `.dm-nav-cta`, `.dm-nav-mobile`.
/// No Tailwind required.
#[component]
pub fn Navbar(
    brand_name: &'static str,
    #[prop(optional)]
    brand_logo_url: Option<&'static str>,
    items: Vec<NavItem>,
    #[prop(optional, into)]
    cta: Option<NavCta>,
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let scrolled = RwSignal::new(false);
    let mobile_open = RwSignal::new(false);

    Effect::new(move |_| {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        let win = web_sys::window().unwrap();
        let cb = Closure::<dyn Fn()>::new(move || {
            let y = web_sys::window().unwrap().scroll_y().unwrap_or(0.0);
            scrolled.set(y > 10.0);
        });
        win.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    });

    let items_mobile = items.clone();
    let cta_mobile = cta.clone();

    view! {
        <nav class=move || format!(
            "dm-nav {} {}",
            if scrolled.get() { "dm-nav-scrolled" } else { "" },
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
                            view! {
                                <div class="dm-nav-dropdown" style="display:inline-block">
                                    <span class="dm-nav-links dm-cursor-pointer" style="display:flex;align-items:center;gap:4px;color:var(--dm-text-secondary)">
                                        {label}
                                        <svg style="width:12px;height:12px;opacity:0.5" viewBox="0 0 12 12" fill="currentColor">
                                            <path d="M2.5 4.5L6 8l3.5-3.5" stroke="currentColor" stroke-width="1.5" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
                                        </svg>
                                    </span>
                                    <div class="dm-dropdown-menu" style="position:absolute;top:100%;left:-16px;min-width:280px;padding-top:12px;opacity:0;pointer-events:none;transform:translateY(-8px);transition:all 0.2s ease">
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
                {items_mobile.iter().flat_map(|item| {
                    if item.is_dropdown() {
                        let children = item.children.clone();
                        let group_label = item.label.clone();
                        let mut elements = vec![
                            view! {
                                <span style="font-family:var(--dm-font-body);font-size:13px;font-weight:600;letter-spacing:0.04em;text-transform:uppercase;color:var(--dm-text-muted);margin-top:0.5rem">
                                    {group_label}
                                </span>
                            }.into_any()
                        ];
                        elements.extend(children.iter().map(|child| {
                            let href = child.href.clone();
                            let label = child.label.clone();
                            view! {
                                <a href=href class="dm-no-underline dm-text-secondary" style="font-size:20px" on:click=move |_| mobile_open.set(false)>{label}</a>
                            }.into_any()
                        }));
                        elements
                    } else {
                        let href = item.href.clone().unwrap_or_default();
                        let label = item.label.clone();
                        vec![view! {
                            <a href=href class="dm-no-underline dm-text-secondary" style="font-size:20px" on:click=move |_| mobile_open.set(false)>{label}</a>
                        }.into_any()]
                    }
                }).collect::<Vec<_>>()}
                {cta_mobile.as_ref().map(|c| {
                    let href = c.href.clone();
                    let label = c.label.clone();
                    view! { <a href=href class="dm-nav-cta" on:click=move |_| mobile_open.set(false)>{label}</a> }
                })}
                <button class="dm-btn dm-btn-ghost dm-mt-4" on:click=move |_| mobile_open.set(false)>"Close"</button>
            </div>
        </Show>
    }
}

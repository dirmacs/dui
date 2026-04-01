//! Navbar — horizontal top navigation for public-facing pages.
//! Fixed position, transparent → glass blur on scroll, mobile hamburger, dropdown support.

use leptos::prelude::*;

/// A navigation link item. Can be a simple link or a dropdown with children.
#[derive(Debug, Clone)]
pub struct NavItem {
    pub label: String,
    pub href: Option<String>,
    pub children: Vec<NavDropdownItem>,
}

impl NavItem {
    /// Simple link item.
    pub fn link(label: &str, href: &str) -> Self {
        Self { label: label.to_string(), href: Some(href.to_string()), children: vec![] }
    }
    /// Dropdown item with children.
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
/// Features:
/// - Fixed position, transparent → glass blur on scroll
/// - Mobile hamburger toggle at 768px
/// - Dropdown menus with label + description per item
/// - CTA button
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Navbar
///         brand_name="DIRMACS"
///         brand_logo_url=Some("logo.png")
///         items=vec![
///             NavItem::dropdown("Solutions", vec![
///                 NavDropdownItem::new("Eruka", "Context Intelligence", "https://eruka.dirmacs.com"),
///             ]),
///             NavItem::link("About", "/about"),
///         ]
///         cta=Some(NavCta::new("Get Started", "/start"))
///     />
/// }
/// ```
#[component]
pub fn Navbar(
    /// Brand name displayed next to logo.
    brand_name: &'static str,
    /// Optional logo image URL.
    #[prop(optional)]
    brand_logo_url: Option<&'static str>,
    /// Navigation items (links and dropdowns).
    items: Vec<NavItem>,
    /// Optional CTA button.
    #[prop(optional)]
    cta: Option<NavCta>,
    /// Optional extra CSS classes on the nav element.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let scrolled = RwSignal::new(false);
    let mobile_open = RwSignal::new(false);

    // Scroll listener for glass effect
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
            "fixed top-0 left-0 right-0 z-[100] h-16 border-b transition-all duration-300 ease-out {} {}",
            if scrolled.get() {
                "bg-[rgba(9,9,11,0.92)] backdrop-blur-xl border-[var(--dm-border)]"
            } else {
                "bg-transparent border-transparent"
            },
            class,
        )>
            <div class="max-w-[1200px] mx-auto px-6 flex items-center justify-between h-full">
                // Brand
                <a href="/" class="flex items-center gap-2.5 no-underline">
                    {brand_logo_url.map(|url| view! {
                        <img src=url alt="" class="w-7 h-7 rounded-md" />
                    })}
                    <span class="font-mono text-[15px] font-bold text-[var(--dm-text)] tracking-[0.06em] uppercase">
                        {brand_name}
                    </span>
                </a>

                // Desktop links
                <div class="hidden md:flex items-center gap-8">
                    {items.iter().map(|item| {
                        if item.is_dropdown() {
                            let children = item.children.clone();
                            let label = item.label.clone();
                            view! {
                                <div class="relative group">
                                    <span class="flex items-center gap-1 font-sans text-sm font-medium text-[var(--dm-text-secondary)] cursor-pointer transition-colors duration-150 hover:text-[var(--dm-text)]">
                                        {label}
                                        <svg class="w-3 h-3 opacity-50" viewBox="0 0 12 12" fill="currentColor">
                                            <path d="M2.5 4.5L6 8l3.5-3.5" stroke="currentColor" stroke-width="1.5" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
                                        </svg>
                                    </span>
                                    <div class="absolute top-[calc(100%+12px)] left-[-16px] bg-[var(--dm-surface)] border-[1.5px] border-[var(--dm-border)] rounded-lg p-2 min-w-[280px] opacity-0 pointer-events-none -translate-y-2 transition-all duration-200 ease-out group-hover:opacity-100 group-hover:pointer-events-auto group-hover:translate-y-0 z-50">
                                        {children.iter().map(|child| {
                                            let href = child.href.clone();
                                            let label = child.label.clone();
                                            let desc = child.description.clone();
                                            view! {
                                                <a href=href class="block px-3.5 py-2.5 rounded-md no-underline transition-all duration-150 hover:bg-[rgba(255,255,255,0.04)]">
                                                    <div class="font-mono text-[11px] font-bold tracking-[0.04em] text-[var(--dm-text)]">{label}</div>
                                                    <div class="text-[11px] text-[var(--dm-text-muted)] mt-0.5">{desc}</div>
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
                                <a href=href class="font-sans text-sm font-medium text-[var(--dm-text-secondary)] no-underline transition-colors duration-150 hover:text-[var(--dm-text)]">
                                    {label}
                                </a>
                            }.into_any()
                        }
                    }).collect::<Vec<_>>()}
                </div>

                // Desktop CTA
                <div class="hidden md:block">
                    {cta.map(|c| {
                        let href = c.href.clone();
                        let label = c.label.clone();
                        view! {
                            <a href=href class="font-mono text-[11px] font-semibold uppercase tracking-[0.05em] px-5 py-2 rounded-md border-[1.5px] border-[var(--dm-accent)] bg-[var(--dm-accent)] text-white no-underline transition-all duration-150 hover:bg-[var(--dm-accent-hover)] hover:border-[var(--dm-accent-hover)] hover:-translate-y-px">
                                {label}
                            </a>
                        }
                    })}
                </div>

                // Mobile toggle
                <button
                    class="md:hidden bg-transparent border-none text-[var(--dm-text-secondary)] cursor-pointer"
                    on:click=move |_| mobile_open.update(|v| *v = !*v)
                    aria-label="Menu"
                >
                    <svg width="24" height="24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M3 12h18M3 6h18M3 18h18"/>
                    </svg>
                </button>
            </div>
        </nav>

        // Mobile menu overlay
        <Show when=move || mobile_open.get()>
            <div class="fixed inset-0 z-[200] bg-[var(--dm-bg)] flex flex-col items-center justify-center gap-8 animate-[dm-fade-in_0.3s_ease-out]">
                {items_mobile.iter().flat_map(|item| {
                    if item.is_dropdown() {
                        item.children.iter().map(|child| {
                            let href = child.href.clone();
                            let label = child.label.clone();
                            view! {
                                <a href=href class="font-sans text-xl text-[var(--dm-text-secondary)] no-underline transition-colors hover:text-[var(--dm-text)]" on:click=move |_| mobile_open.set(false)>
                                    {label}
                                </a>
                            }.into_any()
                        }).collect::<Vec<_>>()
                    } else {
                        let href = item.href.clone().unwrap_or_default();
                        let label = item.label.clone();
                        vec![view! {
                            <a href=href class="font-sans text-xl text-[var(--dm-text-secondary)] no-underline transition-colors hover:text-[var(--dm-text)]" on:click=move |_| mobile_open.set(false)>
                                {label}
                            </a>
                        }.into_any()]
                    }
                }).collect::<Vec<_>>()}
                {cta_mobile.as_ref().map(|c| {
                    let href = c.href.clone();
                    let label = c.label.clone();
                    view! {
                        <a href=href class="font-mono text-[13px] font-semibold uppercase tracking-[0.05em] px-8 py-3 rounded-md border-[1.5px] border-[var(--dm-accent)] bg-[var(--dm-accent)] text-white no-underline" on:click=move |_| mobile_open.set(false)>
                            {label}
                        </a>
                    }
                })}
                <button
                    class="mt-4 bg-transparent border-none text-[var(--dm-text-muted)] cursor-pointer font-mono text-sm"
                    on:click=move |_| mobile_open.set(false)
                >
                    "Close"
                </button>
            </div>
        </Show>
    }
}

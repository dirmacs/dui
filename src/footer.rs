//! Footer — multi-column site footer with brand, links, social, trust line, copyright.

use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct FooterColumn {
    pub heading: String,
    pub links: Vec<FooterLink>,
}
impl FooterColumn {
    pub fn new(heading: &str, links: Vec<FooterLink>) -> Self {
        Self {
            heading: heading.to_string(),
            links,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FooterLink {
    pub label: String,
    pub href: String,
}
impl FooterLink {
    pub fn new(label: &str, href: &str) -> Self {
        Self {
            label: label.to_string(),
            href: href.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SocialPlatform {
    LinkedIn,
    Twitter,
    GitHub,
}

#[derive(Debug, Clone)]
pub struct SocialLink {
    pub platform: SocialPlatform,
    pub href: String,
}
impl SocialLink {
    pub fn new(platform: SocialPlatform, href: &str) -> Self {
        Self {
            platform,
            href: href.to_string(),
        }
    }
}

/// Site footer. Uses DUI CSS: `.dm-footer`, `.dm-footer-grid`, `.dm-footer-heading`, `.dm-footer-link`, `.dm-footer-social`, `.dm-footer-trust`, `.dm-footer-bottom`.
/// No Tailwind required.
#[component]
pub fn Footer(
    brand_name: &'static str,
    #[prop(optional)] brand_logo_url: Option<&'static str>,
    #[prop(default = "")] tagline: &'static str,
    columns: Vec<FooterColumn>,
    #[prop(optional)] social_links: Vec<SocialLink>,
    #[prop(optional)] trust_line: Option<&'static str>,
    copyright: &'static str,
    #[prop(optional)] legal_right: Option<&'static str>,
) -> impl IntoView {
    view! {
        <footer class="dm-footer">
            <div class="dm-footer-inner">
                <div class="dm-footer-grid">
                    <div class="dm-footer-brand">
                        <div class="dm-flex dm-items-center dm-gap-3">
                            {brand_logo_url.map(|url| view! { <img src=url alt="" style="width:28px;height:28px;border-radius:var(--dm-radius)" /> })}
                            <span class="dm-nav-brand-text">{brand_name}</span>
                        </div>
                        <p>{tagline}</p>
                        <div class="dm-footer-social">
                            {social_links.iter().map(|link| {
                                let href = link.href.clone();
                                let svg = match link.platform {
                                    SocialPlatform::LinkedIn => r#"<path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>"#,
                                    SocialPlatform::Twitter => r#"<path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>"#,
                                    SocialPlatform::GitHub => r#"<path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>"#,
                                };
                                let label = format!("{:?}", link.platform);
                                view! {
                                    <a href=href target="_blank" rel="noopener" aria-label=label class="dm-text-muted dm-transition-colors">
                                        <svg viewBox="0 0 24 24" fill="currentColor" class="dm-footer-social" style="width:18px;height:18px" inner_html=svg></svg>
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>

                    {columns.iter().map(|col| {
                        let heading = col.heading.clone();
                        let links = col.links.clone();
                        view! {
                            <div>
                                <div class="dm-footer-heading">{heading}</div>
                                {links.iter().map(|link| {
                                    let href = link.href.clone();
                                    let label = link.label.clone();
                                    view! { <a href=href class="dm-footer-link">{label}</a> }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                {trust_line.map(|text| view! { <div class="dm-footer-trust">{text}</div> })}

                <div class="dm-footer-bottom">
                    <span>{copyright}</span>
                    {legal_right.map(|text| view! { <span>{text}</span> })}
                </div>
            </div>
        </footer>
    }
}

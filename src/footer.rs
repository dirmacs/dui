//! Footer — multi-column site footer with brand, links, social, trust line, copyright.

use leptos::prelude::*;

/// A column of links in the footer.
#[derive(Debug, Clone)]
pub struct FooterColumn {
    pub heading: String,
    pub links: Vec<FooterLink>,
}

impl FooterColumn {
    pub fn new(heading: &str, links: Vec<FooterLink>) -> Self {
        Self { heading: heading.to_string(), links }
    }
}

/// A link in a footer column.
#[derive(Debug, Clone)]
pub struct FooterLink {
    pub label: String,
    pub href: String,
}

impl FooterLink {
    pub fn new(label: &str, href: &str) -> Self {
        Self { label: label.to_string(), href: href.to_string() }
    }
}

/// Social media platform identifiers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SocialPlatform {
    LinkedIn,
    Twitter,
    GitHub,
}

/// A social link in the footer.
#[derive(Debug, Clone)]
pub struct SocialLink {
    pub platform: SocialPlatform,
    pub href: String,
}

impl SocialLink {
    pub fn new(platform: SocialPlatform, href: &str) -> Self {
        Self { platform, href: href.to_string() }
    }
}

/// Site footer with brand, link columns, social, trust line, and copyright.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Footer
///         brand_name="DIRMACS"
///         tagline="Democratising Innovation."
///         columns=vec![
///             FooterColumn::new("Solutions", vec![FooterLink::new("Eruka", "https://eruka.dirmacs.com")]),
///         ]
///         social_links=vec![SocialLink::new(SocialPlatform::GitHub, "https://github.com/dirmacs")]
///         copyright="© 2026 Dirmacs Private Limited"
///     />
/// }
/// ```
#[component]
pub fn Footer(
    /// Brand name.
    brand_name: &'static str,
    /// Optional brand logo URL.
    #[prop(optional)]
    brand_logo_url: Option<&'static str>,
    /// Short tagline under brand.
    #[prop(default = "")]
    tagline: &'static str,
    /// Link columns.
    columns: Vec<FooterColumn>,
    /// Social links.
    #[prop(optional)]
    social_links: Vec<SocialLink>,
    /// Trust line text (e.g. "BUILT IN RUST · PATENT PENDING").
    #[prop(optional)]
    trust_line: Option<&'static str>,
    /// Copyright text.
    copyright: &'static str,
    /// Right-side text on the copyright bar (e.g. "Hyderabad, India").
    #[prop(optional)]
    legal_right: Option<&'static str>,
) -> impl IntoView {
    view! {
        <footer class="border-t-2 border-[var(--dm-border)] pt-16 pb-8 mt-8">
            <div class="max-w-[1200px] mx-auto px-6">
                // Grid: brand + columns
                <div class="grid gap-12" style=format!("grid-template-columns: 2fr {};", "1fr ".repeat(columns.len()))>
                    // Brand column
                    <div>
                        <div class="flex items-center gap-2.5">
                            {brand_logo_url.map(|url| view! {
                                <img src=url alt="" class="w-7 h-7 rounded-md" />
                            })}
                            <span class="font-mono text-[15px] font-bold text-[var(--dm-text)] tracking-[0.06em] uppercase">
                                {brand_name}
                            </span>
                        </div>
                        <p class="text-[0.7rem] text-[var(--dm-text-muted)] mt-3 max-w-[260px] leading-relaxed">
                            {tagline}
                        </p>
                        <div class="flex gap-4 mt-4">
                            {social_links.iter().map(|link| {
                                let href = link.href.clone();
                                let svg = match link.platform {
                                    SocialPlatform::LinkedIn => r#"<path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>"#,
                                    SocialPlatform::Twitter => r#"<path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>"#,
                                    SocialPlatform::GitHub => r#"<path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>"#,
                                };
                                let label = format!("{:?}", link.platform);
                                view! {
                                    <a href=href target="_blank" rel="noopener" aria-label=label class="text-[var(--dm-text-muted)] transition-colors duration-150 hover:text-[var(--dm-text)]">
                                        <svg viewBox="0 0 24 24" fill="currentColor" class="w-[18px] h-[18px]" inner_html=svg></svg>
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Link columns
                    {columns.iter().map(|col| {
                        let heading = col.heading.clone();
                        let links = col.links.clone();
                        view! {
                            <div>
                                <div class="font-mono text-[0.625rem] font-bold uppercase tracking-[0.08em] text-[var(--dm-text-muted)] mb-4">
                                    {heading}
                                </div>
                                {links.iter().map(|link| {
                                    let href = link.href.clone();
                                    let label = link.label.clone();
                                    view! {
                                        <a href=href class="block text-[0.75rem] text-[var(--dm-text-secondary)] no-underline mb-2 transition-colors duration-150 hover:text-[var(--dm-text)]">
                                            {label}
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                // Trust line
                {trust_line.map(|text| view! {
                    <div class="text-center font-mono text-[0.625rem] font-medium tracking-[0.08em] uppercase text-[var(--dm-text-dim)] py-8 border-t border-[var(--dm-border)] mt-12">
                        {text}
                    </div>
                })}

                // Copyright bar
                <div class="flex flex-wrap justify-between gap-4 font-mono text-[0.6rem] text-[var(--dm-text-dim)] mt-4">
                    <span>{copyright}</span>
                    {legal_right.map(|text| view! { <span>{text}</span> })}
                </div>
            </div>
        </footer>
    }
}

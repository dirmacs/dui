//! Hero — landing page hero section with split layout, badge, headline, CTAs.

use leptos::prelude::*;

/// Image placement in the hero.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum HeroImageSide {
    Left,
    #[default]
    Right,
}

/// Landing page hero section.
///
/// Uses DUI CSS classes: `.dm-hero`, `.dm-hero-inner`, `.dm-hero-badge`, `.dm-hero-headline`, `.dm-hero-subtitle`.
/// No Tailwind required.
#[component]
pub fn Hero(
    #[prop(optional)] badge: Option<&'static str>,
    headline: Children,
    #[prop(optional)] tagline: Option<&'static str>,
    #[prop(optional)] subtitle: Option<&'static str>,
    #[prop(optional)] image_url: Option<&'static str>,
    #[prop(default = HeroImageSide::Right)] image_side: HeroImageSide,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = "")] class: &'static str,
) -> impl IntoView {
    let has_image = image_url.is_some();
    let is_left = image_side == HeroImageSide::Left;

    view! {
        <section class=format!("dm-hero dm-hero-bg {}", class) style="border:none">
            <div class=if has_image { "dm-hero-inner dm-hero-split" } else { "dm-hero-inner dm-hero-centered" }>
                {if has_image && is_left { Some(view! {
                    <div class="dm-hero-visual"><img src=image_url.unwrap_or_default() alt="" /></div>
                }) } else { None }}

                <div class="dm-hero-content">
                    {badge.map(|t| view! { <div class="dm-hero-badge">{t}</div> })}
                    <h1 class="dm-hero-headline">{headline()}</h1>
                    {tagline.map(|t| view! { <p class="dm-text-secondary" style="font-size:clamp(0.8rem,1.5vw,1rem);margin-bottom:0.5rem">{t}</p> })}
                    {subtitle.map(|t| view! { <p class="dm-hero-subtitle">{t}</p> })}
                    {children.map(|c| c())}
                </div>

                {if has_image && !is_left { Some(view! {
                    <div class="dm-hero-visual"><img src=image_url.unwrap_or_default() alt="" /></div>
                }) } else { None }}
            </div>
        </section>
    }
}

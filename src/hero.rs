//! Hero — landing page hero section with split layout, badge, headline, CTAs.
//! Supports left-aligned text with optional right-side image/visual.

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
/// Split layout: text content on one side, optional image on the other.
/// Falls back to centered full-width when no image is provided.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Hero
///         badge="Context Intelligence"
///         headline="The reliability layer for AI agents."
///         subtitle="AI agents fabricate information. Eruka fixes it."
///         image_url=Some("logo.png")
///     >
///         <a href="/start" class="dm-btn">"Get Started"</a>
///     </Hero>
/// }
/// ```
#[component]
pub fn Hero(
    /// Badge text above headline (e.g. "Context Intelligence for AI Agents").
    #[prop(optional)]
    badge: Option<&'static str>,
    /// Main headline text. Use view! macro for <em> accent coloring.
    headline: Children,
    /// Optional tagline below headline.
    #[prop(optional)]
    tagline: Option<&'static str>,
    /// Subtitle paragraph below tagline.
    #[prop(optional)]
    subtitle: Option<&'static str>,
    /// Image URL for the visual side.
    #[prop(optional)]
    image_url: Option<&'static str>,
    /// Which side the image appears on (default: Right).
    #[prop(default = HeroImageSide::Right)]
    image_side: HeroImageSide,
    /// Slot for CTAs, trust badges, rotating text, and other custom content.
    #[prop(optional)]
    children: Option<Children>,
    /// Extra CSS classes on the section.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let has_image = image_url.is_some();
    let is_left_image = image_side == HeroImageSide::Left;

    view! {
        <section
            class=format!(
                "min-h-screen flex items-center relative py-8 {} {}",
                "bg-[radial-gradient(ellipse_at_center,rgba(99,102,241,0.15)_0%,transparent_70%)]",
                class,
            )
            style="border: none;"
        >
            <div class=format!(
                "max-w-[1200px] mx-auto px-6 w-full {}",
                if has_image {
                    if is_left_image { "grid md:grid-cols-2 gap-16 items-center" } else { "grid md:grid-cols-2 gap-16 items-center" }
                } else {
                    "flex flex-col items-center text-center"
                }
            )>
                // Image (left side if specified)
                {if has_image && is_left_image {
                    Some(view! {
                        <div class="flex items-center justify-center md:order-first order-first">
                            <img
                                src=image_url.unwrap_or_default()
                                alt=""
                                class="w-[clamp(200px,28vw,420px)] h-auto rounded-[32px] opacity-[0.92]"
                                style="filter: drop-shadow(0 0 60px rgba(99,102,241,0.25)) drop-shadow(0 0 120px rgba(99,102,241,0.1));"
                            />
                        </div>
                    })
                } else { None }}

                // Text content
                <div class=if has_image { "text-left" } else { "text-center max-w-[900px]" }>
                    // Badge
                    {badge.map(|text| view! {
                        <div class="inline-block font-mono text-[11px] font-medium tracking-[0.06em] uppercase text-[var(--dm-accent-hover)] border border-[rgba(99,102,241,0.3)] rounded-full px-4 py-1.5 mb-6">
                            {text}
                        </div>
                    })}

                    // Headline
                    <h1 class="font-[var(--dm-font-display)] italic font-normal text-[clamp(2.5rem,5.5vw,4.5rem)] leading-[1.1] mb-4 text-[var(--dm-text)]" style="font-family: var(--dm-font-display);">
                        {headline()}
                    </h1>

                    // Tagline
                    {tagline.map(|text| view! {
                        <p class="text-[clamp(0.8rem,1.5vw,1rem)] text-[var(--dm-text-secondary)] mb-2">{text}</p>
                    })}

                    // Subtitle
                    {subtitle.map(|text| view! {
                        <p class="text-base text-[var(--dm-text-secondary)] max-w-[640px] leading-[1.8] mb-10" style="font-family: var(--dm-font-body);">
                            {text}
                        </p>
                    })}

                    // Children slot (CTAs, trust badges, etc.)
                    {children.map(|c| c())}
                </div>

                // Image (right side — default)
                {if has_image && !is_left_image {
                    Some(view! {
                        <div class="hidden md:flex items-center justify-center">
                            <img
                                src=image_url.unwrap_or_default()
                                alt=""
                                class="w-[clamp(280px,28vw,420px)] h-auto rounded-[32px] opacity-[0.92]"
                                style="filter: drop-shadow(0 0 60px rgba(99,102,241,0.25)) drop-shadow(0 0 120px rgba(99,102,241,0.1));"
                            />
                        </div>
                    })
                } else { None }}
            </div>
        </section>
    }
}

//! Card — panel with optional header, body, footer, and glow border variants.

use leptos::prelude::*;

/// Card glow color variant for the border effect.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CardGlow {
    #[default]
    None,
    Accent,   // Blue
    Success,  // Green
    Warning,  // Yellow
    Danger,   // Red
    Purple,   // Purple
}

/// A card container with header/body/footer slots and optional animated glow border.
///
/// The "living system" feel: cards can pulse with a subtle border glow to indicate
/// active data streams or healthy connections.
#[component]
pub fn Card(
    /// Glow border variant — the card subtly breathes with this color.
    #[prop(default = CardGlow::None)]
    glow: CardGlow,
    /// Optional extra CSS classes on the outer wrapper.
    #[prop(default = String::new(), into)]
    class: String,
    /// Header slot (optional).
    #[prop(optional)]
    header: Option<Children>,
    /// Footer slot (optional).
    #[prop(optional)]
    footer: Option<Children>,
    /// Main body content.
    children: Children,
) -> impl IntoView {
    let glow_classes = match glow {
        CardGlow::None    => "",
        CardGlow::Accent  => "border-dm-accent/30 shadow-[0_0_16px_rgba(79,124,255,0.12)] hover:shadow-[0_0_24px_rgba(79,124,255,0.2)]",
        CardGlow::Success => "border-green-400/30 shadow-[0_0_16px_rgba(52,211,153,0.12)] hover:shadow-[0_0_24px_rgba(52,211,153,0.2)]",
        CardGlow::Warning => "border-yellow-400/30 shadow-[0_0_16px_rgba(251,191,36,0.12)] hover:shadow-[0_0_24px_rgba(251,191,36,0.2)]",
        CardGlow::Danger  => "border-red-400/30 shadow-[0_0_16px_rgba(248,113,113,0.12)] hover:shadow-[0_0_24px_rgba(248,113,113,0.2)]",
        CardGlow::Purple  => "border-purple-400/30 shadow-[0_0_16px_rgba(167,139,250,0.12)] hover:shadow-[0_0_24px_rgba(167,139,250,0.2)]",
    };

    view! {
        <div class=format!(
            "bg-dm-panel border border-dm rounded-xl overflow-hidden \
             transition-shadow duration-300 {} {}",
            glow_classes, class
        )>
            // Header
            {header.map(|h| view! {
                <div class="px-5 py-4 border-b border-dm flex items-center justify-between">
                    {h()}
                </div>
            })}

            // Body
            <div class="px-5 py-4">
                {children()}
            </div>

            // Footer
            {footer.map(|f| view! {
                <div class="px-5 py-3 border-t border-dm bg-dm-bg/50">
                    {f()}
                </div>
            })}
        </div>
    }
}

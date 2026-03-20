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
    Brutal,   // Brutalist style
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
        CardGlow::Accent  => "border-[rgba(99,102,241,0.4)] shadow-[0_0_20px_rgba(99,102,241,0.15),0_0_40px_rgba(99,102,241,0.05)]",
        CardGlow::Success => "border-[var(--dm-confirmed-border)] shadow-[var(--dm-confirmed-glow)]",
        CardGlow::Warning => "border-[var(--dm-inferred-border)] shadow-[var(--dm-inferred-glow)]",
        CardGlow::Danger  => "border-[var(--dm-unknown-border)] shadow-[var(--dm-unknown-glow)]",
        CardGlow::Purple  => "border-[rgba(168,85,247,0.4)] shadow-[0_0_16px_rgba(168,85,247,0.15)]",
        CardGlow::Brutal  => "border-2 border-[var(--dm-text)] shadow-[4px_4px_0_var(--dm-border)] hover:shadow-[6px_6px_0_var(--dm-accent)] hover:border-[var(--dm-accent)] hover:translate-x-[-2px] hover:translate-y-[-2px]",
    };

    view! {
        <div class=format!(
            "bg-[var(--dm-surface)] border-2 border-[var(--dm-border)] rounded-lg overflow-hidden \
             transition-all duration-300 {} {}",
            glow_classes, class
        )>
            // Header
            {header.map(|h| view! {
                <div class="px-5 py-4 border-b-2 border-[var(--dm-border)] flex items-center justify-between">
                    {h()}
                </div>
            })}

            // Body
            <div class="px-5 py-4">
                {children()}
            </div>

            // Footer
            {footer.map(|f| view! {
                <div class="px-5 py-3 border-t border-[var(--dm-border)] bg-[var(--dm-bg)]/50">
                    {f()}
                </div>
            })}
        </div>
    }
}

//! Card — panel with optional header, body, footer, and glow border variants.

use leptos::prelude::*;

/// Card glow color variant for the border effect.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CardGlow {
    #[default]
    None,
    Accent,
    Success,
    Warning,
    Danger,
    Purple,
    Brutal,
}

/// A card container with header/body/footer slots and optional animated glow border.
///
/// Uses DUI CSS classes: `.dm-card`, `.dm-card-brutal`. Glow variants use inline styles.
/// No Tailwind required.
#[component]
pub fn Card(
    /// Glow border variant.
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
    let (extra_class, extra_style) = match glow {
        CardGlow::None    => ("", ""),
        CardGlow::Accent  => ("", "border-color:rgba(99,102,241,0.4);box-shadow:0 0 20px rgba(99,102,241,0.15),0 0 40px rgba(99,102,241,0.05)"),
        CardGlow::Success => ("", "border-color:var(--dm-confirmed-border);box-shadow:var(--dm-confirmed-glow)"),
        CardGlow::Warning => ("", "border-color:var(--dm-inferred-border);box-shadow:var(--dm-inferred-glow)"),
        CardGlow::Danger  => ("", "border-color:var(--dm-unknown-border);box-shadow:var(--dm-unknown-glow)"),
        CardGlow::Purple  => ("", "border-color:rgba(168,85,247,0.4);box-shadow:0 0 16px rgba(168,85,247,0.15)"),
        CardGlow::Brutal  => ("dm-card-brutal", ""),
    };

    view! {
        <div
            class=format!("dm-card dm-overflow-hidden {} {} {}", extra_class, class, "")
            style=extra_style
        >
            {header.map(|h| view! {
                <div class="dm-px-5 dm-py-4 dm-border-b dm-flex dm-items-center dm-justify-between">
                    {h()}
                </div>
            })}

            <div class="dm-px-5 dm-py-4">
                {children()}
            </div>

            {footer.map(|f| view! {
                <div class="dm-px-5 dm-py-3 dm-border-t" style="background:rgba(9,9,11,0.5)">
                    {f()}
                </div>
            })}
        </div>
    }
}

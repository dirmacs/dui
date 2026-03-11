//! Avatar — image with initials fallback, size variants.

use leptos::prelude::*;

/// Avatar size.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AvatarSize {
    Xs, // 24px
    Sm, // 32px
    #[default]
    Md, // 40px
    Lg, // 48px
    Xl, // 64px
}

impl AvatarSize {
    fn container_class(&self) -> &'static str {
        match self {
            AvatarSize::Xs => "w-6 h-6 text-[10px]",
            AvatarSize::Sm => "w-8 h-8 text-xs",
            AvatarSize::Md => "w-10 h-10 text-sm",
            AvatarSize::Lg => "w-12 h-12 text-base",
            AvatarSize::Xl => "w-16 h-16 text-lg",
        }
    }
}

/// An avatar component showing an image (if available) or initials fallback.
///
/// The initials fallback extracts the first letter of the first two words of `name`.
/// Background color is derived from the name hash for consistent but varied colors.
#[component]
pub fn Avatar(
    /// Full name — used to derive initials and fallback color.
    name: String,
    /// Optional image URL.
    #[prop(optional)]
    src: Option<String>,
    /// Size variant.
    #[prop(default = AvatarSize::Md)]
    size: AvatarSize,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let initials: String = name
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    // Simple hash for deterministic color selection
    let hash: usize = name.bytes().fold(0usize, |acc, b| acc.wrapping_add(b as usize));
    let colors = [
        "bg-blue-500/20 text-blue-400",
        "bg-emerald-500/20 text-emerald-400",
        "bg-purple-500/20 text-purple-400",
        "bg-amber-500/20 text-amber-400",
        "bg-rose-500/20 text-rose-400",
        "bg-cyan-500/20 text-cyan-400",
        "bg-indigo-500/20 text-indigo-400",
        "bg-teal-500/20 text-teal-400",
    ];
    let fallback_color = colors[hash % colors.len()];

    let has_src = src.is_some() && !src.as_deref().unwrap_or("").is_empty();

    view! {
        <div class=format!(
            "rounded-full flex items-center justify-center font-semibold \
             shrink-0 overflow-hidden select-none {} {} {}",
            size.container_class(),
            if has_src { "" } else { fallback_color },
            class,
        )>
            {if has_src {
                view! {
                    <img
                        src=src.unwrap_or_default()
                        alt=name.clone()
                        class="w-full h-full object-cover"
                    />
                }.into_any()
            } else {
                view! { <span>{initials}</span> }.into_any()
            }}
        </div>
    }
}

//! Avatar — image with initials fallback, size variants.

use leptos::prelude::*;

/// Avatar size.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AvatarSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl AvatarSize {
    fn class(&self) -> &'static str {
        match self {
            AvatarSize::Xs => "dm-avatar-sm", // reuse sm for xs
            AvatarSize::Sm => "dm-avatar-sm",
            AvatarSize::Md => "dm-avatar-md",
            AvatarSize::Lg => "dm-avatar-lg",
            AvatarSize::Xl => "dm-avatar-lg", // reuse lg for xl, override with style
        }
    }
    fn style(&self) -> &'static str {
        match self {
            AvatarSize::Xs => "width:24px;height:24px;font-size:10px",
            AvatarSize::Sm => "",
            AvatarSize::Md => "",
            AvatarSize::Lg => "",
            AvatarSize::Xl => "width:64px;height:64px;font-size:18px",
        }
    }
}

/// An avatar component showing an image or initials fallback.
///
/// Uses DUI CSS classes: `.dm-avatar`, `.dm-avatar-sm/md/lg`.
/// No Tailwind required.
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

    let has_src = src.is_some() && !src.as_deref().unwrap_or("").is_empty();

    view! {
        <div
            class=format!("dm-avatar {} {}", size.class(), class)
            style=size.style()
        >
            {if has_src {
                view! {
                    <img
                        src=src.unwrap_or_default()
                        alt=name.clone()
                        style="width:100%;height:100%;object-fit:cover"
                    />
                }.into_any()
            } else {
                view! { <span>{initials}</span> }.into_any()
            }}
        </div>
    }
}

//! Button — primary / secondary / ghost / danger, sizes sm/md/lg, loading + disabled.

use leptos::prelude::*;

/// Button visual variant.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
}

/// Button size.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// A multi-variant button with loading spinner and disabled state.
///
/// Uses DUI CSS classes: `.dm-btn`, `.dm-btn-primary`, `.dm-btn-sm`, etc.
/// No Tailwind required.
///
/// # Example
/// ```rust,ignore
/// view! { <Button variant=ButtonVariant::Primary on_click=|_| {} >"Save"</Button> }
/// ```
#[component]
pub fn Button(
    /// Visual variant.
    #[prop(default = ButtonVariant::Primary)]
    variant: ButtonVariant,
    /// Size.
    #[prop(default = ButtonSize::Md)]
    size: ButtonSize,
    /// Show a spinner and disable interaction.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// Disable the button (independent of loading).
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Click handler.
    #[prop(optional)]
    on_click: Option<Box<dyn Fn(web_sys::MouseEvent)>>,
    /// Optional extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
    /// Button contents (text, icons, etc).
    children: Children,
) -> impl IntoView {
    let is_disabled = move || loading.get() || disabled.get();

    let variant_class = match variant {
        ButtonVariant::Primary => "dm-btn-primary",
        ButtonVariant::Secondary => "dm-btn-secondary",
        ButtonVariant::Ghost => "dm-btn-ghost",
        ButtonVariant::Danger => "dm-btn-danger",
    };

    let size_class = match size {
        ButtonSize::Sm => "dm-btn-sm",
        ButtonSize::Md => "",
        ButtonSize::Lg => "dm-btn-lg",
    };

    view! {
        <button
            class=move || format!(
                "dm-btn {} {} {} {}",
                variant_class,
                size_class,
                if is_disabled() { "dm-btn-disabled" } else { "" },
                class,
            )
            disabled=is_disabled
            on:click=move |ev| {
                if !is_disabled() {
                    if let Some(ref handler) = on_click {
                        handler(ev);
                    }
                }
            }
        >
            <Show when=move || loading.get()>
                <span class="dm-btn-spinner"></span>
            </Show>
            {children()}
        </button>
    }
}

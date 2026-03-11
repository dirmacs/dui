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
/// # Example
/// ```rust
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

    let variant_classes = match variant {
        ButtonVariant::Primary => {
            "bg-dm-accent hover:bg-dm-interactive text-white \
             shadow-[0_0_12px_rgba(79,124,255,0.25)] hover:shadow-[0_0_20px_rgba(79,124,255,0.4)] \
             border border-dm-accent/30"
        }
        ButtonVariant::Secondary => {
            "bg-dm-elevated hover:bg-dm-hover text-dm-text \
             border border-dm/strong hover:border-dm-muted"
        }
        ButtonVariant::Ghost => {
            "bg-transparent hover:bg-dm-hover text-dm-muted hover:text-dm-text \
             border border-transparent"
        }
        ButtonVariant::Danger => {
            "bg-dm-danger/10 hover:bg-dm-danger/20 text-dm-danger \
             border border-dm-danger/30 hover:border-dm-danger/50"
        }
    };

    let size_classes = match size {
        ButtonSize::Sm => "px-3 py-1.5 text-xs rounded-md gap-1.5",
        ButtonSize::Md => "px-4 py-2 text-sm rounded-lg gap-2",
        ButtonSize::Lg => "px-6 py-3 text-base rounded-lg gap-2.5",
    };

    view! {
        <button
            class=move || format!(
                "inline-flex items-center justify-center font-medium \
                 transition-all duration-150 ease-out cursor-pointer \
                 dm-focus-ring select-none {} {} {} {}",
                variant_classes,
                size_classes,
                if is_disabled() { "opacity-50 cursor-not-allowed pointer-events-none" } else { "" },
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
                <span class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-dm-spin shrink-0"></span>
            </Show>
            {children()}
        </button>
    }
}

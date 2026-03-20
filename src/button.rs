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
            "bg-[var(--dm-accent)] hover:bg-[var(--dm-accent-hover)] active:bg-[var(--dm-accent-active)] text-white border-2 border-[var(--dm-accent)] hover:border-[var(--dm-accent-hover)] shadow-[inset_0_1px_0_rgba(255,255,255,0.1)] hover:shadow-[var(--dm-shadow-sm)] hover:-translate-y-px active:translate-y-0 active:shadow-none"
        }
        ButtonVariant::Secondary => {
            "bg-transparent hover:bg-[var(--dm-surface)] text-[var(--dm-text)] border-2 border-[var(--dm-border)] hover:border-[var(--dm-border-hover)]"
        }
        ButtonVariant::Ghost => {
            "bg-transparent hover:bg-[var(--dm-surface)] text-[var(--dm-text-secondary)] border-2 border-transparent hover:border-[var(--dm-border)]"
        }
        ButtonVariant::Danger => {
            "bg-[var(--dm-unknown-muted)] hover:bg-[var(--dm-unknown)] text-[var(--dm-unknown-text)] hover:text-white border-2 border-[var(--dm-unknown-border)] hover:border-[var(--dm-unknown)]"
        }
    };

    let size_classes = match size {
        ButtonSize::Sm => "px-3 py-1.5 text-xs rounded-md gap-1.5",
        ButtonSize::Md => "px-[22px] py-2.5 text-[13px] rounded-md gap-2",
        ButtonSize::Lg => "px-7 py-3 text-sm rounded-md gap-2.5",
    };

    view! {
        <button
            class=move || format!(
                "inline-flex items-center justify-center font-mono font-medium uppercase tracking-[0.04em] \
                 transition-all duration-150 ease-out cursor-pointer dm-focus-ring select-none {} {} {} {}",
                variant_classes,
                size_classes,
                if is_disabled() { "opacity-40 cursor-not-allowed pointer-events-none" } else { "" },
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

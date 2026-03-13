//! Kbd — keyboard shortcut display (keycap styling like GitHub's `<kbd>`).

use leptos::prelude::*;

/// A single keyboard key rendered as a styled `<kbd>` element.
///
/// Mimics a physical keycap with a subtle bottom shadow and monospace font.
///
/// # Example
/// ```rust
/// view! { <Kbd>"⌘"</Kbd> }
/// ```
#[component]
pub fn Kbd(
    /// Extra CSS classes on the `<kbd>` element.
    #[prop(default = "")]
    class: &'static str,
    /// Key label content.
    children: Children,
) -> impl IntoView {
    view! {
        <kbd class=format!(
            "inline-flex items-center justify-center min-w-[20px] h-5 px-1.5 \
             text-[11px] font-mono font-medium leading-none rounded border \
             bg-dm-elevated text-dm-muted border-dm \
             shadow-[0_1px_0_1px_var(--dm-bg)] \
             select-none {}",
            class,
        )>
            {children()}
        </kbd>
    }
}

/// Multiple keyboard keys rendered together as a shortcut combination.
///
/// Each key is wrapped in its own `<Kbd>` element with a small gap between them.
///
/// # Example
/// ```rust
/// view! { <KbdShortcut keys=vec!["⌘", "K"] /> }
/// ```
#[component]
pub fn KbdShortcut(
    /// The individual key labels (e.g. `vec!["⌘", "K"]`).
    keys: Vec<&'static str>,
    /// Extra CSS classes on the wrapper `<span>`.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <span class=format!("inline-flex items-center gap-1 {}", class)>
            {keys.into_iter().map(|key| {
                view! { <Kbd>{key}</Kbd> }
            }).collect::<Vec<_>>()}
        </span>
    }
}

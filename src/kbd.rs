//! Kbd — keyboard shortcut display (keycap styling like GitHub's `<kbd>`).

use leptos::prelude::*;

/// A single keyboard key rendered as a styled `<kbd>` element.
///
/// Uses DUI CSS class: `.dm-kbd`. No Tailwind required.
#[component]
pub fn Kbd(
    /// Extra CSS classes on the `<kbd>` element.
    #[prop(default = "")]
    class: &'static str,
    /// Key label content.
    children: Children,
) -> impl IntoView {
    view! {
        <kbd class=format!("dm-kbd {}", class)>
            {children()}
        </kbd>
    }
}

/// Multiple keyboard keys rendered together as a shortcut combination.
///
/// # Example
/// ```rust,ignore
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
        <span class=format!("dm-inline-flex dm-items-center dm-gap-1 {}", class)>
            {keys.into_iter().map(|key| {
                view! { <Kbd>{key}</Kbd> }
            }).collect::<Vec<_>>()}
        </span>
    }
}

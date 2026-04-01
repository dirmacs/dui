//! ChatMessage — chat bubble with sender-based styling.

use leptos::prelude::*;

/// Chat message sender type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChatSender {
    System,
    User,
}

/// A single chat message bubble.
///
/// Uses DUI CSS classes + inline styles for bubble variants.
/// No Tailwind required.
#[component]
pub fn ChatMessage(
    /// Who sent this message.
    sender: ChatSender,
    /// Message text content.
    content: String,
    /// Render system message content as HTML.
    #[prop(default = false)]
    html: bool,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let is_user = sender == ChatSender::User;
    let use_html = html && !is_user;

    let bubble_style = if is_user {
        "padding:10px 16px;border-radius:var(--dm-radius-lg);border-bottom-right-radius:2px;font-size:14px;line-height:1.6;max-width:600px;border:2px solid var(--dm-accent);background:var(--dm-accent-muted);color:var(--dm-text)"
    } else {
        "padding:10px 16px;border-radius:var(--dm-radius-lg);border-bottom-left-radius:2px;font-size:14px;line-height:1.6;max-width:600px;border:2px solid var(--dm-border);border-left:4px solid var(--dm-accent);background:var(--dm-surface);color:var(--dm-text)"
    };

    let justify = if is_user { "dm-justify-end" } else { "dm-justify-start" };

    view! {
        <div class=format!("dm-flex {} {}", justify, class)>
            {if use_html {
                view! { <div style=bubble_style inner_html=content /> }.into_any()
            } else {
                view! { <div style=bubble_style>{content}</div> }.into_any()
            }}
        </div>
    }
}

use leptos::prelude::*;

/// Chat message sender type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChatSender {
    System,
    User,
}

/// A single chat message bubble with sender-based styling.
///
/// System messages appear on the left with muted background.
/// User messages appear on the right with accent background.
#[component]
pub fn ChatMessage(
    /// Who sent this message.
    sender: ChatSender,
    /// Message text content.
    content: String,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let is_user = sender == ChatSender::User;
    view! {
        <div class=format!(
            "flex {} {}",
            if is_user { "justify-end" } else { "justify-start" },
            class
        )>
            <div class=format!(
                "max-w-[80%] px-4 py-2.5 rounded-2xl text-sm leading-relaxed {}",
                if is_user {
                    "bg-[var(--dm-accent)] text-white rounded-br-sm"
                } else {
                    "bg-[var(--dm-surface)] text-[var(--dm-text)] rounded-bl-sm border border-[var(--dm-border)]"
                }
            )>
                {content}
            </div>
        </div>
    }
}

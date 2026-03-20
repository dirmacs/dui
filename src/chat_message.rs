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
                "max-w-[80%] px-4 py-2.5 rounded-lg text-sm leading-relaxed {}",
                if is_user {
                    "border-2 border-[var(--dm-accent)] bg-[var(--dm-accent-muted)] text-[var(--dm-text)] rounded-br-sm"
                } else {
                    "border-2 border-[var(--dm-border)] border-l-4 border-l-[var(--dm-accent)] bg-[var(--dm-surface)] text-[var(--dm-text)] rounded-bl-sm"
                }
            )>
                {content}
            </div>
        </div>
    }
}

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
///
/// When `html` is true, system messages render content via `inner_html`
/// (for pre-converted markdown). User messages always render as text.
///
/// Messages animate in with a directional slide (left for system, right for user)
/// via the `dm-msg-system` / `dm-msg-user` CSS classes.
#[component]
pub fn ChatMessage(
    /// Who sent this message.
    sender: ChatSender,
    /// Message text content (plain text or HTML when `html` is true).
    content: String,
    /// Render system message content as HTML (e.g. pre-converted markdown).
    #[prop(default = false)]
    html: bool,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let is_user = sender == ChatSender::User;
    let use_html = html && !is_user;
    let anim_class = if is_user { "dm-msg-user" } else { "dm-msg-system" };
    let bubble_class = format!(
        "max-w-[80%] px-4 py-2.5 rounded-lg text-sm leading-relaxed {}",
        if is_user {
            "border-2 border-[var(--dm-accent)] bg-[var(--dm-accent-muted)] text-[var(--dm-text)] rounded-br-sm"
        } else {
            "border-2 border-[var(--dm-border)] border-l-4 border-l-[var(--dm-accent)] bg-[var(--dm-surface)] text-[var(--dm-text)] rounded-bl-sm"
        }
    );
    view! {
        <div class=format!(
            "flex {} {} {}",
            if is_user { "justify-end" } else { "justify-start" },
            anim_class,
            class
        )>
            {if use_html {
                let cls = bubble_class.clone();
                view! { <div class=cls inner_html=content /> }.into_any()
            } else {
                view! { <div class=bubble_class>{content}</div> }.into_any()
            }}
        </div>
    }
}

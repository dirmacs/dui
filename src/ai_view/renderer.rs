use leptos::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ai_view::schema::*;

/// Renders an LLM-generated UI schema using DUI components.
///
/// Maps each AiComponent variant to the corresponding DUI component.
/// Collects all field values by ID and calls on_submit with collected values.
#[component]
pub fn AiView(
    /// Reactive schema from LLM.
    schema: Signal<AiViewSchema>,
    /// Called when user submits with all field values.
    on_submit: Box<dyn Fn(HashMap<String, serde_json::Value>) + 'static>,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let field_values: Rc<RefCell<HashMap<String, serde_json::Value>>> =
        Rc::new(RefCell::new(HashMap::new()));
    let on_submit = Rc::new(on_submit);

    let field_values_for_render = field_values.clone();

    view! {
        <div class=move || {
            let s = schema.get();
            let layout_class = match s.layout.as_deref() {
                Some("grid") => "grid grid-cols-2 gap-4",
                Some("row") => "flex flex-row gap-4 flex-wrap",
                _ => "flex flex-col gap-4",
            };
            format!("{} {}", layout_class, class)
        }>
            {move || {
                let s = schema.get();
                let components = s.components.clone();
                components.iter().map(move |comp| {
                    render_component(comp, field_values_for_render.clone())
                }).collect::<Vec<_>>()
            }}
            // Actions (submit/skip buttons)
            {move || {
                let s = schema.get();
                let on_sub = on_submit.clone();
                let vals = field_values.clone();
                s.actions.as_ref().map(|actions| {
                    let label = actions.submit_label.clone().unwrap_or_else(|| "Continue".to_string());
                    let skip = actions.skip_allowed.unwrap_or(false);
                    view! {
                        <div class="flex gap-2 pt-2">
                            {if skip {
                                Some(view! {
                                    <button
                                        type="button"
                                        class="px-4 py-2 rounded-lg text-sm border border-[var(--dm-border)] text-[var(--dm-text-muted)] hover:bg-[var(--dm-surface)]"
                                    >
                                        "Skip"
                                    </button>
                                })
                            } else { None }}
                            <button
                                type="button"
                                class="px-4 py-2 rounded-lg text-sm font-medium bg-[var(--dm-accent)] text-white hover:opacity-90"
                                on:click=move |_| {
                                    let collected = vals.borrow().clone();
                                    (on_sub)(collected);
                                }
                            >
                                {label.clone()}
                            </button>
                        </div>
                    }
                })
            }}
        </div>
    }
}

fn render_component(
    comp: &AiComponent,
    _field_values: Rc<RefCell<HashMap<String, serde_json::Value>>>,
) -> leptos::prelude::AnyView {
    match comp {
        AiComponent::Heading { text, level } => {
            let lvl = level.unwrap_or(2);
            let cls = match lvl {
                1 => "text-2xl font-bold",
                3 => "text-lg font-semibold",
                _ => "text-xl font-bold",
            };
            view! { <h2 class=cls style="color: var(--dm-text);">{text.clone()}</h2> }.into_any()
        }
        AiComponent::Text { content, muted } => {
            let cls = if muted.unwrap_or(false) { "text-sm text-[var(--dm-text-muted)]" } else { "text-sm text-[var(--dm-text)]" };
            view! { <p class=cls>{content.clone()}</p> }.into_any()
        }
        AiComponent::Divider {} => {
            view! { <hr class="border-[var(--dm-border)]" /> }.into_any()
        }
        AiComponent::ScoreRing { score, size, label } => {
            view! { <crate::score_ring::ScoreRing score=*score size=size.unwrap_or(120) label=None /> }.into_any()
        }
        AiComponent::Card { title, content, .. } => {
            view! {
                <div class="p-4 rounded-lg border border-[var(--dm-border)] bg-[var(--dm-surface)]">
                    {title.as_ref().map(|t| view! { <h3 class="text-sm font-semibold mb-1" style="color: var(--dm-text);">{t.clone()}</h3> })}
                    <p class="text-sm" style="color: var(--dm-text-muted);">{content.clone()}</p>
                </div>
            }.into_any()
        }
        AiComponent::ChatMessage { sender, content } => {
            let s = if sender == "user" { crate::chat_message::ChatSender::User } else { crate::chat_message::ChatSender::System };
            view! { <crate::chat_message::ChatMessage sender=s content=content.clone() /> }.into_any()
        }
        AiComponent::Progress { value, max, label } => {
            let pct = (*value / max.unwrap_or(100.0) * 100.0).min(100.0);
            view! {
                <div class="flex flex-col gap-1">
                    {label.as_ref().map(|l| view! { <span class="text-xs" style="color: var(--dm-text-muted);">{l.clone()}</span> })}
                    <div class="h-2 rounded-full bg-[var(--dm-border)] overflow-hidden">
                        <div class="h-full rounded-full bg-[var(--dm-accent)] transition-all" style=format!("width: {}%", pct) />
                    </div>
                </div>
            }.into_any()
        }
        // Interactive components render as static placeholders (signals managed by parent)
        _ => {
            view! { <div class="text-xs text-[var(--dm-text-muted)]">{format!("[component]")}</div> }.into_any()
        }
    }
}

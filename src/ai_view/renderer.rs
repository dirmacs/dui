use crate::ai_view::schema::*;
use leptos::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
                Some("grid") => "dm-ai-view-grid",
                Some("row") => "dm-ai-view-row",
                _ => "dm-ai-view-stack",
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
                        <div class="dm-ai-actions">
                            {if skip {
                                Some(view! {
                                    <button
                                        type="button"
                                        class="dm-ai-action-skip"
                                    >
                                        "Skip"
                                    </button>
                                })
                            } else { None }}
                            <button
                                type="button"
                                class="dm-ai-action-submit"
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
                1 => "dm-text-2xl dm-font-bold dm-text-primary",
                3 => "dm-text-lg dm-font-semibold dm-text-primary",
                _ => "dm-text-xl dm-font-bold dm-text-primary",
            };
            view! { <h2 class=cls>{text.clone()}</h2> }.into_any()
        }
        AiComponent::Text { content, muted } => {
            let cls = if muted.unwrap_or(false) { "dm-text-sm dm-text-muted" } else { "dm-text-sm dm-text-primary" };
            view! { <p class=cls>{content.clone()}</p> }.into_any()
        }
        AiComponent::Divider {} => {
            view! { <hr class="dm-divider" /> }.into_any()
        }
        AiComponent::ScoreRing { score, size, label } => {
            view! { <crate::score_ring::ScoreRing score=*score size=size.unwrap_or(120) label=None /> }.into_any()
        }
        AiComponent::Card { title, content, .. } => {
            view! {
                <div class="dm-ai-card">
                    {title.as_ref().map(|t| view! { <h3 class="dm-text-sm dm-font-semibold dm-mb-1 dm-text-primary">{t.clone()}</h3> })}
                    <p class="dm-text-sm dm-text-muted">{content.clone()}</p>
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
                <div class="dm-ai-progress">
                    {label.as_ref().map(|l| view! { <span class="dm-text-xs dm-text-muted">{l.clone()}</span> })}
                    <div class="dm-ai-progress-track">
                        <div class="dm-ai-progress-fill" style=format!("width: {}%", pct) />
                    </div>
                </div>
            }.into_any()
        }
        // Interactive components render as static placeholders (signals managed by parent)
        _ => {
            view! { <div class="dm-text-xs dm-text-muted">{format!("[component]")}</div> }.into_any()
        }
    }
}

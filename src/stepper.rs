//! Stepper — multi-step progress indicator.

use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct StepDef {
    pub label: String,
    pub complete: bool,
}

/// Multi-step progress indicator.
///
/// Uses DUI CSS classes: `.dm-stepper`, `.dm-step`, `.dm-step-circle`, `.dm-step-active`, `.dm-step-complete`.
/// No Tailwind required.
#[component]
pub fn Stepper(
    steps: Vec<StepDef>,
    current: RwSignal<usize>,
    #[prop(default = "")] class: &'static str,
) -> impl IntoView {
    let total = steps.len();
    view! {
        <nav aria-label="Progress" class=format!("dm-stepper {}", class)>
            {steps.into_iter().enumerate().map(|(i, step)| {
                let label = step.label.clone();
                let is_last = i == total - 1;
                view! {
                    <div class=move || {
                        let cur = current.get();
                        let state = if i < cur || step.complete { "dm-step-complete" }
                            else if i == cur { "dm-step-active" }
                            else { "" };
                        format!("dm-step {}", state)
                    }>
                        <div class="dm-step-circle" aria-current=move || if i == current.get() { "step" } else { "" }>
                            {move || if i < current.get() || step.complete { "\u{2713}".to_string() } else { format!("{}", i + 1) }}
                        </div>
                        <span class="dm-step-label">{label.clone()}</span>
                    </div>
                    {if !is_last {
                        Some(view! { <div class="dm-step-connector"></div> })
                    } else { None }}
                }
            }).collect::<Vec<_>>()}
        </nav>
    }
}

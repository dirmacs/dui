use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct StepDef {
    pub label: String,
    pub complete: bool,
}

#[component]
pub fn Stepper(
    steps: Vec<StepDef>,
    current: RwSignal<usize>,
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let total = steps.len();
    view! {
        <nav aria-label="Progress" class=format!("flex items-center w-full {}", class)>
            <ol class="flex items-center w-full">
                {steps.into_iter().enumerate().map(|(i, step)| {
                    let label = step.label.clone();
                    let is_last = i == total - 1;
                    view! {
                        <li class=move || format!("flex items-center {}", if is_last { "" } else { "flex-1" })>
                            <div class="flex flex-col items-center gap-1">
                                <div
                                    class=move || {
                                        let cur = current.get();
                                        format!("flex items-center justify-center w-8 h-8 rounded-full border-2 text-xs font-bold transition-colors {}",
                                            if i < cur || step.complete {
                                                "border-[var(--dm-success)] bg-[var(--dm-success)] text-white"
                                            } else if i == cur {
                                                "border-[var(--dm-accent)] bg-[var(--dm-accent)]/10 text-[var(--dm-accent)]"
                                            } else {
                                                "border-[var(--dm-border)] text-[var(--dm-text-muted)]"
                                            })
                                    }
                                    aria-current=move || if i == current.get() { "step" } else { "" }
                                >
                                    {move || if i < current.get() || step.complete { "✓".to_string() } else { format!("{}", i + 1) }}
                                </div>
                                <span class=move || format!("text-xs font-medium whitespace-nowrap {}",
                                    if i == current.get() { "text-[var(--dm-accent)]" } else { "text-[var(--dm-text-muted)]" })>
                                    {label.clone()}
                                </span>
                            </div>
                            {if !is_last {
                                Some(view! {
                                    <div class=move || format!("flex-1 h-0.5 mx-2 transition-colors {}",
                                        if i < current.get() { "bg-[var(--dm-success)]" } else { "bg-[var(--dm-border)]" }) />
                                })
                            } else { None }}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ol>
        </nav>
    }
}

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
                            <div class="flex flex-col items-center gap-1.5">
                                <div
                                    class=move || {
                                        let cur = current.get();
                                        let base = "flex items-center justify-center w-8 h-8 rounded-full text-xs font-bold";
                                        if i < cur || step.complete {
                                            // Completed: green filled
                                            format!("{} bg-[var(--dm-confirmed)] text-white", base)
                                        } else if i == cur {
                                            // Current: blue with ring
                                            format!("{} bg-[var(--dm-accent)]/15 text-[var(--dm-accent)] \
                                                     ring-2 ring-[var(--dm-accent)] ring-offset-2 ring-offset-[var(--dm-bg)]", base)
                                        } else {
                                            // Future: gray outline
                                            format!("{} border-2 border-[var(--dm-border-hover)] text-[var(--dm-text-muted)]", base)
                                        }
                                    }
                                    style="transition: all var(--dm-duration-base, 220ms) cubic-bezier(0.4, 0, 0.2, 1)"
                                    aria-current=move || if i == current.get() { "step" } else { "" }
                                >
                                    {move || if i < current.get() || step.complete { "✓".to_string() } else { format!("{}", i + 1) }}
                                </div>
                                <span class=move || {
                                    let cur = current.get();
                                    if i == cur {
                                        "text-xs font-medium whitespace-nowrap text-[var(--dm-accent)]"
                                    } else if i < cur || step.complete {
                                        "text-xs font-medium whitespace-nowrap text-[var(--dm-confirmed-text)]"
                                    } else {
                                        "text-xs font-medium whitespace-nowrap text-[var(--dm-text-muted)]"
                                    }
                                }>
                                    {label.clone()}
                                </span>
                            </div>
                            {if !is_last {
                                Some(view! {
                                    <div class=move || format!("flex-1 h-0.5 mx-2 rounded-full {}",
                                        if i < current.get() { "bg-[var(--dm-confirmed)]" } else { "bg-[var(--dm-border)]" })
                                        style="transition: background-color var(--dm-duration-slow, 350ms) cubic-bezier(0.4, 0, 0.2, 1)"
                                    />
                                })
                            } else { None }}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ol>
        </nav>
    }
}

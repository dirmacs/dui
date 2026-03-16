use leptos::prelude::*;

/// Definition for a single step.
#[derive(Clone, Debug)]
pub struct StepDef {
    pub label: &'static str,
    pub icon: &'static str,
    pub complete: bool,
}

/// Multi-step progress indicator.
///
/// Shows where user is in the survey flow.
/// Horizontal on desktop, connected with lines between steps.
///
/// # Example
/// ```rust
/// let current_step = RwSignal::new(0usize);
/// let steps = vec![
///     StepDef { label: "Your Business", icon: "diamond", complete: true },
///     StepDef { label: "Sales", icon: "diamond-alt", complete: false },
/// ];
/// view! { <Stepper steps=steps current=current_step /> }
/// ```
#[component]
pub fn Stepper(
    /// Step definitions.
    steps: Vec<StepDef>,
    /// Current active step index (0-based).
    current: RwSignal<usize>,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let total = steps.len();
    view! {
        <nav class=format!("flex items-center gap-0 w-full {}", class) aria-label="Progress" role="navigation">
            {steps.into_iter().enumerate().map(|(i, step)| {
                let is_current = move || current.get() == i;
                let is_complete = step.complete || (move || current.get() > i)();
                let is_last = i == total - 1;

                // Colors based on state
                let circle_style = move || {
                    if is_current() {
                        "background: var(--dm-accent, #6366f1); color: white;"
                    } else if is_complete {
                        "background: var(--dm-success, #22c55e); color: white;"
                    } else {
                        "background: var(--dm-surface, #1e1e2e); color: var(--dm-text-muted);"
                    }
                };

                let label_style = move || {
                    if is_current() {
                        "color: var(--dm-text);"
                    } else {
                        "color: var(--dm-text-muted);"
                    }
                };

                let line_style = if is_complete {
                    "background: var(--dm-success, #22c55e);"
                } else {
                    "background: var(--dm-border, #333);"
                };

                view! {
                    <div class="flex items-center" style="flex: 1;">
                        // Step circle + label
                        <div class="flex flex-col items-center gap-1 cursor-pointer" on:click=move |_| current.set(i)>
                            <div class="flex items-center justify-center w-8 h-8 rounded-full text-sm font-bold transition-colors" style=circle_style>
                                {if is_complete { "✓".to_string() } else { format!("{}", i + 1) }}
                            </div>
                            <span class="text-xs font-medium whitespace-nowrap" style=label_style>
                                {step.label}
                            </span>
                        </div>
                        // Connecting line (except for last step)
                        {if !is_last { Some(view! {
                            <div class="flex-1 h-0.5 mx-2" style=line_style />
                        }) } else { None }}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </nav>
    }
}
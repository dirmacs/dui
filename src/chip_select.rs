//! ChipSelect — multi-option toggle selector with pill buttons.

use leptos::prelude::*;

/// Multi-option toggle selector with horizontally-wrapping pill buttons.
///
/// Uses DUI CSS classes + inline styles. No Tailwind required.
#[component]
pub fn ChipSelect(
    /// Available options.
    options: Vec<&'static str>,
    /// Reactive selected values.
    selected: RwSignal<Vec<String>>,
    /// Label displayed above the chips.
    #[prop(optional)]
    label: Option<&'static str>,
    /// Maximum number of selections.
    #[prop(optional)]
    max: Option<usize>,
    /// Disabled state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let toggle = move |option: String| {
        if disabled.get() { return; }
        let mut current = selected.get();
        if let Some(pos) = current.iter().position(|s| *s == option) {
            current.remove(pos);
        } else {
            if let Some(max_sel) = max {
                if current.len() >= max_sel { return; }
            }
            current.push(option);
        }
        selected.set(current);
    };

    view! {
        <div class=format!("dm-flex dm-flex-col dm-gap-2 {}", class)>
            {label.map(|l| view! {
                <label class="dm-input-label">{l}</label>
            })}
            <div role="listbox" aria-label=label.unwrap_or("Select options") aria-multiselectable="true" class="dm-flex dm-flex-wrap dm-gap-2">
                {options.into_iter().map(|opt| {
                    let opt_str = opt.to_string();
                    let opt_for_click = opt_str.clone();
                    let opt_for_check = opt_str.clone();
                    let opt_for_aria = opt_str.clone();
                    view! {
                        <button
                            type="button"
                            role="option"
                            aria-selected=move || if selected.get().contains(&opt_for_aria) { "true" } else { "false" }
                            tabindex="0"
                            disabled=move || disabled.get()
                            class="dm-badge dm-cursor-pointer dm-select-none dm-transition"
                            style=move || {
                                let is_selected = selected.get().contains(&opt_for_check);
                                if is_selected {
                                    "border-color:var(--dm-accent);background:var(--dm-accent-muted);color:var(--dm-accent);min-height:44px;min-width:44px;padding:6px 12px;font-size:14px"
                                } else {
                                    "border-color:var(--dm-border);background:var(--dm-surface);color:var(--dm-text-secondary);min-height:44px;min-width:44px;padding:6px 12px;font-size:14px"
                                }
                            }
                            on:click=move |_| toggle(opt_for_click.clone())
                        >
                            {opt_str}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

//! RadioGroup — radio button group with vertical or horizontal layout.

use leptos::prelude::*;

/// Orientation of the radio group layout.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum RadioOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// A single option within a [`RadioGroup`].
#[derive(Debug, Clone)]
pub struct RadioOption {
    /// Unique value for this option.
    pub value: String,
    /// Display label.
    pub label: String,
    /// Optional description shown below the label.
    pub description: Option<String>,
    /// Whether this specific option is disabled.
    pub disabled: bool,
}

/// A radio button group with ARIA semantics and keyboard navigation.
///
/// Arrow keys cycle between options; Space selects the focused option.
/// Supports vertical (stacked) and horizontal layouts.
///
/// # Example
/// ```rust
/// let selected = RwSignal::new("monthly".to_string());
/// let options = vec![
///     RadioOption { value: "monthly".into(), label: "Monthly".into(), description: None, disabled: false },
///     RadioOption { value: "yearly".into(), label: "Yearly".into(), description: Some("Save 20%".into()), disabled: false },
/// ];
/// view! { <RadioGroup value=selected options=options label="Billing cycle" /> }
/// ```
#[component]
pub fn RadioGroup(
    /// Currently selected value.
    #[prop(into)]
    value: RwSignal<String>,
    /// Available radio options.
    options: Vec<RadioOption>,
    /// Group label displayed above the options.
    #[prop(optional)]
    label: Option<&'static str>,
    /// Disabled state for the entire group.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Layout orientation.
    #[prop(default = RadioOrientation::Vertical)]
    orientation: RadioOrientation,
    /// Extra CSS classes on the outer wrapper.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let orientation_class = match orientation {
        RadioOrientation::Vertical => "flex-col gap-3",
        RadioOrientation::Horizontal => "flex-row flex-wrap gap-6",
    };

    // Store option values for keyboard navigation
    let option_values: Vec<String> = options.iter().map(|o| o.value.clone()).collect();
    let option_disabled: Vec<bool> = options.iter().map(|o| o.disabled).collect();

    view! {
        <div
            class=format!("flex flex-col gap-2 {}", class)
            role="radiogroup"
            aria-label=label.unwrap_or("Radio group")
        >
            // Group label
            {label.map(|l| view! {
                <span class="text-sm font-medium text-dm-muted">{l}</span>
            })}

            // Options container
            <div class=format!("flex {}", orientation_class)>
                {options.into_iter().enumerate().map(|(idx, opt)| {
                    let opt_value = opt.value.clone();
                    let opt_value_click = opt.value.clone();
                    let opt_value_key = opt.value.clone();
                    let opt_disabled = opt.disabled;
                    let opt_label = opt.label.clone();
                    let opt_description = opt.description.clone();

                    // Clone navigational data for keyboard handler
                    let nav_values = option_values.clone();
                    let nav_disabled = option_disabled.clone();

                    let select_option = move || {
                        if !disabled.get() && !opt_disabled {
                            value.set(opt_value_click.clone());
                        }
                    };

                    view! {
                        <div
                            class=move || format!(
                                "inline-flex items-start gap-2.5 select-none {}",
                                if disabled.get() || opt_disabled { "opacity-50" } else { "" },
                            )
                            on:click=move |_| select_option()
                        >
                            // Radio circle
                            <span
                                role="radio"
                                aria-checked=move || if value.get() == opt_value { "true" } else { "false" }
                                aria-disabled=move || if disabled.get() || opt_disabled { "true" } else { "false" }
                                tabindex=move || if disabled.get() || opt_disabled { "-1" } else { "0" }
                                class=move || format!(
                                    "shrink-0 relative inline-flex items-center justify-center \
                                     w-[18px] h-[18px] rounded-full border-2 \
                                     transition-all duration-150 \
                                     dm-focus-ring \
                                     {} {}",
                                    if value.get() == opt_value_key {
                                        "border-dm-accent"
                                    } else {
                                        "border-dm hover:border-dm-strong"
                                    },
                                    if disabled.get() || opt_disabled { "cursor-not-allowed" } else { "cursor-pointer" },
                                )
                                on:keydown={
                                    let nav_values = nav_values.clone();
                                    let nav_disabled = nav_disabled.clone();
                                    move |ev: web_sys::KeyboardEvent| {
                                        use wasm_bindgen::JsCast;
                                        let key = ev.key();
                                        match key.as_str() {
                                            " " => {
                                                ev.prevent_default();
                                                if !disabled.get() && !opt_disabled {
                                                    value.set(nav_values[idx].clone());
                                                }
                                            }
                                            "ArrowDown" | "ArrowRight" => {
                                                ev.prevent_default();
                                                // Find next enabled option
                                                let len = nav_values.len();
                                                for offset in 1..len {
                                                    let next = (idx + offset) % len;
                                                    if !nav_disabled[next] && !disabled.get() {
                                                        value.set(nav_values[next].clone());
                                                        // Focus the next radio element
                                                        if let Some(target) = ev.target() {
                                                            if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                                                                if let Some(parent) = el.closest("[role='radiogroup']").ok().flatten() {
                                                                    let radios = parent.query_selector_all("[role='radio']").ok();
                                                                    if let Some(radios) = radios {
                                                                        if let Some(next_el) = radios.item(next as u32) {
                                                                            if let Some(next_html) = next_el.dyn_ref::<web_sys::HtmlElement>() {
                                                                                let _ = next_html.focus();
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        break;
                                                    }
                                                }
                                            }
                                            "ArrowUp" | "ArrowLeft" => {
                                                ev.prevent_default();
                                                let len = nav_values.len();
                                                for offset in 1..len {
                                                    let prev = (idx + len - offset) % len;
                                                    if !nav_disabled[prev] && !disabled.get() {
                                                        value.set(nav_values[prev].clone());
                                                        if let Some(target) = ev.target() {
                                                            if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                                                                if let Some(parent) = el.closest("[role='radiogroup']").ok().flatten() {
                                                                    let radios = parent.query_selector_all("[role='radio']").ok();
                                                                    if let Some(radios) = radios {
                                                                        if let Some(prev_el) = radios.item(prev as u32) {
                                                                            if let Some(prev_html) = prev_el.dyn_ref::<web_sys::HtmlElement>() {
                                                                                let _ = prev_html.focus();
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        break;
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            >
                                // Inner filled circle when selected
                                <Show when={
                                    let v = opt.value.clone();
                                    move || value.get() == v
                                }>
                                    <span class="w-[6px] h-[6px] rounded-full bg-dm-accent"></span>
                                </Show>
                            </span>

                            // Label + Description
                            {(opt_label.as_str() != "" || opt_description.is_some()).then(|| view! {
                                <div class="flex flex-col gap-0.5">
                                    <span
                                        class=move || format!(
                                            "text-sm text-dm-text leading-tight {}",
                                            if disabled.get() || opt_disabled { "cursor-not-allowed" } else { "cursor-pointer" },
                                        )
                                    >
                                        {opt_label.clone()}
                                    </span>
                                    {opt_description.clone().map(|d| view! {
                                        <span
                                            class=move || format!(
                                                "text-xs text-dm-dim leading-snug {}",
                                                if disabled.get() || opt_disabled { "cursor-not-allowed" } else { "cursor-pointer" },
                                            )
                                        >
                                            {d.clone()}
                                        </span>
                                    })}
                                </div>
                            })}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

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
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub disabled: bool,
}

/// A radio button group with ARIA semantics and keyboard navigation.
///
/// Uses DUI CSS classes: `.dm-radio`, `.dm-radio-circle`, `.dm-radio-selected`, `.dm-radio-dot`.
/// No Tailwind required.
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
    let orientation_style = match orientation {
        RadioOrientation::Vertical => "flex-direction:column;gap:12px",
        RadioOrientation::Horizontal => "flex-direction:row;flex-wrap:wrap;gap:24px",
    };

    let option_values: Vec<String> = options.iter().map(|o| o.value.clone()).collect();
    let option_disabled: Vec<bool> = options.iter().map(|o| o.disabled).collect();

    view! {
        <div
            class=format!("dm-flex dm-flex-col dm-gap-2 {}", class)
            role="radiogroup"
            aria-label=label.unwrap_or("Radio group")
        >
            {label.map(|l| view! {
                <span class="dm-text-sm dm-font-medium dm-text-muted">{l}</span>
            })}

            <div class="dm-flex" style=orientation_style>
                {options.into_iter().enumerate().map(|(idx, opt)| {
                    let opt_value = opt.value.clone();
                    let opt_value_click = opt.value.clone();
                    let opt_value_key = opt.value.clone();
                    let opt_disabled = opt.disabled;
                    let opt_label = opt.label.clone();
                    let opt_description = opt.description.clone();
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
                                "dm-radio {}",
                                if disabled.get() || opt_disabled { "dm-opacity-40 dm-cursor-not-allowed" } else { "" },
                            )
                            on:click=move |_| select_option()
                        >
                            <span
                                role="radio"
                                aria-checked=move || if value.get() == opt_value { "true" } else { "false" }
                                aria-disabled=move || if disabled.get() || opt_disabled { "true" } else { "false" }
                                tabindex=move || if disabled.get() || opt_disabled { "-1" } else { "0" }
                                class=move || format!(
                                    "dm-radio-circle dm-cursor-pointer {}",
                                    if value.get() == opt_value_key { "dm-radio-selected" } else { "" },
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
                                                let len = nav_values.len();
                                                for offset in 1..len {
                                                    let next = (idx + offset) % len;
                                                    if !nav_disabled[next] && !disabled.get() {
                                                        value.set(nav_values[next].clone());
                                                        if let Some(target) = ev.target() {
                                                            if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                                                                if let Some(parent) = el.closest("[role='radiogroup']").ok().flatten() {
                                                                    if let Some(radios) = parent.query_selector_all("[role='radio']").ok() {
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
                                                                    if let Some(radios) = parent.query_selector_all("[role='radio']").ok() {
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
                                <Show when={
                                    let v = opt.value.clone();
                                    move || value.get() == v
                                }>
                                    <span class="dm-radio-dot"></span>
                                </Show>
                            </span>

                            {(opt_label.as_str() != "" || opt_description.is_some()).then(|| view! {
                                <div>
                                    <span class="dm-text-sm dm-text-primary dm-cursor-pointer">{opt_label.clone()}</span>
                                    {opt_description.clone().map(|d| view! {
                                        <span class="dm-text-xs dm-text-muted" style="display:block;margin-top:2px">{d.clone()}</span>
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

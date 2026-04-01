//! Accordion — collapsible content sections with animated expand/collapse.

use leptos::prelude::*;

/// A single collapsible accordion section.
///
/// Uses DUI CSS classes: `.dm-accordion-item`, `.dm-accordion-trigger`, `.dm-accordion-content`.
/// No Tailwind required.
#[component]
pub fn AccordionItem(
    /// Header text.
    #[prop(into)]
    title: String,
    /// Start in the open state.
    #[prop(default = false)]
    default_open: bool,
    /// Extra CSS classes on the outer wrapper.
    #[prop(default = "")]
    class: &'static str,
    /// Collapsible body content.
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(default_open);

    let section_id = format!("dm-accordion-{}", js_sys::Math::random().to_bits());
    let section_id_clone = section_id.clone();

    view! {
        <div class=move || format!(
            "dm-accordion-item {} {}",
            if open.get() { "dm-accordion-open" } else { "" },
            class,
        )>
            <button
                class="dm-accordion-trigger"
                on:click=move |_| open.update(|v| *v = !*v)
                aria-expanded=move || if open.get() { "true" } else { "false" }
                aria-controls=section_id.clone()
            >
                <span class="dm-text-sm dm-font-medium">{title.clone()}</span>
                <svg
                    class="dm-accordion-chevron"
                    style="width:16px;height:16px"
                    xmlns="http://www.w3.org/2000/svg" fill="none"
                    viewBox="0 0 24 24" stroke-width="2" stroke="currentColor"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
                </svg>
            </button>

            <div
                id=section_id_clone.clone()
                class="dm-accordion-content"
                style=move || if open.get() { "" } else { "display:none" }
            >
                {children()}
            </div>
        </div>
    }
}

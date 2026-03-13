//! Accordion — collapsible content sections with animated expand/collapse.

use leptos::prelude::*;

/// A single collapsible accordion section.
///
/// Each `AccordionItem` manages its own open/closed state. Compose multiple
/// items together inside a wrapper `<div>` for a full accordion.
///
/// ```rust
/// view! {
///     <div class="border border-dm rounded-xl overflow-hidden divide-y divide-dm">
///         <AccordionItem title="Section One">
///             <p>"Content for section one."</p>
///         </AccordionItem>
///         <AccordionItem title="Section Two">
///             <p>"Content for section two."</p>
///         </AccordionItem>
///     </div>
/// }
/// ```
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
    let body = children();

    // Generate a stable ID for aria-controls
    let section_id = format!("dm-accordion-{}", js_sys::Math::random().to_bits());
    let section_id_clone = section_id.clone();

    view! {
        <div class=format!("w-full {}", class)>
            // Header — acts as a toggle button
            <button
                class="w-full px-4 py-3 flex items-center justify-between cursor-pointer \
                       hover:bg-dm-hover text-dm-text transition-colors text-left"
                on:click=move |_| open.update(|v| *v = !*v)
                aria-expanded=move || if open.get() { "true" } else { "false" }
                aria-controls=section_id.clone()
            >
                <span class="text-sm font-medium">{title.clone()}</span>

                // Chevron icon — rotates 180° when open
                <svg
                    class=move || format!(
                        "w-4 h-4 text-dm-dim transition-transform duration-200 shrink-0 {}",
                        if open.get() { "rotate-180" } else { "" }
                    )
                    xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                    stroke-width="2" stroke="currentColor"
                >
                    <path stroke-linecap="round" stroke-linejoin="round"
                          d="m19.5 8.25-7.5 7.5-7.5-7.5" />
                </svg>
            </button>

            // Collapsible content
            <div
                id=section_id_clone
                role="region"
                class=move || {
                    if open.get() {
                        "overflow-hidden"
                    } else {
                        "hidden"
                    }
                }
            >
                <div class="px-4 pb-4 pt-1 text-sm text-dm-muted">
                    {body}
                </div>
            </div>
        </div>
    }
}

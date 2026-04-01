//! ScrollReveal — scroll-triggered fade-in animation wrapper.
//! Uses IntersectionObserver to add `.visible` to `.dm-fade-in` elements.
//! Respects `prefers-reduced-motion`.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Wraps children in a fade-in container that animates when scrolled into view.
///
/// Uses DUI CSS: `.dm-fade-in` + `.visible`. No Tailwind required.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <ScrollReveal>
///         <Card>"This fades in on scroll"</Card>
///     </ScrollReveal>
///     <ScrollReveal delay_ms=200>
///         <Card>"This fades in 200ms later"</Card>
///     </ScrollReveal>
/// }
/// ```
#[component]
pub fn ScrollReveal(
    /// Content to reveal.
    children: Children,
    /// Extra CSS classes.
    #[prop(default = "")]
    class: &'static str,
    /// Stagger delay in ms (0, 100, 200, 300, 400).
    #[prop(default = 0)]
    delay_ms: u32,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    let stagger_class = match delay_ms {
        0 => "",
        1..=100 => "dm-stagger-1",
        101..=200 => "dm-stagger-2",
        201..=300 => "dm-stagger-3",
        _ => "dm-stagger-4",
    };

    // Set up IntersectionObserver on mount
    Effect::new(move |_| {
        let Some(el) = node_ref.get() else { return };

        // Check prefers-reduced-motion
        let prefers_reduced = web_sys::window()
            .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
            .map(|mql| mql.matches())
            .unwrap_or(false);

        if prefers_reduced {
            // Skip animation, show immediately
            let _ = el.class_list().add_1("visible");
            return;
        }

        // Create IntersectionObserver
        let el_clone = el.clone();
        let callback = Closure::<dyn Fn(js_sys::Array, web_sys::IntersectionObserver)>::new(
            move |entries: js_sys::Array, observer: web_sys::IntersectionObserver| {
                for i in 0..entries.length() {
                    if let Some(entry) = entries.get(i).dyn_ref::<web_sys::IntersectionObserverEntry>() {
                        if entry.is_intersecting() {
                            let _ = el_clone.class_list().add_1("visible");
                            observer.unobserve(&el_clone);
                        }
                    }
                }
            },
        );

        let mut options = web_sys::IntersectionObserverInit::new();
        options.threshold(&JsValue::from_f64(0.1));

        if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
            callback.as_ref().unchecked_ref(),
            &options,
        ) {
            observer.observe(&el);
        }

        // Keep closure alive
        callback.forget();
    });

    view! {
        <div node_ref=node_ref class=format!("dm-fade-in {} {}", stagger_class, class)>
            {children()}
        </div>
    }
}

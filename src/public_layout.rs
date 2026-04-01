//! PublicLayout — page shell for public-facing (unauthenticated) pages.
//! The public equivalent of the Sidebar-based authenticated layout.
//! Wraps: Navbar (fixed top) + content area + Footer.

use leptos::prelude::*;

/// Public page shell: Navbar + content + Footer.
///
/// Use this to wrap public-facing pages (landing pages, about, blog)
/// as opposed to the Sidebar layout used for authenticated dashboards.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <PublicLayout>
///         <Hero headline="Hello World" />
///         <section>"Content here"</section>
///     </PublicLayout>
/// }
/// ```
///
/// Note: Navbar and Footer should be rendered by the caller around this,
/// or this component provides the content wrapper with proper spacing.
#[component]
pub fn PublicLayout(
    /// Page content between navbar and footer.
    children: Children,
    /// Extra CSS classes on the main wrapper.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!(
            "min-h-screen bg-[var(--dm-bg)] text-[var(--dm-text)] {}",
            class,
        )>
            // Content area with top padding for fixed navbar (64px)
            <main>
                {children()}
            </main>
        </div>
    }
}

//! # DUI
//!
//! A production-ready Leptos 0.7 component library with 29 accessible,
//! signal-driven components and a dark-first design system.
//!
//! ## Features
//!
//! - **29 components**: Form controls, data display, navigation, overlays, feedback, layout
//! - **Accessible**: ARIA roles, keyboard navigation, focus management, screen reader support
//! - **Dark-first with light mode**: Complete CSS custom property theming system
//! - **Signal-driven**: Built on Leptos reactive primitives (`Signal`, `RwSignal`)
//! - **Zero JS**: Pure Rust/WASM, no JavaScript dependencies
//!
//! ## Quick Start
//!
//! ```toml
//! [dependencies]
//! DUI = "0.2"
//! ```
//!
//! ```rust,ignore
//! use dui::prelude::*;
//!
//! #[component]
//! fn MyApp() -> impl IntoView {
//!     provide_toast();
//!     view! {
//!         <Button variant=ButtonVariant::Primary on_click=Box::new(|_| {})>
//!             "Click me"
//!         </Button>
//!         <ToastContainer />
//!     }
//! }
//! ```
//!
//! ## CSS Setup
//!
//! Link `DUI.css` in your HTML. For light mode, add `data-theme="light"`
//! to your `<html>` element, or let it auto-detect via `prefers-color-scheme`.
//!
//! ## Components
//!
//! | Category | Components |
//! |----------|-----------|
//! | **Form** | Button, Input, Textarea, Select, Checkbox, Radio, Switch |
//! | **Data** | Badge, Card, Table, Avatar, StatsCard, StatusBadge, ProgressBar, Skeleton |
//! | **Nav** | Sidebar, Tabs, Breadcrumb |
//! | **Overlay** | Modal, Tooltip, Dropdown, Sheet, CommandPalette |
//! | **Feedback** | Toast, AlertBanner, EmptyState |
//! | **Layout** | Divider, Accordion, Kbd |

pub mod accordion;
pub mod alert_banner;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod command;
pub mod divider;
pub mod dropdown;
pub mod empty_state;
pub mod input;
pub mod kbd;
pub mod modal;
pub mod progress_bar;
pub mod radio;
pub mod select;
pub mod sheet;
pub mod sidebar;
pub mod skeleton;
pub mod stats_card;
pub mod status_badge;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod toast;
pub mod tooltip;

/// Re-export all components at crate root for convenience.
pub mod prelude {
    pub use crate::accordion::*;
    pub use crate::alert_banner::*;
    pub use crate::avatar::*;
    pub use crate::badge::*;
    pub use crate::breadcrumb::*;
    pub use crate::button::*;
    pub use crate::card::*;
    pub use crate::checkbox::*;
    pub use crate::command::*;
    pub use crate::divider::*;
    pub use crate::dropdown::*;
    pub use crate::empty_state::*;
    pub use crate::input::*;
    pub use crate::kbd::*;
    pub use crate::modal::*;
    pub use crate::progress_bar::*;
    pub use crate::radio::*;
    pub use crate::select::*;
    pub use crate::sheet::*;
    pub use crate::sidebar::*;
    pub use crate::skeleton::*;
    pub use crate::stats_card::*;
    pub use crate::status_badge::*;
    pub use crate::switch::*;
    pub use crate::table::*;
    pub use crate::tabs::*;
    pub use crate::textarea::*;
    pub use crate::toast::*;
    pub use crate::tooltip::*;
}

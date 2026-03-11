//! dirmacs-ui — Shared Leptos component library for all Dirmacs applications.
//!
//! Provides a unified design system: buttons, cards, modals, sidebars, tables,
//! badges, toasts, and more. Every component is signal-driven and styled to
//! the Dirmacs dark-organic visual language.
//!
//! # Usage
//!
//! Add to your Cargo.toml:
//! ```toml
//! dirmacs-ui = { path = "../dirmacs-ui" }
//! ```
//!
//! Then import components:
//! ```rust
//! use dirmacs_ui::button::Button;
//! use dirmacs_ui::card::Card;
//! ```

pub mod alert_banner;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod empty_state;
pub mod input;
pub mod modal;
pub mod progress_bar;
pub mod select;
pub mod sidebar;
pub mod skeleton;
pub mod status_badge;
pub mod table;
pub mod tabs;
pub mod toast;
pub mod tooltip;

/// Re-export all components at crate root for convenience.
pub mod prelude {
    pub use crate::alert_banner::*;
    pub use crate::avatar::*;
    pub use crate::badge::*;
    pub use crate::button::*;
    pub use crate::card::*;
    pub use crate::empty_state::*;
    pub use crate::input::*;
    pub use crate::modal::*;
    pub use crate::progress_bar::*;
    pub use crate::select::*;
    pub use crate::sidebar::*;
    pub use crate::skeleton::*;
    pub use crate::status_badge::*;
    pub use crate::table::*;
    pub use crate::tabs::*;
    pub use crate::toast::*;
    pub use crate::tooltip::*;
}

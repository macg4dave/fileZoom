//! Runner module - orchestrates the TUI application run loop.
//!
//! This module is intentionally thin; implementation lives in submodules to
//! keep code organized: `terminal` for terminal setup, `event_loop` for the
//! main loop, and `commands` for pure helpers that mutate `App` state.

pub mod terminal;
pub mod event_loop2;
pub mod handlers;
pub mod commands;

pub use event_loop2::run_app;

#![warn(clippy::all, rust_2018_idioms)]

mod app;
#[cfg(target_arch = "wasm32")]
pub mod api;
pub use app::TemplateApp;

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use std::sync::Arc;

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        persist_window: true,
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(task_notes_gui::TemplateApp::new(cc, None, None, String::default()))),
    )
}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use eframe::web_sys::{Request, RequestInit, RequestMode, Response};
use serde::{Deserialize, Serialize};
use task_notes_gui::api::Update;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();

    use egui::{Style, Visuals};
    use wasm_bindgen_futures::spawn_local;

    wasm_bindgen_futures::spawn_local(async move {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(move |cc| {
                    let style = Style {
                        visuals: Visuals::dark(),
                        ..Style::default()
                    };
                    cc.egui_ctx.set_style(style);
                    Box::new(task_notes_gui::TemplateApp::new(
                        cc,
                        "http://localhost:5000/".to_string()
                    ))
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}

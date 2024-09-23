#![allow(dead_code)]

mod run_ui;
mod state;

use with_egui::run;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let attr = winit::window::WindowAttributes::new().with_title("egui + galileo");

    let window = event_loop.create_window(attr).unwrap();

    run(window, event_loop).await;
}

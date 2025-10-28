mod app;
mod renderer;
//mod tui_renderer;
mod gui_renderer;

use app::BiosApp;
use renderer::Renderer;
//use tui_renderer::TuiRenderer;
use gui_renderer::GuiRenderer;

fn main() {
    let mut app = BiosApp::new();

    let mode = std::env::args().nth(1).unwrap_or_else(|| "tui".to_string());

    match mode.as_str() {
        "gui" => {
            let mut renderer = GuiRenderer;
            renderer.run(&mut app);
        }
        _ => {
            //let mut renderer = TuiRenderer;
            //renderer.run(&mut app);
        }
    }
}


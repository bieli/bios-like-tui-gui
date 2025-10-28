use raylib::prelude::*;
use crate::{app::BiosApp, renderer::Renderer};

pub struct GuiRenderer;

impl Renderer for GuiRenderer {
    fn run(&mut self, app: &mut BiosApp) {
        let mut show_message = false;
        let mut message_title = String::new();
        let mut message_body = String::new();

        let (mut rl, thread) = raylib::init()
            .size(1024, 768)
            .title("BIOS Setup Utility - GUI Mode [github.com/bieli/bios-like-tui-gui]")
            .build();

        rl.set_target_fps(60);

        let mut submenu_index = 0;

        while !rl.window_should_close() {

            // Submenu items
            let submenu_items = match app.current_item() {
                "System Info" => vec![
                    "> Manufacturer: Fujitsu",
                    "> BIOS Version: 4.6.5.4",
                    "> Language: English",
                ],
                "Date/Time" => vec![
                    "> Current Date: Mon 28/10/2025",
                    "> Current Time: 22:53:59",
                ],
                "Security" => vec![
                    "> Access Level: Administrator",
                    "> Password: Not Set",
                ],
                "Exit" => vec![
                    "> F10: Save & Exit",
                    "> ESC: Exit Without Saving",
                ],
                _ => vec!["> No details available."],
            };

            if show_message {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    show_message = false;
                }
            } else {
                if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
                    app.move_left();
                    submenu_index = 0;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                    app.move_right();
                    submenu_index = 0;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_UP) {
                    if submenu_index > 0 {
                        submenu_index -= 1;
                    }
                }
                if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
                    submenu_index += 1;
                }
            }

            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::LIGHTGRAY);

            let w = d.get_screen_width();
            let h = d.get_screen_height();
            let border_color = Color::BLUE;
            let thickness = 4;

            d.draw_rectangle(0, 0, w, thickness, border_color);
            d.draw_rectangle(0, h - thickness, w, thickness, border_color);
            d.draw_rectangle(0, 0, thickness, h, border_color);
            d.draw_rectangle(w - thickness, 0, thickness, h, border_color);

            d.draw_text("BIOS Setup Utility", 20, 20, 24, Color::BLUE);

        }
    }
}


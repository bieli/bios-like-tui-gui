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

                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    show_message = true;
                    message_title = submenu_items[submenu_index].trim_start_matches("> ").to_string();
                    message_body = "This is an example message.\nPress Enter to close.".to_string();
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

            let mut x = 40;
            for (i, item) in app.menu_items.iter().enumerate() {
                let color = if i == app.selected { Color::YELLOW } else { Color::DARKGRAY };
                d.draw_text(item, x, 60, 20, color);
                x += item.len() as i32 * 12 + 30;
            }

            if submenu_index >= submenu_items.len() {
                submenu_index = submenu_items.len().saturating_sub(1);
            }

            let left_x = 20;
            let left_y = 100;
            let left_w = w / 2 - 30;
            let left_h = h - 160;
            d.draw_rectangle_lines(left_x, left_y, left_w, left_h, border_color);

            for (i, line) in submenu_items.iter().enumerate() {
                let color = if i == submenu_index { Color::YELLOW } else { Color::BLACK };
                d.draw_text(line, left_x + 10, left_y + 10 + i as i32 * 24, 20, color);
            }

            let help_x = w / 2 + 10;
            let help_y = 100;
            let help_w = w / 2 - 40;
            let help_h = (h - 160) / 2 - 10;
            d.draw_rectangle_lines(help_x, help_y, help_w, help_h, border_color);
            d.draw_text("Help", help_x + 10, help_y + 10, 20, Color::BLUE);
            d.draw_text("Use ← → arrows to navigate", help_x + 10, help_y + 40, 18, Color::BLACK);
            d.draw_text("Use ↑ ↓ to highlight submenu", help_x + 10, help_y + 60, 18, Color::BLACK);
            d.draw_text("Press F1 for general help", help_x + 10, help_y + 80, 18, Color::BLACK);

            let instr_y = help_y + help_h + 20;
            d.draw_rectangle_lines(help_x, instr_y, help_w, help_h, border_color);
            d.draw_text("Instructions", help_x + 10, instr_y + 10, 20, Color::BLUE);
            d.draw_text("F9: Previous Values", help_x + 10, instr_y + 40, 18, Color::BLACK);
            d.draw_text("F10: Save & Exit", help_x + 10, instr_y + 60, 18, Color::BLACK);
            d.draw_text("ESC: Exit", help_x + 10, instr_y + 80, 18, Color::BLACK);

            if show_message {
                let box_w = 400;
                let box_h = 120;
                let box_x = w / 2 - box_w / 2;
                let box_y = h / 2 - box_h / 2;

                d.draw_rectangle(box_x, box_y, box_w, box_h, Color::WHITE);
                d.draw_rectangle_lines(box_x, box_y, box_w, box_h, Color::BLUE);
                d.draw_text(&message_title, box_x + 20, box_y + 10, 20, Color::BLUE);
                d.draw_text(&message_body, box_x + 20, box_y + 40, 18, Color::BLACK);
                d.draw_text("[ OK ]", box_x + box_w / 2 - 30, box_y + box_h - 30, 20, Color::YELLOW);
            }
        }
    }
}


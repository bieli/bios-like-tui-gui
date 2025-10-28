use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};
use crate::{app::BiosApp, renderer::Renderer};

pub struct TuiRenderer;

impl Renderer for TuiRenderer {
    fn run(&mut self, app: &mut BiosApp) {
        let mut show_message = false;
        let mut message_title = String::new();
        let mut message_body = String::new();

        let mut stdout = stdout();
        terminal::enable_raw_mode().unwrap();
        execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide).unwrap();

        let (cols, rows) = terminal::size().unwrap();
        let width = cols;
        let height = rows;

        let mut submenu_index = 0;

        loop {
            for y in 0..height {
                execute!(
                    stdout,
                    cursor::MoveTo(0, y),
                    SetBackgroundColor(Color::Blue),
                    Print(" ".repeat(width as usize)),
                    ResetColor
                )
                .unwrap();
            }

            let horizontal = "─".repeat((width - 2) as usize);
            execute!(
                stdout,
                cursor::MoveTo(0, 0),
                SetForegroundColor(Color::Blue),
                Print("┌"),
                Print(&horizontal),
                Print("┐")
            )
            .unwrap();
            for y in 1..height - 1 {
                execute!(
                    stdout,
                    cursor::MoveTo(0, y),
                    Print("│"),
                    cursor::MoveTo(width - 1, y),
                    Print("│")
                )
                .unwrap();
            }
            execute!(
                stdout,
                cursor::MoveTo(0, height - 1),
                Print("└"),
                Print(&horizontal),
                Print("┘"),
                ResetColor
            )
            .unwrap();

            execute!(
                stdout,
                cursor::MoveTo(3, 1),
                SetForegroundColor(Color::White),
                Print("BIOS Setup Utility"),
                ResetColor
            )
            .unwrap();

            let mut x = 3;
            for (i, item) in app.menu_items.iter().enumerate() {
                execute!(stdout, cursor::MoveTo(x, 3)).unwrap();
                if i == app.selected {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Yellow),
                        Print(format!("[{}]", item)),
                        ResetColor
                    )
                    .unwrap();
                } else {
                    execute!(stdout, Print(format!(" {} ", item))).unwrap();
                }
                x += item.len() as u16 + 4;
            }

            let left_x = 3;
            let left_y = 5;
            let left_w = width / 2 - 6;
            let left_h = height - 10;
            draw_box(&mut stdout, left_x, left_y, left_w, left_h, Color::Blue);

            let submenu_items = match app.current_item() {
                "System Info" => vec![
                    "> Manufacturer: Fujitsu",
                    "> BIOS Version: 4.6.5.4",
                    "> Language: English",
                ],
                "Date/Time" => vec![
                    "> Current Date: Mon 03/13/2023",
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

            if submenu_index >= submenu_items.len() {
                submenu_index = submenu_items.len().saturating_sub(1);
            }

            for (i, line) in submenu_items.iter().enumerate() {
                execute!(
                    stdout,
                    cursor::MoveTo(left_x + 2, left_y + 1 + i as u16),
                    SetForegroundColor(if i == submenu_index { Color::Yellow } else { Color::White }),
                    Print(line),
                    ResetColor
                )
                .unwrap();
            }

            let help_x = width / 2 + 2;
            let help_y = 5;
            let help_w = width / 2 - 6;
            let help_h = (height - 10) / 2 - 1;
            draw_box(&mut stdout, help_x, help_y, help_w, help_h, Color::Blue);
            execute!(
                stdout,
                cursor::MoveTo(help_x + 2, help_y + 1),
                SetForegroundColor(Color::White),
                Print("Help"),
                cursor::MoveTo(help_x + 2, help_y + 2),
                Print("Use ← → arrows to navigate"),
                cursor::MoveTo(help_x + 2, help_y + 3),
                Print("Use ↑ ↓ to highlight submenu"),
                cursor::MoveTo(help_x + 2, help_y + 4),
                Print("Press F1 for general help"),
                ResetColor
            )
            .unwrap();

            let instr_x = help_x;
            let instr_y = help_y + help_h + 1;
            let instr_w = help_w;
            let instr_h = help_h;
            draw_box(&mut stdout, instr_x, instr_y, instr_w, instr_h, Color::Blue);
            execute!(
                stdout,
                cursor::MoveTo(instr_x + 2, instr_y + 1),
                SetForegroundColor(Color::White),
                Print("Instructions"),
                cursor::MoveTo(instr_x + 2, instr_y + 2),
                Print("F9: Previous Values"),
                cursor::MoveTo(instr_x + 2, instr_y + 3),
                Print("F10: Save & Exit"),
                cursor::MoveTo(instr_x + 2, instr_y + 4),
                Print("ESC: Exit"),
                ResetColor
            )
            .unwrap();

            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Left => app.move_left(),
                    KeyCode::Right => app.move_right(),
                    KeyCode::Up => if submenu_index > 0 { submenu_index -= 1; },
                    KeyCode::Down => if submenu_index < submenu_items.len() - 1 { submenu_index += 1; },
                    KeyCode::Enter => {
                        show_message = true;
                        message_title = submenu_items[submenu_index].trim_start_matches("> ").to_string();
                        message_body = "\nThis is an example message.".to_string();
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }

          if show_message {
              let box_w = 40;
              let box_h = 7;
              let box_x = (width - box_w) / 2;
              let box_y = (height - box_h) / 2;

              for y in 0..box_h {
                  execute!(
                      stdout,
                      cursor::MoveTo(box_x, box_y + y),
                      SetBackgroundColor(Color::White),
                      Print(" ".repeat(box_w as usize)),
                      ResetColor
                  )
                  .unwrap();
              }

              draw_box(&mut stdout, box_x, box_y, box_w, box_h, Color::White);

              execute!(
                  stdout,
                  SetBackgroundColor(Color::White),
                  cursor::MoveTo(box_x + 2, box_y + 1),
                  SetForegroundColor(Color::Blue),
                  Print(&message_title),
                  cursor::MoveTo(box_x + 2, box_y + 2),
                  SetForegroundColor(Color::Black),
                  Print(&message_body),
                  SetBackgroundColor(Color::Black),
                  cursor::MoveTo(box_x + box_w / 2 - 4, box_y + box_h - 2),
                  SetForegroundColor(Color::Yellow),
                  Print("[ OK ]"),
                  ResetColor
              )
              .unwrap();

              if let Event::Key(key) = event::read().unwrap() {
                  if key.code == KeyCode::Enter {
                      show_message = false;
                  }
              }
          }

        }

        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Show
        ).unwrap();

        terminal::disable_raw_mode().unwrap();
    }
}

fn draw_box(stdout: &mut std::io::Stdout, x: u16, y: u16, w: u16, h: u16, color: Color) {
    let horizontal = "─".repeat((w - 2) as usize);
    execute!(
        stdout,
        cursor::MoveTo(x, y),
        SetForegroundColor(color),
        Print("┌"),
        Print(&horizontal),
        Print("┐")
    )
    .unwrap();
    for i in 1..h - 1 {
        execute!(
            stdout,
            cursor::MoveTo(x, y + i),
            Print("│"),
            cursor::MoveTo(x + w - 1, y + i),
            Print("│")
        )
        .unwrap();
    }
    execute!(
        stdout,
        cursor::MoveTo(x, y + h - 1),
        Print("└"),
        Print(&horizontal),
        Print("┘"),
        ResetColor
    )
    .unwrap();
}


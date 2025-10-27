pub struct BiosApp {
    pub menu_items: Vec<&'static str>,
    pub selected: usize,
}

impl BiosApp {
    pub fn new() -> Self {
        Self {
            menu_items: vec!["System Info", "Date/Time", "Security", "Exit"],
            selected: 0,
        }
    }

    pub fn move_left(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.selected < self.menu_items.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn current_item(&self) -> &str {
        self.menu_items[self.selected]
    }

    pub fn get_details(&self) -> &'static str {
        match self.current_item() {
            "System Info" => "Manufacturer: Fujitsu\nBIOS Version: 4.6.5.4\nLanguage: English",
            "Date/Time" => "Current Date: Mon 03/13/2023\nCurrent Time: 22:53:59",
            "Security" => "Access Level: Administrator\nPassword: Not Set",
            "Exit" => "Press F10 to Save & Exit\nPress ESC to Exit Without Saving",
            _ => "No details available.",
        }
    }
}

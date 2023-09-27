use owo_colors::OwoColorize;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    keybinds: BTreeMap<String, String>,
    filter: String,
}

impl State {
    fn filtered_keybinds(&self) -> Vec<(&String, &String)> {
        if self.filter.is_empty() {
            self.keybinds.iter().collect()
        } else {
            self.keybinds
                .iter()
                .filter(|(k, v)| k.contains(&self.filter) || v.contains(&self.filter))
                .collect()
        }
    }

    fn max_key_length(&self) -> usize {
        return self
            .filtered_keybinds()
            .iter()
            .map(|(key, _)| key.len())
            .max()
            .unwrap_or(0);
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::Key]);

        self.keybinds = configuration;
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::Key(Key::Esc | Key::Ctrl('c')) => {
                close_focus();
            }
            Event::Key(Key::Backspace) => {
                self.filter.pop();

                should_render = true;
            }
            Event::Key(Key::Char(c))
                if c.is_ascii_alphabetic() || c.is_ascii_digit() || c.is_whitespace() =>
            {
                self.filter.push(c);

                should_render = true;
            }
            _ => (),
        };

        should_render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        for (key, value) in &self.filtered_keybinds() {
            println!(
                "{} {:width$} {} {}",
                "-".green().to_string(),
                key.cyan().bold(),
                "->".green().to_string(),
                value.cyan().bold(),
                width = &self.max_key_length()
            );
        }
        println!();
        println!(
            "{} {}",
            ">".cyan().bold(),
            if self.filter.is_empty() {
                "Search: _".cyan().bold().to_string()
            } else {
                format!("{}{}", "Search: ", self.filter)
                    .cyan()
                    .bold()
                    .to_string()
            }
        );
    }
}

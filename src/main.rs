use owo_colors::OwoColorize;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;
mod helper;

static LOAD_ZELLIJ_BINDINGS: &str = "LOAD_ZELLIJ_BINDINGS";

#[derive(Default)]
struct State {
    load_zellij_bindings: Option<String>,
    keybinds: BTreeMap<String, String>,
    filter: String,
}

impl State {
    fn filtered_keybinds(&self) -> Vec<(&String, &String)> {
        let filter = self.filter.to_lowercase();

        self.keybinds
            .iter()
            .filter(|(k, v)| {
                k.to_lowercase().contains(&filter) || v.to_lowercase().contains(&filter)
            })
            .collect()
    }
    fn max_key_length(&self) -> usize {
        self.filtered_keybinds()
            .iter()
            .map(|(key, _)| key.len())
            .max()
            .unwrap_or(0)
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::Key, EventType::ModeUpdate]);

        self.load_zellij_bindings = configuration.get(LOAD_ZELLIJ_BINDINGS).cloned();
        self.keybinds = configuration;
        self.keybinds.remove(LOAD_ZELLIJ_BINDINGS);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::Key(key) => match key.bare_key {
                BareKey::Esc => {
                    close_focus();
                }
                BareKey::Char('c') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    close_focus();
                }
                BareKey::Backspace => {
                    self.filter.pop();
                    should_render = true;
                }
                BareKey::Char(c)
                    if c.is_ascii_alphanumeric() || c == '-' || c == ':' || c.is_whitespace() =>
                {
                    self.filter.push(c);
                    should_render = true;
                }
                _ => {}
            },
            Event::ModeUpdate(mode_info) => {
                match self.load_zellij_bindings == Some("false".to_string()) {
                    true => {}
                    false => {
                        for (mode, key_actions) in mode_info.keybinds {
                            let mode_str = helper::modes_to_string(mode).cyan().bold().to_string();
                            for (key, actions) in key_actions {
                                let key_str = key.to_string();
                                let action_str =
                                    helper::actions_to_string(actions).cyan().bold().to_string();
                                self.keybinds.insert(
                                    format!("Mode: {}, Action: {}", mode_str, action_str),
                                    key_str,
                                );
                            }
                        }
                    }
                }

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
                "-".green(),
                key,
                "->".green(),
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

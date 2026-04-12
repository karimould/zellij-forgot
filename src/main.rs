use owo_colors::OwoColorize;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;
mod helper;
mod keybindings;

use keybindings::{ModeGroup, SearchResult};

static LOAD_ZELLIJ_BINDINGS: &str = "LOAD_ZELLIJ_BINDINGS";

#[derive(Default)]
struct State {
    load_zellij_bindings: Option<String>,
    custom_keybinds: BTreeMap<String, String>, // User-configured custom binds
    filter: String,
    mode_groups: Vec<ModeGroup>,   // Grouped keybindings from Zellij
    search_results: Vec<SearchResult>,
    cursor: usize,        // Selected result index
    scroll: usize,        // Scroll offset for results
    rows: usize,
    cols: usize,
}

impl State {
    fn perform_search(&mut self) {
        self.search_results = keybindings::search(&self.mode_groups, &self.filter);
        self.cursor = 0;
        self.scroll = 0;
    }

    fn move_down(&mut self) {
        let max = self.search_results.len().saturating_sub(1);
        if self.cursor < max {
            self.cursor += 1;
            // Scroll if needed
            let visible = self.visible_result_rows();
            if self.cursor >= self.scroll + visible {
                self.scroll += 1;
            }
        }
    }

    fn move_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            if self.cursor < self.scroll {
                self.scroll = self.cursor;
            }
        }
    }

    fn jump_first(&mut self) {
        self.cursor = 0;
        self.scroll = 0;
    }

    fn jump_last(&mut self) {
        let len = self.search_results.len();
        if len > 0 {
            self.cursor = len - 1;
            let visible = self.visible_result_rows();
            self.scroll = len.saturating_sub(visible);
        }
    }

    /// How many result entries are visible (each may render multiple lines)
    fn visible_result_rows(&self) -> usize {
        // Reserve: 2 for search bar, 1 for help line
        let content_rows = self.rows.saturating_sub(3);
        // Each result takes 1 line + N lines for related bindings
        // Conservative estimate: treat each as visible
        content_rows / 2
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
        self.custom_keybinds = configuration;
        self.custom_keybinds.remove(LOAD_ZELLIJ_BINDINGS);
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
                BareKey::Char('u') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    self.filter.clear();
                    self.perform_search();
                    should_render = true;
                }
                BareKey::Backspace => {
                    self.filter.pop();
                    self.perform_search();
                    should_render = true;
                }
                BareKey::Char('j') | BareKey::Down => {
                    self.move_down();
                    should_render = true;
                }
                BareKey::Char('k') | BareKey::Up => {
                    self.move_up();
                    should_render = true;
                }
                BareKey::Char('g') | BareKey::Home => {
                    self.jump_first();
                    should_render = true;
                }
                BareKey::Char('G') | BareKey::End => {
                    self.jump_last();
                    should_render = true;
                }
                BareKey::Char(c)
                    if c.is_ascii_alphanumeric() || c == '-' || c == ':' || c.is_whitespace() =>
                {
                    self.filter.push(c);
                    self.perform_search();
                    should_render = true;
                }
                _ => {}
            },
            Event::ModeUpdate(mode_info) => {
                if self.load_zellij_bindings != Some("false".to_string()) {
                    self.mode_groups = keybindings::parse_keybindings(mode_info.keybinds);
                    self.perform_search();
                }
                should_render = true;
            }
            _ => (),
        };

        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;

        // Content area: rows minus search bar (1) + separator (1) + help (1)
        let content_rows = rows.saturating_sub(3);

        if self.mode_groups.is_empty() && self.custom_keybinds.is_empty() {
            println!("{}", "Waiting for keybindings...".dimmed());
            // Fill remaining rows
            for _ in 1..content_rows {
                println!();
            }
            render_search_and_help(&self.filter, cols);
            return;
        }

        // Custom keybinds (user-defined) show at top if filter matches
        let show_custom = !self.custom_keybinds.is_empty() && {
            let f = self.filter.to_lowercase();
            f.is_empty()
                || self.custom_keybinds.iter().any(|(k, v)| {
                    k.to_lowercase().contains(&f) || v.to_lowercase().contains(&f)
                })
        };

        let mut printed_rows = 0;

        if show_custom {
            let filter_lower = self.filter.to_lowercase();
            let matched: Vec<_> = self
                .custom_keybinds
                .iter()
                .filter(|(k, v)| {
                    filter_lower.is_empty()
                        || k.to_lowercase().contains(&filter_lower)
                        || v.to_lowercase().contains(&filter_lower)
                })
                .collect();

            if !matched.is_empty() && printed_rows < content_rows {
                println!("{}", "Custom:".yellow().bold());
                printed_rows += 1;
                for (k, v) in &matched {
                    if printed_rows >= content_rows {
                        break;
                    }
                    println!(
                        "  {:20} {}",
                        v.cyan().bold(),
                        k.dimmed()
                    );
                    printed_rows += 1;
                }
            }
        }

        // Render search results from Zellij keybindings
        if !self.search_results.is_empty() {
            let total = self.search_results.len();
            let visible_from = self.scroll;
            let mut result_idx = visible_from;

            while result_idx < total && printed_rows < content_rows {
                let sr = &self.search_results[result_idx];
                let is_selected = result_idx == self.cursor;

                let selected_marker = if is_selected {
                    ">".cyan().bold().to_string()
                } else {
                    " ".to_string()
                };

                // Format mode context: [Mode1, Mode2, ...] or [Mode] if only one
                let mode_ctx = if sr.modes.len() > 3 {
                    format!("[shared]").dimmed().to_string()
                } else if sr.modes.len() > 1 {
                    format!("[{}]", sr.modes.join(", ")).dimmed().to_string()
                } else if !sr.modes.is_empty() {
                    format!("[{}]", sr.modes[0]).dimmed().to_string()
                } else {
                    String::new()
                };

                // If switches to a mode, show mode name as action
                let action_display = if let Some(target_mode) = sr.switches_to_mode {
                    format!("→ {} mode", helper::modes_to_string(target_mode))
                        .green()
                        .bold()
                        .to_string()
                } else {
                    format!("→ {}", sr.action_desc).green().to_string()
                };

                println!(
                    "{} {} {} {}",
                    selected_marker,
                    sr.key.cyan().bold(),
                    action_display,
                    mode_ctx
                );
                printed_rows += 1;

                // If this binding switches to a mode AND is selected,
                // show bindings available in that mode
                if is_selected && !sr.related_bindings.is_empty() {
                    let max_show = (content_rows.saturating_sub(printed_rows))
                        .min(sr.related_bindings.len());
                    if max_show > 0 {
                        let max_key_len = sr.related_bindings[..max_show]
                            .iter()
                            .map(|b| b.key.len())
                            .max()
                            .unwrap_or(0);

                        for rb in &sr.related_bindings[..max_show] {
                            if printed_rows >= content_rows {
                                break;
                            }
                            println!(
                                "    {:<width$}  {}",
                                rb.key.cyan(),
                                rb.action_desc().dimmed(),
                                width = max_key_len
                            );
                            printed_rows += 1;
                        }
                    }
                }

                result_idx += 1;
            }

            // Fill remaining space
            while printed_rows < content_rows {
                println!();
                printed_rows += 1;
            }
        } else if !self.filter.is_empty() {
            // No results
            println!("{}", "No matches found.".yellow());
            while printed_rows < content_rows {
                println!();
                printed_rows += 1;
            }
        }

        render_search_and_help(&self.filter, cols);
    }
}

fn render_search_and_help(filter: &str, cols: usize) {
    // Two-line layout: search + help
    // Line 1: Search on left, help line 1 on right
    // Line 2: empty on left, help line 2 on right

    let search_prompt = if filter.is_empty() {
        format!("{} {}", ">".cyan().bold(), "Search: _".cyan().bold())
    } else {
        format!(
            "{} {}",
            ">".cyan().bold(),
            format!("Search: {}_", filter).cyan().bold()
        )
    };

    let help_l1 = "↑/↓ j/k  g/G first/last".dimmed().to_string();
    let help_l2 = "Ctrl+u clear  Esc close".dimmed().to_string();

    // Calculate padding for right-align help
    let search_len = search_prompt.len() - count_ansi_escape_overhead(&search_prompt);
    let help1_len = help_l1.len() - count_ansi_escape_overhead(&help_l1);
    let help2_len = help_l2.len() - count_ansi_escape_overhead(&help_l2);

    let padding1 = if search_len + help1_len + 2 < cols {
        cols - search_len - help1_len
    } else {
        2
    };

    let padding2 = if help2_len + 2 < cols {
        cols - help2_len - 2
    } else {
        2
    };

    println!("{}{:width$}{}", search_prompt, "", help_l1, width = padding1);
    println!("{:width$}{}", "", help_l2, width = padding2);
}

/// Count ANSI escape sequence characters to exclude from visible length
fn count_ansi_escape_overhead(s: &str) -> usize {
    let mut overhead = 0;
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
            overhead += 1;
        } else if in_escape {
            overhead += 1;
            if c.is_alphabetic() {
                in_escape = false;
            }
        }
    }

    overhead
}

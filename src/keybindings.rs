//! Keybinding parsing and grouping by mode

use std::collections::BTreeMap;
use zellij_tile::prelude::{actions::Action, InputMode};

#[derive(Debug, Clone)]
pub struct ModeGroup {
    pub mode: InputMode,
    pub trigger_key: Option<String>, // Key that switches TO this mode from Normal
    pub bindings: Vec<Binding>,
}

#[derive(Debug, Clone)]
pub struct Binding {
    pub key: String,
    pub actions: Vec<Action>,
    pub switches_to_mode: Option<InputMode>, // If action is SwitchToMode(X)
}

impl Binding {
    pub fn action_desc(&self) -> String {
        crate::helper::actions_to_string(self.actions.clone())
    }
}

/// Parse mode_info from Zellij and group by modes
pub fn parse_keybindings<K>(mode_info_keybinds: Vec<(InputMode, Vec<(K, Vec<Action>)>)>) -> Vec<ModeGroup>
where
    K: std::fmt::Display,
{
    let mut mode_groups: BTreeMap<InputMode, ModeGroup> = BTreeMap::new();

    // First pass: collect all bindings for each mode
    for (mode, key_actions) in mode_info_keybinds {
        let mut bindings = Vec::new();

        for (key, actions) in key_actions {
            let key_str = key.to_string();

            // Check if this switches to another mode
            let switches_to: Option<InputMode> = actions.iter().find_map(|a| match a {
                Action::SwitchToMode(target_mode) => Some(target_mode.clone()),
                _ => None,
            });

            bindings.push(Binding {
                key: key_str,
                actions: actions.clone(),
                switches_to_mode: switches_to,
            });
        }

        mode_groups.insert(mode, ModeGroup {
            mode,
            trigger_key: None, // Will fill in second pass
            bindings,
        });
    }

    // Second pass: find trigger keys (keys in Normal mode that switch to other modes)
    let mut triggers: BTreeMap<InputMode, String> = BTreeMap::new();
    if let Some(normal_group) = mode_groups.get(&InputMode::Normal) {
        for binding in &normal_group.bindings {
            if let Some(target_mode) = binding.switches_to_mode {
                triggers.insert(target_mode, binding.key.clone());
            }
        }
    }

    // Apply triggers
    for (mode, trigger_key) in triggers {
        if let Some(group) = mode_groups.get_mut(&mode) {
            group.trigger_key = Some(trigger_key);
        }
    }

    mode_groups.into_values().collect()
}

/// Search result - may represent one binding found in multiple modes
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// All modes where this binding exists (deduplicated)
    pub modes: Vec<String>,
    /// The matched binding (key + action)
    pub key: String,
    pub action_desc: String,
    pub switches_to_mode: Option<InputMode>,
    /// Bindings available after switching to target mode
    pub related_bindings: Vec<Binding>,
    pub score: u32,
}

/// Smart search that understands mode sequences, deduplicates shared bindings
pub fn search(mode_groups: &[ModeGroup], query: &str) -> Vec<SearchResult> {
    if query.is_empty() {
        // Show all modes as grouped entries
        return mode_groups.iter().map(|mg| {
            let trigger_label = mg.trigger_key.as_deref()
                .map(|k| format!("{} →", k))
                .unwrap_or_default();
            SearchResult {
                modes: vec![crate::helper::modes_to_string(mg.mode)],
                key: trigger_label,
                action_desc: format!("{} mode", crate::helper::modes_to_string(mg.mode)),
                switches_to_mode: Some(mg.mode),
                related_bindings: mg.bindings.clone(),
                score: 1,
            }
        }).collect();
    }

    let query_lower = query.to_lowercase();

    // key: (key_string, action_string) → accumulated result
    let mut dedup: std::collections::BTreeMap<(String, String), SearchResult> =
        std::collections::BTreeMap::new();

    for mg in mode_groups {
        let mode_name = crate::helper::modes_to_string(mg.mode);

        for binding in &mg.bindings {
            let key_lower = binding.key.to_lowercase();
            let action_str = binding.action_desc();
            let action_lower = action_str.to_lowercase();

            let score: u32;

            if key_lower == query_lower {
                score = 100;
            } else if key_lower.starts_with(&query_lower) {
                score = 80;
            } else if key_lower.contains(&query_lower) {
                score = 60;
            } else if action_lower.contains(&query_lower) {
                score = 40;
            } else if mode_name.to_lowercase().contains(&query_lower) {
                score = 20;
            } else {
                continue; // No match
            }

            let dedup_key = (binding.key.clone(), action_str.clone());

            if let Some(existing) = dedup.get_mut(&dedup_key) {
                // Same binding found in another mode - just add the mode name
                if !existing.modes.contains(&mode_name) {
                    existing.modes.push(mode_name.clone());
                }
                if score > existing.score {
                    existing.score = score;
                }
            } else {
                // First time seeing this binding
                let related = if let Some(target_mode) = binding.switches_to_mode {
                    mode_groups
                        .iter()
                        .find(|g| g.mode == target_mode)
                        .map(|g| g.bindings.clone())
                        .unwrap_or_default()
                } else {
                    Vec::new()
                };

                dedup.insert(dedup_key, SearchResult {
                    modes: vec![mode_name.clone()],
                    key: binding.key.clone(),
                    action_desc: action_str,
                    switches_to_mode: binding.switches_to_mode,
                    related_bindings: related,
                    score,
                });
            }
        }
    }

    let mut results: Vec<SearchResult> = dedup.into_values().collect();
    results.sort_by(|a, b| b.score.cmp(&a.score));
    results
}

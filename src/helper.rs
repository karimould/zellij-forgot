use zellij_tile::prelude::{actions::Action, InputMode};

pub fn modes_to_string(mode: InputMode) -> String {
    match mode {
        InputMode::Normal => "Normal".to_string(),
        InputMode::Locked => "Locked".to_string(),
        InputMode::Resize => "Resize".to_string(),
        InputMode::Pane => "Pane".to_string(),
        InputMode::Tab => "Tab".to_string(),
        InputMode::Scroll => "Scroll".to_string(),
        InputMode::EnterSearch => "Enter Search".to_string(),
        InputMode::Search => "Search".to_string(),
        InputMode::RenameTab => "RenameTab".to_string(),
        InputMode::RenamePane => "RenamePane".to_string(),
        InputMode::Session => "Session".to_string(),
        InputMode::Move => "Move".to_string(),
        InputMode::Prompt => "Prompt".to_string(),
        InputMode::Tmux => "Tmux".to_string(),
    }
}

//  Zellij Version 0.41.2
pub fn actions_to_string(actions: Vec<Action>) -> String {
    actions
        .iter()
        .map(|action| match action {
            Action::Quit => "Quit".to_string(),
            Action::Write(key, data, bool) => format!(
                "Write({:?}, {}, {})",
                key,
                String::from_utf8_lossy(data),
                bool
            ),
            Action::WriteChars(chars) => format!("WriteChars({})", chars),
            Action::SwitchToMode(mode) => format!("SwitchToMode({:?})", mode),
            Action::SwitchModeForAllClients(mode) => format!("SwitchModeForAllClients({:?})", mode),
            Action::Resize(resize, direction) => format!("Resize({:?}, {:?})", resize, direction),
            Action::FocusNextPane => "FocusNextPane".to_string(),
            Action::FocusPreviousPane => "FocusPreviousPane".to_string(),
            Action::MoveFocus(direction) => format!("MoveFocus({:?})", direction),
            Action::MoveFocusOrTab(direction) => format!("MoveFocusOrTab({:?})", direction),
            Action::MovePane(direction) => format!("MovePane({:?})", direction),
            Action::MovePaneBackwards => "MovePaneBackward".to_string(),
            Action::ClearScreen => "ClearScreen".to_string(),
            Action::DumpScreen(_, _) => "DumpScreen".to_string(),
            Action::DumpLayout => "DumpLayout".to_string(),
            Action::EditScrollback => "EditScrollback".to_string(),
            Action::ScrollUp => "ScrollUp".to_string(),
            Action::ScrollUpAt(position) => format!("ScrollUpAt({:?})", position),
            Action::ScrollDown => "ScrollDown".to_string(),
            Action::ScrollDownAt(position) => format!("ScrollDownAt({:?})", position),
            Action::ScrollToBottom => "ScrollToBottom".to_string(),
            Action::ScrollToTop => "ScrollToTop".to_string(),
            Action::PageScrollUp => "PageScrollUp".to_string(),
            Action::PageScrollDown => "PageScrollDown".to_string(),
            Action::HalfPageScrollUp => "HalfPageScrollUp".to_string(),
            Action::HalfPageScrollDown => "HalfPageScrollDown".to_string(),
            Action::ToggleFocusFullscreen => "ToggleFocusFullscreen".to_string(),
            Action::TogglePaneFrames => "TogglePaneFrames".to_string(),
            Action::ToggleActiveSyncTab => "ToggleActiveSyncTab".to_string(),
            Action::NewPane(direction, command, bool) => {
                format!("NewPane({:?}, {:?}, {})", direction, command, bool)
            }
            Action::EditFile(path, direction, bool1, bool2, bool3, coordinates) => format!(
                "EditFile({:?}, {:?}, {}, {}, {}, {:?})",
                path, direction, bool1, bool2, bool3, coordinates
            ),
            Action::NewFloatingPane(run_command_action, command, option) => {
                format!(
                    "NewFloatingPane({:?}, {:?}, {:?})",
                    run_command_action, command, option
                )
            }
            Action::NewTiledPane(direction, run_command_action, command) => format!(
                "NewTiledPane({:?}, {:?}, {:?})",
                direction, run_command_action, command
            ),
            Action::NewInPlacePane(run_command_action, command) => {
                format!("NewInPlacePane({:?}, {:?})", run_command_action, command)
            }
            Action::TogglePaneEmbedOrFloating => "TogglePaneEmbedOrFloating".to_string(),
            Action::ToggleFloatingPanes => "ToggleFloatingPanes".to_string(),
            Action::CloseFocus => "CloseFocus".to_string(),
            Action::PaneNameInput(data) => {
                format!("PaneNameInput({})", String::from_utf8_lossy(data))
            }
            Action::UndoRenamePane => "UndoRenamePane".to_string(),
            Action::NewTab(..) => "NewTab".to_string(),
            Action::NoOp => "NoOp".to_string(),
            Action::GoToNextTab => "GoToNextTab".to_string(),
            Action::GoToPreviousTab => "GoToPreviousTab".to_string(),
            Action::CloseTab => "CloseTab".to_string(),
            Action::GoToTab(index) => format!("GoToTab({})", index),
            Action::GoToTabName(name, exact) => format!("GoToTabName({}, {})", name, exact),
            Action::ToggleTab => "ToggleTab".to_string(),
            Action::TabNameInput(data) => {
                format!("TabNameInput({})", String::from_utf8_lossy(data))
            }
            Action::UndoRenameTab => "UndoRenameTab".to_string(),
            Action::Run(run_command_action) => format!("Run({:?})", run_command_action),
            Action::Detach => "Detach".to_string(),
            Action::LeftClick(position) => format!("LeftClick({:?})", position),
            Action::RightClick(position) => format!("RightClick({:?})", position),
            Action::MiddleClick(position) => format!("MiddleClick({:?})", position),
            Action::LaunchOrFocusPlugin(_, _, _, _, _) => "LaunchOrFocusPlugin".to_string(),
            Action::LaunchPlugin(_, _, _, _, _) => "LaunchPlugin".to_string(),
            Action::LeftMouseRelease(position) => format!("LeftMouseRelease({:?})", position),
            Action::RightMouseRelease(position) => format!("RightMouseRelease({:?})", position),
            Action::MiddleMouseRelease(position) => format!("MiddleMouseRelease({:?})", position),
            Action::MouseHoldLeft(position) => format!("MouseHoldLeft({:?})", position),
            Action::MouseHoldRight(position) => format!("MouseHoldRight({:?})", position),
            Action::MouseHoldMiddle(position) => format!("MouseHoldMiddle({:?})", position),
            Action::Copy => "Copy".to_string(),
            Action::Confirm => "Confirm".to_string(),
            Action::Deny => "Deny".to_string(),
            Action::SkipConfirm(_) => "SkipConfirm".to_string(),
            Action::SearchInput(data) => format!("SearchInput({})", String::from_utf8_lossy(data)),
            Action::Search(direction) => format!("Search({:?})", direction),
            Action::SearchToggleOption(option) => format!("SearchToggleOption({:?})", option),
            Action::ToggleMouseMode => "ToggleMouseMode".to_string(),
            Action::PreviousSwapLayout => "PreviousSwapLayout".to_string(),
            Action::NextSwapLayout => "NextSwapLayout".to_string(),
            Action::QueryTabNames => "QueryTabNames".to_string(),
            Action::NewTiledPluginPane(_, _, _, _) => "NewTiledPluginPane".to_string(),
            Action::NewFloatingPluginPane(_, _, _, _, _) => "NewFloatingPluginPane".to_string(),
            Action::NewInPlacePluginPane(_, _, _) => "NewInPlacePluginPane".to_string(),
            Action::StartOrReloadPlugin(_) => "StartOrReloadPlugin".to_string(),
            Action::CloseTerminalPane(id) => format!("CloseTerminalPane({})", id),
            Action::ClosePluginPane(id) => format!("ClosePluginPane({})", id),
            Action::FocusTerminalPaneWithId(id, force) => {
                format!("FocusTerminalPaneWithId({}, {})", id, force)
            }
            Action::FocusPluginPaneWithId(id, force) => {
                format!("FocusPluginPaneWithId({}, {})", id, force)
            }
            Action::RenameTerminalPane(id, data) => format!(
                "RenameTerminalPane({}, {})",
                id,
                String::from_utf8_lossy(data)
            ),
            Action::RenamePluginPane(id, data) => format!(
                "RenamePluginPane({}, {})",
                id,
                String::from_utf8_lossy(data)
            ),
            Action::RenameTab(id, data) => {
                format!("RenameTab({}, {})", id, String::from_utf8_lossy(data))
            }
            Action::BreakPane => "BreakPane".to_string(),
            Action::BreakPaneRight => "BreakPaneRight".to_string(),
            Action::BreakPaneLeft => "BreakPaneLeft".to_string(),
            Action::RenameSession(name) => format!("RenameSession({})", name),
            _ => "Unknown Action".to_string(),
        })
        .collect::<Vec<String>>()
        .join(", ")
}

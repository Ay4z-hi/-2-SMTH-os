use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WinTab {
    pub id: u64,
    pub app_name: String,
    pub is_active: bool,
    pub window_id: u32, // Wayland surface ID
}

#[derive(Debug)]
pub struct TabGroup {
    pub id: u64,
    pub tabs: Vec<WinTab>,
    pub position: (f64, f64),
    pub size: (f64, f64),
}

pub struct WinTabManager {
    groups: HashMap<u64, TabGroup>,
    active_group_id: Option<u64>,
}

impl WinTabManager {
    pub fn new() -> Self {
        WinTabManager {
            groups: HashMap::new(),
            active_group_id: None,
        }
    }

    pub fn merge_windows(&mut self, window_a: u32, window_b: u32, app_name: &str) {
        // Logic to find or create a group
        // Add window_b as a new tab to the group containing window_a
        // Update UI to show red dot indicator on active tab
        log::info!("Merging windows {} and {} into group", window_a, window_b);
    }

    pub fn extract_tab(&mut self, group_id: u64, tab_index: usize) -> Option<WinTab> {
        // Logic to remove a tab from a group
        // If it was the only tab, close the group
        // Return the tab data to create a new floating window
        log::info!("Extracting tab {} from group {}", tab_index, group_id);
        None
    }

    pub fn set_active_tab(&mut self, group_id: u64, tab_index: usize) {
        if let Some(group) = self.groups.get_mut(&group_id) {
            for (i, tab) in group.tabs.iter_mut().enumerate() {
                tab.is_active = (i == tab_index);
            }
            self.active_group_id = Some(group_id);
        }
    }
}

fn main() {
    env_logger::init();
    log::info!("SMTH WinTab Manager initialized.");
    let mut manager = WinTabManager::new();
    // In production, this would connect to the Wayland compositor
    // and listen for window events.
}

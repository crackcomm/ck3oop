use crate::mods::{ModList, ModLoadOrder};

/// Workspace is a main unit of the application.
/// Before you can work with mods, you need to create a workspace.
#[allow(dead_code)]
pub struct Workspace {
    /// Unique identifier of the workspace.
    pub id: String,
    /// Name of the workspace.
    pub name: String,
    /// Path to the game directory.
    /// (Steam/steamapps/common/Crusader Kings III).
    pub game_path: String,
    /// Path to the game data directory
    /// (documents/Paradox Interactive/Crusader Kings III).
    pub game_data_path: String,
    /// All mods available in the workspace.
    pub mod_list: ModList,
    /// Load order of the mods.
    pub mod_load_order: ModLoadOrder,
}
impl Default for Workspace {
    fn default() -> Self {
        Workspace::new()
    }
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            id: String::new(),
            name: String::new(),
            game_path: String::new(),
            game_data_path: String::new(),
            mod_list: ModList::new(),
            mod_load_order: ModLoadOrder::new(),
        }
    }
}

use crate::mods::{ModList, ModLoadOrder};

/// Workspace is a main unit of the application.
/// Before you can work with mods, you need to create a workspace.
#[derive(Default)]
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

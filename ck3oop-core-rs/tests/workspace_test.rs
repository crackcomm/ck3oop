#[test]
fn test_workspace_default() {
    let workspace = ck3oop_core_rs::workspace::Workspace::default();
    assert_eq!(workspace.mod_list.mods.len(), 0);
    assert_eq!(workspace.mod_load_order.mod_list.mods.len(), 0);
}

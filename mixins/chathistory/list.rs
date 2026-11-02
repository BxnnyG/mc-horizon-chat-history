// The mod's row in Options > Mods.
fn chathistory_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "chathistory".into(),
        title: "Long Chat History 1.0.0".into(),
        description: "Keeps 16384 chat messages instead of 100, so you can scroll back much further.".into(),
        keys: Vec::new(),
    });
}

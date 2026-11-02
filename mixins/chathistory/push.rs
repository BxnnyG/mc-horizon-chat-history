// `ChatComponent.addMessageToQueue` as the game has it, keeping 16384
// messages instead of 100 (More Chat History); the game's own is skipped.
fn chathistory_push(
    chat: &mut Chat,
    font: &crate::font::Font,
    message: &Message,
    focused: bool,
) -> core::ops::ControlFlow<()> {
    if !chat.abilities.shows(message.source) {
        return core::ops::ControlFlow::Break(());
    }
    let plain = message.content.plain();
    crate::logging::write_only(log::Level::Info, &format!("[CHAT] {plain}"));
    if chat.log {
        println!("[CHAT] {plain}");
    }
    chat.add_lines(font, message, focused);
    chat.messages.insert(0, message.clone());
    chat.messages.truncate(16384);
    chat.received += 1;
    core::ops::ControlFlow::Break(())
}

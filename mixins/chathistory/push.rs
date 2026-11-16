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
    let mut message = message.clone();
    let plain = message.content.plain();
    crate::logging::write_only(log::Level::Info, &format!("[CHAT] {plain}"));
    if chat.log {
        println!("[CHAT] {plain}");
    }
    message.time = chat.now();
    chat.received += 1;
    if chat.qol.compact
        && let Some(last) = chat.messages.first()
        && repeats(last, &message)
    {
        let mut last = chat.messages.remove(0);
        let shown = last.wrapped.min(chat.lines.len());
        chat.lines.drain(..shown);
        let (scroll, new_since_scroll) = (chat.scroll, chat.new_since_scroll);
        last.repeats += 1;
        last.added = message.added;
        last.time = message.time;
        last.wrapped = chat.add_lines(font, &last, focused);
        if scroll > 0 {
            chat.scroll = scroll;
            chat.new_since_scroll = new_since_scroll;
            chat.scroll_by(last.wrapped as i32 - shown as i32, focused);
        }
        chat.messages.insert(0, last);
        return core::ops::ControlFlow::Break(());
    }
    message.wrapped = chat.add_lines(font, &message, focused);
    chat.messages.insert(0, message);
    chat.messages.truncate(16384);
    core::ops::ControlFlow::Break(())
}

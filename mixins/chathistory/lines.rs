// `GuiMessage.splitLines` into the chat as the game does it, keeping 16384
// lines instead of 100 (More Chat History); the game's own is skipped.
fn chathistory_lines(
    chat: &mut Chat,
    font: &crate::font::Font,
    message: &Message,
    focused: bool,
) -> core::ops::ControlFlow<usize> {
    let whole = message.content.strip_legacy().plain();
    if let Some(query) = &chat.search
        && !whole.to_lowercase().contains(query.as_str())
    {
        return core::ops::ControlFlow::Break(0);
    }
    let whole: std::sync::Arc<str> = whole.into();
    let mut count = 0;
    let icon = if message.tag.as_ref().is_some_and(|tag| tag.icon) {
        9 + 4 + 2
    } else {
        0
    };
    let width = chat.max_width() - icon;
    let content = chat.message_text(message);
    for line in font.wrap_components(&content, width) {
        count += 1;
        if focused && chat.scroll > 0 {
            chat.new_since_scroll = true;
            chat.scroll_by(1, focused);
        }
        chat.lines.insert(
            0,
            Line {
                added: message.added,
                content: line,
                tag: message.tag.clone(),
                message: whole.clone(),
            },
        );
    }
    chat.lines.truncate(16384);
    core::ops::ControlFlow::Break(count)
}

// `GuiMessage.splitLines` into the chat as the game does it, keeping 16384
// lines instead of 100 (More Chat History); the game's own is skipped.
fn chathistory_lines(
    chat: &mut Chat,
    font: &crate::font::Font,
    message: &Message,
    focused: bool,
) -> core::ops::ControlFlow<()> {
    let icon = if message.tag.as_ref().is_some_and(|tag| tag.icon) {
        9 + 4 + 2
    } else {
        0
    };
    let width = chat.max_width() - icon;
    let content = chat.displayed(&message.content);
    for line in font.wrap_components(&content, width) {
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
            },
        );
    }
    chat.lines.truncate(16384);
    core::ops::ControlFlow::Break(())
}

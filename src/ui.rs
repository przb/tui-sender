use std::path::PathBuf;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListState, StatefulWidget},
};

pub struct MsgListWidget<'a> {
    paths: &'a [PathBuf],
    selected_index: usize,
}

impl<'a, 'b> MsgListWidget<'a> {
    pub fn new(paths: &'a [PathBuf], selected_index: usize) -> Self {
        Self {
            paths,
            selected_index,
        }
    }
    pub fn get_state(&self) -> ListState {
        ListState::default().with_selected(self.selected_index.into())
    }
}

impl StatefulWidget for &MsgListWidget<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let list = List::new(self.paths.iter().map(|pb| pb.to_string_lossy()))
            .block(Block::bordered().title("Select a Message to Send"))
            .style(Style::new().white())
            .highlight_style(Style::new().italic().bg(Color::Cyan).fg(Color::Black))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);
        list.render(area, buf, state);
    }
}

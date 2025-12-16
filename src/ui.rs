use std::path::{Path, PathBuf};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListState, StatefulWidget},
};

pub struct MsgListWidget {
    paths: Vec<PathBuf>,
    selected: usize,
}

impl MsgListWidget {
    pub fn new(paths: impl IntoIterator<Item = impl AsRef<Path>>, selected_index: usize) -> Self {
        Self {
            paths: paths
                .into_iter()
                .map(|p| PathBuf::from(p.as_ref()))
                .collect(),
            selected: selected_index,
        }
    }
}

impl StatefulWidget for &MsgListWidget {
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

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, List, Widget},
};

pub struct MsgListWidget {
    paths: Vec<PathBuf>,
}

impl MsgListWidget {
    pub fn new(paths: impl IntoIterator<Item = impl AsRef<Path>>) -> Self {
        Self {
            paths: paths
                .into_iter()
                .map(|p| PathBuf::from(p.as_ref()))
                .collect(),
        }
    }
}

impl Widget for &MsgListWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let list = List::new(self.paths.iter().map(|pb| pb.to_string_lossy()))
            .block(Block::bordered().title("Select a Message to Send"))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);
        list.render(area, buf);
    }
}

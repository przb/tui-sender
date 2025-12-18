use std::path::{Path, PathBuf};

use super::actions;
use color_eyre::{Result, eyre::Context};
use ratatui::{
    crossterm::event,
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use walkdir::WalkDir;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[expect(unused)]
pub enum AppStatus {
    Sending,
    WaitingResponse,
    Idle,
    Exit,
}

fn read_messages(messages_dir: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(messages_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.file_type().is_file().then_some(e.into_path()))
        .collect()
}

pub struct Store {
    status: AppStatus,
    msgs_dir: PathBuf,
    msgs_list: Vec<PathBuf>,
    /// index of the active message currently selected
    active_msg_selection: usize,
}
impl Store {
    pub fn from_args(args: crate::args::Args) -> Self {
        let messages_dir = args.messages_dir.unwrap_or("messages".into());
        Self {
            msgs_list: read_messages(&messages_dir),
            msgs_dir: messages_dir,
            status: AppStatus::Idle,
            active_msg_selection: 0,
        }
    }

    pub fn run<T: Backend>(mut self, terminal: &mut Terminal<T>) -> Result<()> {
        while self.status != AppStatus::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            let event = self.receive_events()?;

            if let Some(event) = event {
                self.update(event);
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame<'_>) {
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        let msg_list = crate::ui::MsgListWidget::new(&self.msgs_list, self.active_msg_selection);

        frame.render_stateful_widget(&msg_list, outer_layout[0], &mut msg_list.get_state());

        frame.render_widget(
            Paragraph::new(super::LONG_TEXT)
                .block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL))
                .wrap(Wrap { trim: false }),
            outer_layout[1],
        );
    }

    fn update(&mut self, action: actions::Action) {
        match action {
            actions::Action::IncrementSelection => {
                self.active_msg_selection = if self.active_msg_selection >= self.msgs_list.len() - 1
                {
                    0
                } else {
                    self.active_msg_selection + 1
                };
            }

            actions::Action::DecrementSelection => {
                self.active_msg_selection = if self.active_msg_selection <= 0 {
                    self.msgs_list.len() - 1
                } else {
                    self.active_msg_selection - 1
                };
            }
            actions::Action::RefreshList => {
                self.msgs_list = read_messages(&self.msgs_dir);
            }
            actions::Action::QuitProgram => self.status = AppStatus::Exit,
        }
    }

    /// blocks until an event is received
    fn receive_events(&mut self) -> Result<Option<actions::Action>> {
        let event = event::read().context("failed to read event")?;
        Ok(match event {
            event::Event::Key(key_event) => self.handle_key_event(key_event),
            // do nothing
            _ => None,
        })
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Option<actions::Action> {
        use actions::Action;
        match key_event.code {
            event::KeyCode::Char('q') => Some(Action::QuitProgram),
            event::KeyCode::Char('r') => Some(Action::RefreshList),
            event::KeyCode::Char('j') | event::KeyCode::Down => Some(Action::IncrementSelection),
            event::KeyCode::Char('k') | event::KeyCode::Up => Some(Action::DecrementSelection),
            // do nothing
            _ => None,
        }
    }
}

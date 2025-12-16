pub mod client;
#[expect(unused)]
pub mod meta;
pub mod msgs;
#[expect(unused)]
pub mod people_info;
pub mod server;

use std::{
    path::{Path, PathBuf},
    sync::mpsc,
};

use itertools::Itertools;
use ratatui::{
    Terminal,
    crossterm::event,
    layout::{Constraint, Direction, Layout},
    prelude::{Backend, Stylize},
    style::Color,
    widgets::{Block, Borders, Paragraph},
};
use walkdir::WalkDir;

use crate::args::Args;

use color_eyre::eyre::{Context, Result};

#[expect(unused)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AppStatus {
    Sending,
    WaitingResponse,
    Idle,
    Exit,
}

pub struct App {
    pub status: AppStatus,
    messages_dir: PathBuf,
    paths: Vec<PathBuf>,
}

fn read_messages(messages_dir: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(messages_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.file_type().is_file().then_some(e.into_path()))
        .collect()
}
impl App {
    #[expect(unused)]
    pub fn new() -> Self {
        Self {
            status: AppStatus::Idle,
            messages_dir: PathBuf::default(),
            paths: Vec::new(),
        }
    }

    pub(crate) fn from_args(args: crate::args::Args) -> Self {
        let messages_dir = args.messages_dir.unwrap_or("messages".into());
        Self {
            paths: read_messages(&messages_dir),
            messages_dir,
            status: AppStatus::Idle,
        }
    }

    #[expect(unused)]
    fn old_main(&self) -> Result<()> {
        let (ready_tx, ready_rx) = mpsc::channel();
        let client_addr = String::from("127.0.0.1:8888");
        let server_addr = client_addr.clone();

        let client_jh = std::thread::spawn(|| client::client(client_addr, ready_rx));
        let server_jh = std::thread::spawn(|| server::server(server_addr, ready_tx));

        let () = server_jh.join().unwrap().context("server failure")?;
        let () = client_jh.join().unwrap().context("client failed")?;

        Ok(())
    }
    pub fn run<T: Backend>(mut self, terminal: &mut Terminal<T>) -> Result<()> {
        while self.status != AppStatus::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame<'_>) {
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());
        frame.render_widget(
            Paragraph::new("some text")
                .block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
            outer_layout[0],
        );

        let msg_list = crate::ui::MsgListWidget::new(self.paths.clone());

        frame.render_widget(&msg_list, outer_layout[1]);
    }
    fn handle_events(&mut self) -> Result<()> {
        let event = event::read().context("failed to read event")?;
        match event {
            event::Event::Key(key_event) if matches!(key_event.code, event::KeyCode::Char('q')) => {
                self.status = AppStatus::Exit;
            }
            _ => {
                // do nothing
            }
        }

        Ok(())
    }
}

// impl Widget for &App {
//     fn render(self, area: Rect, buf: &mut Buffer)
//     where
//         Self: Sized,
//     {
//         let title = ratatui::prelude::Line::from(" Message Sender ".bold());

//         let instructions = Line::from(vec!["decrement".into(), "<Left>".bold().blue()]);

//         area.
//     }
// }

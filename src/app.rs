pub mod client;
pub mod meta;
pub mod msgs;
pub mod people_info;
pub mod server;

pub enum AppStatus {
    Sending,
    WaitingResponse,
    Idle,
}
pub struct App {
    pub status: AppStatus,
}

impl App {
    pub fn new() -> Self {
        Self {
            status: AppStatus::Idle,
        }
    }
}

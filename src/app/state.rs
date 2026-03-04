use crate::data::models::{CiStatus, GitStatus, QualityMetrics, TasksStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Git,
    CiCd,
    Tasks,
    Quality,
}

impl ActivePanel {
    pub fn next(self) -> Self {
        match self {
            Self::Git => Self::CiCd,
            Self::CiCd => Self::Tasks,
            Self::Tasks => Self::Quality,
            Self::Quality => Self::Git,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Git => Self::Quality,
            Self::CiCd => Self::Git,
            Self::Tasks => Self::CiCd,
            Self::Quality => Self::Tasks,
        }
    }
}

pub struct App {
    pub should_quit: bool,
    pub active_panel: ActivePanel,
    pub git_status: Option<GitStatus>,
    pub ci_status: Option<CiStatus>,
    pub tasks_status: Option<TasksStatus>,
    pub quality_metrics: Option<QualityMetrics>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            active_panel: ActivePanel::Git,
            git_status: None,
            ci_status: None,
            tasks_status: None,
            quality_metrics: None,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next_panel(&mut self) {
        self.active_panel = self.active_panel.next();
    }

    pub fn prev_panel(&mut self) {
        self.active_panel = self.active_panel.prev();
    }
}

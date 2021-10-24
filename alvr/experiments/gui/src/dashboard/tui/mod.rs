mod events;
mod repl;

use self::{events::EventsPanel, repl::ReplPanel};
use alvr_common::ServerEvent;
use alvr_session::SessionDesc;
use std::{
    collections::VecDeque,
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{canvas::Line, Block, Borders},
    Terminal,
};

pub struct Dashboard {
    unprocessed_events: Arc<Mutex<Vec<ServerEvent>>>,

    running: Arc<AtomicBool>,
}

impl Dashboard {
    pub fn new(_: SessionDesc) -> Self {
        Self {
            unprocessed_events: Arc::new(Mutex::new(vec![])),
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn run(&self, mut request_handler: impl FnMut(String) -> String) {
        let stdout = io::stdout().into_raw_mode().unwrap();
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        let key_events = Arc::new(Mutex::new(VecDeque::new()));
        thread::spawn({
            let key_events = Arc::clone(&key_events);
            let running = Arc::clone(&self.running);
            move || {
                for event in io::stdin().keys() {
                    if let Ok(event) = event {
                        key_events.lock().unwrap().push_back(event);
                    }

                    if !running.load(Ordering::Relaxed) {
                        break;
                    }
                }
            }
        });

        let mut events_panel = EventsPanel::new();
        let mut repl_panel = ReplPanel::new();

        while self.running.load(Ordering::Relaxed) {
            events_panel.push_events(
                self.unprocessed_events
                    .lock()
                    .unwrap()
                    .drain(..)
                    .collect::<Vec<_>>(),
            );

            terminal
                .draw(|frame| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Percentage(50),
                            Constraint::Length(1),
                            Constraint::Percentage(50),
                        ])
                        .split(frame.size());

                    events_panel.draw(frame, chunks[0]);
                    frame.render_widget(Block::default().borders(Borders::TOP), chunks[1]);
                    repl_panel.draw(frame, chunks[2]);
                })
                .unwrap();

            while let Some(key) = key_events.lock().unwrap().pop_front() {
                if let Key::Ctrl('c') = key {
                    self.running.store(false, Ordering::Relaxed);
                    request_handler("quit()".into());
                } else {
                    repl_panel.react_to_key(key, &mut request_handler);
                }
            }
        }
    }

    pub fn update_session(&self, _: SessionDesc) {}

    pub fn report_event(&self, event: ServerEvent) {
        self.unprocessed_events.lock().unwrap().push(event);
    }

    pub fn request_exit(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

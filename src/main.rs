use std::{error::Error, io, thread, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    prelude::{Backend, CrosstermBackend},
};

use crate::{app::App, client::Client, server::Server, ui::setup_ui};

pub mod app;
mod client;
mod lamport_clock;
mod server;
pub mod ui;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const NODE_IDS: [&str; 3] = ["A", "B", "C"];

fn main() {
    thread::spawn(|| {
        let server = Server::new(SERVER_ADDRESS);
        let _ = server.run();
    });

    // Wait for server to start
    thread::sleep(Duration::from_millis(100));

    let _client_handles = NODE_IDS.map(|node_id| {
        thread::spawn(move || {
            let client = Client::new(node_id);
            client.run(SERVER_ADDRESS);
        })
    });

    // Run the UI
    if let Err(e) = handle_ui() {
        eprintln!("Error running UI: {}", e);
    }
}

fn handle_ui() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdio = io::stdout();
    execute!(stdio, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdio);
    let mut terminal = Terminal::new(backend)?;

    // Create the app and run it
    let mut app = App::default();
    let _ = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}

fn run_app<B: Backend<Error = io::Error>>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            setup_ui(frame, app);
        })?;
    }
}

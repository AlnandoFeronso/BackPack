use core::hash;
use std::thread;
use std::time::Duration;
use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use tui::widgets::Block;
use tui::widgets::Borders;
use std::io;
//
//WORKSPACE     1      2     3
//
//          [     ]  [    ] [ ]
//
//
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::hash::DefaultHasher;
use tui::backend::CrosstermBackend;
use tui::*;
use tui::widgets;
#[derive(Debug, Serialize, Deserialize )]
enum Colour {
    Green,
    Magenta,
    Cyan,
    Pink,
}

#[derive(Debug, Serialize, Deserialize )]
struct Workspace {
    name: String,
    done: VecDeque<String>,
    todo: VecDeque<String>,
    current: VecDeque<String>,
    colour: Colour,
}

impl Workspace {
    fn new(workspace_name: String) -> Workspace {
        return Workspace {
            name: workspace_name,
            done: VecDeque::new(),
            todo: VecDeque::new(),
            current: VecDeque::new(),
            colour: Colour::Pink,
        };
    }
}

fn read_save(workspace_name: String) -> Workspace {
    let path = format!("~/.config/backpack/workspaces/{workspace_name}");
    println!("reading from: {path}");
    let serialized = fs::read_to_string(path).unwrap_or(String::from(""));
    if serialized == "" {
        return Workspace::new(workspace_name);
    }
    return serde_json::from_str(&serialized).unwrap();
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("ok").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;


    thread::sleep(Duration::from_millis(5000));

    disable_raw_mode()?;
    execute!(
       terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture 
)?;
    terminal.show_cursor()?;

    Ok(())
}

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

// If you switch to ratatui, just change `tui` to `ratatui` here
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders};
#[derive(Debug, Serialize, Deserialize)]
enum Colour {
    Green,
    Magenta,
    Cyan,
    Pink,
}

enum InputMode {
    TextMode,
    BlockMode,
    VisualMode,
}

#[derive(Debug, Serialize, Deserialize)]
struct Workspace {
    name: String,
    done: VecDeque<String>,
    todo: VecDeque<String>,
    current: VecDeque<String>,
    colour: Colour,
}
struct Pos{
    x: usize,
    y: usize,
}

impl Pos{
    const fn new() -> Self{
        Self{
            x: 0,
            y: 0,
        }
    }
    fn set(&mut self , x: usize, y: usize){
        self.x = x;
        self.y = y;
    }
    const fn from(x: usize, y:usize) -> Self{
        Self { x, y }
    }
}

struct App {
    workspace: Option<Workspace>,
    box_pos: Pos,
    character_pos: Pos,
    input_mode: InputMode,
}

impl App {
    fn new() -> Self {
        Self {
            workspace: None,
            box_pos: Pos::new(),
            character_pos: Pos::new(),
            input_mode: InputMode::TextMode,
        }
    }

    fn move_cursor_left(&mut self){

    }
}



impl Workspace {
    fn new() -> Self {
        Self {
            name: String::from("new_workspace"),
            done: VecDeque::new(),
            todo: VecDeque::new(),
            current: VecDeque::new(),
            colour: Colour::Pink,
        }
    }
}

fn read_save(workspace_name: String) -> Workspace {
    let home_dir = env::var("HOME").expect("Failed to find HOME directory");
    let path: PathBuf = [
        &home_dir,
        ".config",
        "backpack",
        "workspaces",
        &workspace_name,
    ]
    .iter()
    .collect();

    let path_str = path.to_string_lossy();

    let serialized = fs::read_to_string(&path).unwrap_or_else(|_| String::from(""));

    if serialized.is_empty() {
        return Workspace::new();
    }

    serde_json::from_str(&serialized).unwrap_or_else(|_| Workspace::new())
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    terminal.draw(|f| {
        let size = f.area();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            )
            .split(size);

        let block1 = Block::default().title("ok1").borders(Borders::ALL);
        f.render_widget(block1, chunks[0]);

        let block2 = Block::default().title("ok2").borders(Borders::ALL);
        f.render_widget(block2, chunks[1]);

        let block3 = Block::default().title("ok3").borders(Borders::ALL);
        f.render_widget(block3, chunks[2]);
    })?;

    thread::sleep(Duration::from_millis(5000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

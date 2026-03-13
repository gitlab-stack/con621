mod api;
mod app;
mod img;
mod ui;

// in retrospect sshgoon would be a funnier name. oh well. the only remnant you're getting of that name is this niche code comment. and this commit.
// look at this nerd reading the source code. would you really not trust ME! 
use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::prelude::*;
use app::{App, Screen, InputTarget};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    if let Err(e) = res {
        eprintln!("Error: {e}");
    }
    Ok(())
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                return Ok(());
            }

            match app.screen {
                Screen::Search => handle_search(app, key.code),
                Screen::Results => handle_results(app, key.code),
                Screen::Detail => handle_detail(app, key.code),
                Screen::Help => {
                    if matches!(key.code, KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q')) {
                        app.screen = app.prev_screen.clone();
                    }
                }
            }

            if app.should_quit {
                return Ok(());
            }
        }
    }
}

fn handle_search(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.should_quit = true,
        KeyCode::Tab => {
            app.input_target = match app.input_target {
                InputTarget::Tags => InputTarget::Sort,
                InputTarget::Sort => InputTarget::Rating,
                InputTarget::Rating => InputTarget::Tags,
            };
        }
        KeyCode::Enter => {
            app.search();
        }
        KeyCode::Char('?') if !matches!(app.input_target, InputTarget::Tags) => {
            app.prev_screen = app.screen.clone();
            app.screen = Screen::Help;
        }
        KeyCode::Char(c) => {
            if matches!(app.input_target, InputTarget::Tags) {
                app.tag_input.push(c);
            } else if matches!(app.input_target, InputTarget::Sort) {
                app.cycle_sort();
            } else {
                app.cycle_rating();
            }
        }
        KeyCode::Backspace => {
            if matches!(app.input_target, InputTarget::Tags) {
                app.tag_input.pop();
            }
        }
        _ => {}
    }
}

fn handle_results(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.screen = Screen::Search;
        }
        KeyCode::Up | KeyCode::Char('k') => app.prev_post(),
        KeyCode::Down | KeyCode::Char('j') => app.next_post(),
        KeyCode::Enter => {
            if !app.posts.is_empty() {
                app.screen = Screen::Detail;
            }
        }
        KeyCode::Char('n') => app.next_page(),
        KeyCode::Char('p') => app.prev_page(),
        KeyCode::Char('o') => app.open_in_browser(),
        KeyCode::Char('d') => app.download_current(),
        KeyCode::Char('i') => app.toggle_image(),
        KeyCode::Char('?') => {
            app.prev_screen = app.screen.clone();
            app.screen = Screen::Help;
        }
        _ => {}
    }
}

fn handle_detail(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.screen = Screen::Results;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.detail_scroll > 0 {
                app.detail_scroll -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.detail_scroll += 1;
        }
        KeyCode::Char('o') => app.open_in_browser(),
        KeyCode::Char('d') => app.download_current(),
        KeyCode::Left | KeyCode::Char('h') => {
            app.prev_post();
            app.detail_scroll = 0;
            if app.show_image { app.load_image_for_current(); }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.next_post();
            app.detail_scroll = 0;
            if app.show_image { app.load_image_for_current(); }
        }
        KeyCode::Char('i') => app.toggle_image(),
        KeyCode::Char('?') => {
            app.prev_screen = app.screen.clone();
            app.screen = Screen::Help;
        }
        _ => {}
    }
}

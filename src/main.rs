use std::{path::PathBuf, pin::Pin, sync::Arc};

use clap::Parser as _;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use futures::{StreamExt as _, future::join_all, stream::FuturesOrdered};
use tokio::{fs::read_to_string, sync::OnceCell};

use lazy_json::{
    Parser,
    lazy_tree::cursor::{self, Target},
    parser::Builder,
};

use crate::{
    error::Error,
    event::{Event, EventHandler},
    interface::Interface,
    models::column,
    state::{State, Update},
    task::Task,
    terminal::TerminalHandler,
};

mod error;
mod event;
mod interface;
mod models;
mod terminal;

#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    file: PathBuf,
}

enum Remove {
    All,
    Pop { how_much: usize },
}

#[derive(Debug)]
struct Add {
    columns: Box<[column::Model]>,
    update: Update,
}

mod task {
    use super::*;

    pub type Task = Pin<Box<dyn Future<Output = Result<Kind, Error>>>>;

    pub enum Kind {
        UpdateColumns {
            remove: Option<Remove>,
            add: Option<Add>,
        },
        MoveCursor {
            new_path: Box<[usize]>,
        },
    }
}

mod state {

    #[derive(Debug, Default)]
    pub struct State {
        pub exit: bool,
        pub path: Vec<usize>,
        pub left: Box<[usize]>,
        pub right: Box<[usize]>,
        pub preview: bool,
    }

    impl State {
        pub fn update(&mut self, update: Update) {
            self.left = update.left;
            self.right = update.right;
            self.preview = update.preview;
        }
    }

    #[derive(Debug)]
    pub struct Update {
        pub left: Box<[usize]>,
        pub right: Box<[usize]>,
        pub preview: bool,
    }
}

static PARSER: OnceCell<Parser> = OnceCell::const_new();

async fn init(file: PathBuf) -> Result<&'static Parser, Error> {
    assert!(!PARSER.initialized(), "Parser already initialized");

    PARSER
        .get_or_try_init(|| async move {
            Ok(Builder::new(read_to_string(file).await?)
                .with_cache_interval(1)
                .build()?)
        })
        .await
}

async fn parser() -> &'static Parser {
    PARSER.get().expect("Parser is not initialized")
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("{e}");
    }
}

async fn run() -> Result<(), Error> {
    let args = Cli::parse();

    let mut events = EventHandler::new(250);
    let mut terminal = TerminalHandler::new()?;
    let mut interface = Interface::new(terminal.size()?);

    let mut state = State::default();

    let mut tasks: FuturesOrdered<Task> = FuturesOrdered::new();

    tasks.push_back({
        Box::pin(async move {
            init(args.file).await?;

            Ok(task::Kind::MoveCursor {
                new_path: [0, 0].into(),
            })
        })
    });

    while !state.exit {
        interface.draw_on(&mut terminal)?;

        tokio::select! {
            event = events.next() => {
                if let Some(task) = match event {
                    Event::Tick if !tasks.is_empty() => {
                        handle_event::tick(&mut interface)
                    }
                    Event::Resize(width, height) => {
                        handle_event::resize(width, height, &state, &mut interface)
                    }
                    Event::Key(key) => {
                        handle_event::key(key, &mut state, &mut interface)
                    }
                    _ => None,
                } {
                    tasks.push_back(task);
                }
            }
            task = tasks.next(), if !tasks.is_empty() => {
                if let Some(task) = match task.unwrap()? {
                    task::Kind::UpdateColumns { remove, add } => {
                        handle_task::change_columns(remove, add, &mut state, &mut interface)
                    }
                    task::Kind::MoveCursor { new_path } => {
                        handle_task::move_cursor(new_path, &mut state, &mut interface)
                    }
                } {
                    tasks.push_back(task);
                }
            }
        }
    }

    drop(terminal);

    interface
        .model()
        .base()
        .page()
        .overlay()
        .print()
        .entries()
        .iter()
        .for_each(|(_, content)| {
            println!("{}", content);
        });

    Ok(())
}

mod handle_event {
    use super::*;

    pub fn tick(interface: &mut Interface) -> Option<Task> {
        interface.spin();
        None
    }

    pub fn resize(
        width: u16,
        height: u16,
        state: &State,
        interface: &mut Interface,
    ) -> Option<Task> {
        interface.resize(width, height);

        let path = state.path.clone();
        let capacity = interface.table_inner_area().height as usize;
        let skip = 0;

        Some(Box::pin(async move {
            Ok(task::Kind::UpdateColumns {
                remove: Some(Remove::All),
                add: Some(make_columns(path, capacity, skip).await?),
            })
        }))
    }

    pub fn key(
        key: KeyEvent,
        state: &mut State,
        interface: &mut Interface,
    ) -> Option<Task> {
        match key.code {
            KeyCode::Esc => {
                interface.hide_controls();
                None
            }
            KeyCode::Char('?') => {
                interface.show_controls();
                None
            }
            KeyCode::Char(' ') => {
                interface.toggle_overlay();
                None
            }
            KeyCode::Enter => {
                interface.select_entry(state);
                None
            }
            KeyCode::Right if key.modifiers == KeyModifiers::SHIFT => {
                interface.inc_column_width();
                None
            }
            KeyCode::Left if key.modifiers == KeyModifiers::SHIFT => {
                interface.dec_column_width();
                None
            }
            KeyCode::Left if key.modifiers == KeyModifiers::CONTROL => {
                interface.inc_print_width();
                None
            }
            KeyCode::Right if key.modifiers == KeyModifiers::CONTROL => {
                interface.dec_print_width();
                None
            }
            KeyCode::Down => {
                let path = state.path.clone();

                Some(Box::pin(async move {
                    let mut cursor = parser().await.cursor().await?;
                    cursor.go_to(&path).await?;
                    cursor.jump(cursor::Direction::Right).await?;

                    Ok(task::Kind::MoveCursor {
                        new_path: cursor.path().clone().into_boxed_slice(),
                    })
                }))
            }
            KeyCode::Up => {
                let path = state.path.clone();

                Some(Box::pin(async move {
                    let mut cursor = parser().await.cursor().await?;
                    cursor.go_to(&path).await?;
                    cursor.jump(cursor::Direction::Left).await?;

                    Ok(task::Kind::MoveCursor {
                        new_path: cursor.path().clone().into_boxed_slice(),
                    })
                }))
            }
            KeyCode::Right if state.preview => {
                let mut path = state.path.clone();
                path.push(0);

                Some(Box::pin(async move {
                    Ok(task::Kind::MoveCursor {
                        new_path: path.into_boxed_slice(),
                    })
                }))
            }
            KeyCode::Left if state.path.len() > 2 => {
                let mut path = state.path.clone();
                path.pop();

                Some(Box::pin(async move {
                    Ok(task::Kind::MoveCursor {
                        new_path: path.into_boxed_slice(),
                    })
                }))
            }
            KeyCode::Char('c') | KeyCode::Char('C')
                if key.modifiers == KeyModifiers::CONTROL =>
            {
                state.exit = true;

                None
            }
            _ => None,
        }
    }
}

mod handle_task {
    use super::*;

    pub fn change_columns(
        remove: Option<Remove>,
        add: Option<Add>,
        state: &mut State,
        interface: &mut Interface,
    ) -> Option<Task> {
        interface.change_columns(remove, add, state);
        None
    }

    pub fn move_cursor(
        new_path: Box<[usize]>,
        state: &mut State,
        interface: &mut Interface,
    ) -> Option<Task> {
        let (how_much, skip) = if let Some(diff) =
            path_in_borders(&new_path, &state.left, &state.right)
        {
            let skip = diff - 1;
            let how_much = interface.columns_len() - skip;

            (Some(how_much), Some(skip))
        } else {
            if new_path.len() > state.path.len() {
                let skip = interface.columns_len();

                (None, Some(skip))
            } else if new_path.len() < state.path.len() {
                if let Some(diff) = vec_equ(&new_path, &state.path) {
                    let skip = diff;
                    let how_much = interface.columns_len() - skip;

                    (Some(how_much), Some(skip))
                } else {
                    let how_much = state.preview as usize;
                    state.preview = true;

                    (Some(how_much), None)
                }
            } else {
                if let Some(diff) = vec_equ(&new_path, &state.path) {
                    let skip = diff;
                    let how_much = interface.columns_len() - skip;

                    (Some(how_much), Some(skip))
                } else {
                    (None, None)
                }
            }
        };

        state.path = new_path.to_vec();

        match (how_much, skip) {
            (None, None) => None,
            _ => Some({
                let path = new_path.clone();
                let capacity = interface.table_inner_area().height as usize;

                Box::pin(async move {
                    let remove =
                        how_much.map(|how_much| Remove::Pop { how_much });

                    let add = if let Some(skip) = skip {
                        Some(make_columns(path.to_vec(), capacity, skip).await?)
                    } else {
                        None
                    };

                    Ok(task::Kind::UpdateColumns { remove, add })
                })
            }),
        }
    }
}

fn path_in_borders(
    path: &[usize],
    left: &[usize],
    right: &[usize],
) -> Option<usize> {
    assert!(left.len() == right.len());

    for (i, (l, r)) in left.iter().zip(right.iter()).enumerate() {
        if let Some(step) = path.get(i) {
            if *step < *l || *step > *r {
                return Some(i);
            }
        } else {
            break;
        }
    }

    None
}

async fn make_columns(
    path: Vec<usize>,
    capacity: usize,
    mut skip: usize,
) -> Result<Add, Error> {
    assert!(path.len() >= 2);

    let path = Arc::new(path);

    let mut columns = vec![];
    let mut left = vec![0];
    let mut right = vec![0];
    let mut preview = false;

    let mut cursor = parser().await.cursor().await?;

    for (i, step) in path.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let page_start = (*step / capacity) * capacity;

        let total = cursor.total().unwrap();

        left.push(page_start);
        right.push(total.min(page_start + capacity) - 1);

        cursor
            .walk(cursor::Direction::Down(Target::Index(*step)))
            .await?;

        if skip != 0 {
            skip -= 1;
            continue;
        }

        let cursors = (page_start..total.min(page_start + capacity))
            .into_iter()
            .map(|index| {
                let path = path.clone();

                tokio::spawn(async move {
                    let mut cursor = parser().await.cursor().await?;
                    if cursor.go_to(&path[..i]).await?
                        && cursor
                            .walk(cursor::Direction::Down(Target::Index(index)))
                            .await?
                    {
                        let mut path = path[..i].to_vec();
                        path.push(index);

                        Ok::<_, Error>(Some((
                            path.into_boxed_slice(),
                            cursor.content().to_string(),
                        )))
                    } else {
                        Ok(None)
                    }
                })
            })
            .collect::<Vec<_>>();

        let handlers = join_all(cursors).await;

        let mut blanks = Vec::with_capacity(handlers.len());
        for result in handlers {
            if let Some(blank) = result?? {
                blanks.push(blank);
            }
        }

        if !blanks.is_empty() {
            columns
                .push(column::Builder::default().with_entries(blanks).build());
        }
    }

    if skip == 0 {
        let total = cursor.total().unwrap();

        let cursors = (0..total.min(capacity))
            .into_iter()
            .map(|index| {
                let path = path.clone();

                tokio::spawn(async move {
                    let mut cursor = parser().await.cursor().await?;
                    if cursor.go_to(&path).await?
                        && cursor
                            .walk(cursor::Direction::Down(Target::Index(index)))
                            .await?
                    {
                        let mut path = path.to_vec();
                        path.push(index);

                        Ok::<_, Error>(Some((
                            path.into_boxed_slice(),
                            cursor.content().to_string(),
                        )))
                    } else {
                        Ok(None)
                    }
                })
            })
            .collect::<Vec<_>>();

        let handlers = join_all(cursors).await;

        let mut blanks = Vec::with_capacity(handlers.len());
        for result in handlers {
            if let Some(blank) = result?? {
                blanks.push(blank);
            }
        }

        if !blanks.is_empty() {
            preview = true;

            columns
                .push(column::Builder::default().with_entries(blanks).build());
        }
    }

    Ok(Add {
        columns: columns.into_boxed_slice(),
        update: Update {
            left: left.into_boxed_slice(),
            right: right.into_boxed_slice(),
            preview,
        },
    })
}

fn vec_equ<T: PartialEq>(a: &[T], b: &[T]) -> Option<usize> {
    if (a.is_empty() && !b.is_empty()) || (!a.is_empty() && b.is_empty()) {
        return Some(0);
    }

    a.iter()
        .zip(b.iter())
        .enumerate()
        .find_map(|(i, (x, y))| if x != y { Some(i) } else { None })
}

pub fn clip_string(mut string: String, ending: &str, length: usize) -> String {
    if length == 0 {
        string = String::default();
    } else if string.chars().count() > length {
        string = format!(
            "{}{}",
            string
                .chars()
                .take(length.saturating_sub(ending.chars().count()))
                .collect::<String>(),
            ending
        );
    }

    string
}

use std::time::Duration;

use crossterm::event::{KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

#[allow(unused)]
#[derive(Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct EventHandler {
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    pub fn new(rate: u64) -> Self {
        let rate = Duration::from_millis(rate);

        let (sender, receiver) = mpsc::unbounded_channel();

        tokio::spawn({
            let sender = sender.clone();

            async move {
                let mut reader = crossterm::event::EventStream::new();
                let mut tick = tokio::time::interval(rate);

                loop {
                    let delay = tick.tick();
                    let event = reader.next().fuse();

                    tokio::select! {
                        _ = sender.closed() => {
                            break;
                        }
                        _ = delay => {
                            sender.send(Event::Tick).unwrap();
                        }
                        Some(Ok(event)) = event => {
                            match event {
                                crossterm::event::Event::Key(key)
                                    if key.kind == crossterm::event::KeyEventKind::Press =>
                                {
                                    sender.send(Event::Key(key)).unwrap();
                                },
                                crossterm::event::Event::Mouse(mouse) => {
                                    sender.send(Event::Mouse(mouse)).unwrap();
                                },
                                crossterm::event::Event::Resize(x, y) => {
                                    sender.send(Event::Resize(x, y)).unwrap();
                                },
                                _ => {}
                            }
                        }
                    };
                }
            }
        });

        Self { receiver }
    }

    pub async fn next(&mut self) -> Event {
        self.receiver
            .recv()
            .await
            .expect("Could not receive the next event")
    }
}

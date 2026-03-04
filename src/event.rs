use crossterm::event as xt_event;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

#[derive(Debug)]
pub enum EventKind {
    Key(xt_event::KeyCode),
    Resize(u16, u16),
    Shutdown,
}

pub fn init_event_handler(tick_rate: Duration) -> Receiver<EventKind> {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        event_loop(&tx, tick_rate);
        let _ = tx.send(EventKind::Shutdown);
    });

    rx
}

fn is_event_available(tick_rate: Duration) -> Option<bool> {
    xt_event::poll(tick_rate).ok()
}

fn event_loop(tx: &Sender<EventKind>, tick_rate: Duration) -> Option<()> {
    loop {
        if is_event_available(tick_rate)? {
            match xt_event::read() {
                Ok(xt_event::Event::Key(key)) => {
                    if key.kind == xt_event::KeyEventKind::Press {
                        tx.send(EventKind::Key(key.code)).ok()?
                    }
                }

                Ok(xt_event::Event::Resize(columns, rows)) => {
                    tx.send(EventKind::Resize(columns, rows)).ok()?
                }

                Err(_) => return None,
                _ => {}
            }
        }
    }
}

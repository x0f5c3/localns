use std::{
    path::Path,
    pin::Pin,
    sync::mpsc as std_mpsc,
    task::{Context, Poll},
    thread,
    time::Duration,
};

use futures::Stream;
use notify::{recommended_watcher, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent};
use pin_project_lite::pin_project;
use tokio::sync::mpsc as tokio_mpsc;

pub enum FileEvent {
    Create,
    Delete,
    Change,
}

pin_project! {
    pub struct FileEventStream {
        watcher: RecommendedWatcher,
        #[pin]
        receiver: tokio_mpsc::Receiver<FileEvent>,
    }
}

impl Stream for FileEventStream {
    type Item = FileEvent;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        this.receiver.poll_recv(cx)
    }
}

pub fn watch(path: &Path) -> Result<FileEventStream, String> {
    let (std_sender, std_receiver) = std_mpsc::channel();
    let (tk_sender, tk_receiver) = tokio_mpsc::channel(10);

    let mut watcher = new_debouncer(
        Duration::from_secs(30),
        Some(Duration::from_millis(500)),
        std_sender,
    )
    .map_err(|e| format!("Unable to watch file {}: {}", path.display(), e))?;

    let watch_path = match path.parent() {
        Some(parent) => parent,
        None => path,
    };

    watcher
        .watcher()
        .watch(watch_path, RecursiveMode::Recursive)
        .map_err(|e| format!("Unable to watch file {}: {}", path.display(), e))?;

    let target = path.to_owned();
    thread::spawn(move || loop {
        for ev in std_receiver.iter() {
            let event = match ev.map(|c| c) {
                Ok(EventKind::Modify(ModifyKind::ev_path)) => {
                    if target == ev_path {
                        FileEvent::Change
                    } else {
                        continue;
                    }
                }
                Event::NoticeRemove(ev_path) => {
                    if target == ev_path {
                        FileEvent::Delete
                    } else {
                        continue;
                    }
                }
                Event::Create(ev_path) => {
                    if target == ev_path {
                        FileEvent::Create
                    } else {
                        continue;
                    }
                }
                DebouncedEvent::Write(ev_path) => {
                    if target == ev_path {
                        FileEvent::Change
                    } else {
                        continue;
                    }
                }
                DebouncedEvent::Remove(ev_path) => {
                    if target == ev_path {
                        FileEvent::Delete
                    } else {
                        continue;
                    }
                }
                DebouncedEvent::Rename(path1, path2) => {
                    if target == path1 {
                        FileEvent::Delete
                    } else if target == path2 {
                        FileEvent::Create
                    } else {
                        continue;
                    }
                }
                _ => {
                    continue;
                }
            };

            if tk_sender.blocking_send(event).is_err() {
                return;
            }
        }
    });

    Ok(FileEventStream {
        watcher,
        receiver: tk_receiver,
    })
}

use bytes::Bytes;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, Notify};
use crossbeam_skiplist::SkipMap;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct Db {
    shared: Arc<Shared>,
}

#[derive(Debug)]
struct Shared {
    state: DashMap<String, Entry>,
    pub_sub: DashMap<String, broadcast::Sender<Bytes>>,
    // Order expirations by time
    expirations: SkipMap<Instant, String>,
    // Notify background task that a new expiration has been added
    background_task_notify: Notify,
}

#[derive(Debug)]
struct Entry {
    data: Bytes,
    expires_at: Option<Instant>,
}

impl Db {
    pub fn new() -> Self {
        let shared = Arc::new(Shared {
            state: DashMap::new(),
            pub_sub: DashMap::new(),
            expirations: SkipMap::new(),
            background_task_notify: Notify::new(),
        });

        // Start background task
        let shared_clone = shared.clone();
        tokio::spawn(async move {
            purge_expired_tasks(shared_clone).await;
        });

        Self { shared }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        self.shared.state.get(key).and_then(|entry| {
            if let Some(expires_at) = entry.expires_at {
                if Instant::now() >= expires_at {
                    return None;
                }
            }
            Some(entry.data.clone())
        })
    }

    pub fn set(&self, key: String, value: Bytes, expire: Option<Duration>) {
        let expires_at = expire.map(|duration| Instant::now() + duration);
        
        self.shared.state.insert(
            key.clone(),
            Entry {
                data: value,
                expires_at,
            },
        );

        if let Some(at) = expires_at {
            self.shared.expirations.insert(at, key);
            self.shared.background_task_notify.notify_one();
        }
    }

    pub fn subscribe(&self, channel: String) -> broadcast::Receiver<Bytes> {
        let entry = self.shared.pub_sub.entry(channel).or_insert_with(|| {
            let (tx, _) = broadcast::channel(1024);
            tx
        });
        entry.subscribe()
    }

    pub fn publish(&self, channel: &str, message: Bytes) -> usize {
        match self.shared.pub_sub.get(channel) {
            Some(tx) => tx.send(message).unwrap_or(0),
            None => 0,
        }
    }
}

async fn purge_expired_tasks(shared: Arc<Shared>) {
    loop {
        // Check for expired keys
        let now = Instant::now();
        
        while let Some(entry) = shared.expirations.front() {
            if *entry.key() <= now {
                let key = entry.value();
                // Check if it's still expired in the main state (avoid race conditions)
                if let Some(state_entry) = shared.state.get(key) {
                    if let Some(expires_at) = state_entry.expires_at {
                        if expires_at <= now {
                            shared.state.remove(key);
                            debug!("Purged expired key: {}", key);
                        }
                    }
                }
                entry.remove();
            } else {
                break;
            }
        }

        // Wait for next expiration or notification
        let sleep_duration = shared.expirations.front()
            .map(|entry| entry.key().duration_since(now))
            .unwrap_or(Duration::from_secs(60));

        tokio::select! {
            _ = tokio::time::sleep(sleep_duration) => {}
            _ = shared.background_task_notify.notified() => {}
        }
    }
}

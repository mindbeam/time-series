use std::convert::TryInto;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Store {
    inner: Arc<Mutex<Inner>>,
}
struct Inner {
    events: sled::Tree,
    last_event_id: u64,
}

impl Store {
    pub fn open_temp() -> Result<Self, sled::Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();

        Self::open(tmpdirpath)
    }

    #[allow(dead_code)]
    pub fn open(basedir: &std::path::Path) -> Result<Self, sled::Error> {
        let pathbuf = basedir.join(format!("./time-series.sled"));

        let db = sled::open(pathbuf.as_path())?;
        let events = db.open_tree("events")?;

        let last_event_id = events
            .last()
            .map_or(0, |v| v.map_or(0, |(k, _)| read_ne_u64(&k)));

        println!("(INIT) Last Event ID: {}", last_event_id);
        Ok(Store {
            inner: Arc::new(Mutex::new(Inner {
                events,
                last_event_id,
            })),
        })
    }

    pub fn add_event(&self, event: Vec<u8>) {
        self.inner.lock().unwrap().add_event(event)
    }
}

impl Inner {
    pub fn add_event(&mut self, mut event: Vec<u8>) {
        self.last_event_id += 1;
        let event_id = self.last_event_id;
        println!("Last Event ID: {}", event_id);

        self.events.insert(event_id.to_ne_bytes(), event).unwrap();
    }
}

fn read_ne_u64(input: &[u8]) -> u64 {
    let (int_bytes, _rest) = input.split_at(std::mem::size_of::<u64>());
    // *input = rest;
    u64::from_ne_bytes(int_bytes.try_into().unwrap())
}
use std::{convert::TryInto, path::Path};

use module::hubitat::Hubitat;
use store::Store;

mod module;
mod store;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut store = Store::open(Path::new(".")).unwrap();
    store.add_event(b"START".to_vec());
    let he_client = Hubitat::new("ws://192.168.50.44".to_owned(), store);
    he_client.run().await;
    // let server = Server::new(8080);

    // server.run().await;
}

// TODO:
// [ ] JSON Parsing
// [ ] Reparse historical data (bincode?)
// [ ] Low-road plotters implementation (local png export?)
// [ ] Evaluate Arrow
// [ ] Auto-reconnect
// [ ] Run on Raspberry Pi
// [ ] Data export protocol
// [ ] React + Observable + WASM + Websocket
// [ ] Mindbase integration

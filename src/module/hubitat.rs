use futures::StreamExt;
pub use futures_timer::Delay;
use std::time::Duration;
use tokio_tungstenite::connect_async;
use url::Url;

use crate::{
    module::hubitat::model::{Event, DTO},
    store::Store,
};

pub mod model;

pub struct Hubitat {
    url: Url,
    store: Store,
}

impl Hubitat {
    pub fn new(connect_addr: String, store: Store) -> Self {
        let url = url::Url::parse(&connect_addr)
            .unwrap()
            .join("/eventsocket")
            .unwrap();
        println!("{}", url);

        Hubitat { url, store }
    }
    pub async fn run(&self) {
        loop {
            println!("Connecting to Hubitat...");
            match connect_async(&self.url).await {
                Ok((ws_stream, _)) => {
                    println!("WebSocket handshake has been successfully completed");

                    let (write, read) = ws_stream.split();

                    println!("Connected to Hubitat");

                    read.for_each(|message| async {
                        match message {
                            Ok(m) => {
                                let data = m.into_data();
                                if data.len() > 0 {
                                    if let Ok(dto) = serde_json::from_slice::<DTO>(&data) {
                                        // println!("{:?}", dto);
                                        if let Ok(event) = Event::from_dto(dto) {
                                            let encoded = bincode::serialize(&event).unwrap();
                                            self.store.add_event(encoded);
                                            println!("Added Event: {:?}", event);
                                        } else {
                                            println!("Dropped Event - Could not decode Event")
                                        }
                                    } else {
                                        println!("Dropped Event - Could not decode DTO")
                                    }
                                }
                            }
                            Err(e) => println!("Hubitat Message Error: {:?}", e),
                        }
                    })
                    .await;
                }
                Err(e) => {
                    println!("Hubitat Connect Error: {:?}", e);
                }
            }

            Delay::new(Duration::from_secs(2)).await
        }
    }
}

use futures::StreamExt;
pub use futures_timer::Delay;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;
use url::Url;

use crate::store::Store;

// { "source":"DEVICE","name":"switch","displayName" : "Back Yard Market lights", "value" : "off", "unit":"null","deviceId":28,"hubId":0,"installedAppId":0,"descriptionText" : "Back Yard Market lights is off [physical]"}
pub struct Event {
    source: String,
    name: String,
    displayName: String,
    value: String,
    unit: String,
    deviceId: usize,
    hubId: usize,
    installedAppId: usize,
    descriptionText: String,
}

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
                                    self.store.add_event(data);
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

use futures::StreamExt;
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
        let (ws_stream, _) = connect_async(&self.url).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (write, read) = ws_stream.split();

        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
            self.store.add_event(data);
        })
        .await;
    }
}

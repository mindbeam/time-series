
use serde::{Deserialize, Serialize};
use warp::body::bytes;
use warp::Filter;

pub struct Server {
    port: u16,
}

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub async fn run(&self) {
        // let (input_sender, input_receiver) = mpsc::unbounded_channel::<InputMessage>();
        // let hub = self.hub.clone();

        let hubitat_hook = warp::post()
            .and(warp::path("hook"))
            .and(warp::path("hubitat"))
            .and(warp::body::bytes())
            // Only accept bodies smaller than 64kb...
            // .and(warp::body::content_length_limit(1024 * 64))
            // .and(warp::body::json())
            .map(|mut update: bytes::Bytes| {
                // employee.rate = rate;
                println!("{}", String::from_utf8_lossy(&update));
                // warp::reply::json(&employee)
                "ok"
            });

        let shutdown = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
        };

        // serve and bind to respective rules
        let (_, serving) = warp::serve(hubitat_hook)
            .bind_with_graceful_shutdown(([0, 0, 0, 0], self.port), shutdown);

        // let running_hub = self.hub.run(input_receiver);

        serving.await;
        // tokio::select! {
        //     _ = serving => {},
        // }
    }
}

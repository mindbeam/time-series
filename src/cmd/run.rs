use tokio::runtime::Runtime;

use crate::{module::hubitat::Hubitat, store::Store};

pub(crate) fn run(store: Store) -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    let mut rt = Runtime::new()?;

    // Spawn the root task
    rt.block_on(async {
        let he_client = Hubitat::new("ws://192.168.50.44".to_owned(), store);
        he_client.run().await;
    });

    Ok(())
}

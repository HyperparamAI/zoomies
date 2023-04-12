use zoomies::{Config, Event, Metric, UdpClient};

use tokio::io;
use tokio::runtime::Runtime;

fn main() -> io::Result<()> {
    let rt  = Runtime::new().unwrap();

    rt.block_on(async {
        let config = Config::new();
        let client = UdpClient::with_config(config).await?;
        client.send(&Metric::Inc::<u32>("zoomies")).await?;
        let event = Event::new()
            .title("Chungus")
            .text("Big Chungus is back");
        client.send(&event).await?;
        Ok(())
    })
}

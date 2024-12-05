//! This example shows how to publish [ResourceRecord]s directly to the DHT.
//!
//! Change the `Keypair::random()` to your own keypair to publish your own records.
//! Change the `packet.answers` to your own records.
//!
//! run this example from the project root:
//!     $ cargo run --example publish

use tracing::Level;
use tracing_subscriber;

use std::time::Instant;

use pkarr::{Keypair, SignedPacket};

#[cfg(feature = "relay")]
use pkarr::client::relay::Client;
#[cfg(not(feature = "relay"))]
use pkarr::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_env_filter("pkarr")
        .init();

    let client = Client::builder().build()?;

    let keypair = Keypair::random();

    let signed_packet = SignedPacket::builder()
        .txt("_foo".try_into().unwrap(), "bar".try_into().unwrap(), 30)
        .sign(&keypair)?;

    let instant = Instant::now();

    println!("\nPublishing {} ...", keypair.public_key());

    match client.publish(&signed_packet).await {
        Ok(()) => {
            println!(
                "\nSuccessfully published {} in {:?}",
                keypair.public_key(),
                instant.elapsed(),
            );
        }
        Err(err) => {
            println!("\nFailed to publish {} \n {}", keypair.public_key(), err);
        }
    };

    Ok(())
}

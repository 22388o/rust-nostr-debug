use nostr_sdk::prelude::*;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let keys: Keys = Keys::generate();
    let client = Client::new(&keys);

    client.add_relay("wss://relay.damus.io").await?;
    client.connect().await;

    let test_pk =
        PublicKey::from_bech32("npub1zfss807aer0j26mwp2la0ume0jqde3823rmu97ra6sgyyg956e0s6xw445")?;
    let filter = Filter::new().author(test_pk).kind(Kind::Metadata).limit(1);

    client.add_relay("wss://relay.nostr.net").await?;
    client.connect_relay("wss://relay.nostr.net").await?;

    let before = Instant::now();
    let events = client.get_events_of(vec![filter], None).await?;
    println!("event: {:?}", events.first().unwrap());
    println!("Elapsed time: {:.2?}", before.elapsed());

    Ok(())
}

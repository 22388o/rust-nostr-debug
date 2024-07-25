use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let keys: Keys = Keys::generate();
    let signer = NostrSigner::Keys(keys);

    // Config
    let opts = Options::new().automatic_authentication(true);

    // Setup nostr client
    let client = ClientBuilder::default().signer(signer).opts(opts).build();

    client.add_relay("wss://relay.damus.io").await?;
    client.connect().await;

    // Add nip-17 relay
    client.add_relay("wss://auth.nostr1.com").await?;
    client.connect_relay("wss://auth.nostr1.com").await?;

    // Send private message
    let receiver =
        PublicKey::from_bech32("npub1zfss807aer0j26mwp2la0ume0jqde3823rmu97ra6sgyyg956e0s6xw445")
            .unwrap();
    let message = "Hello world";

    match client
        .send_private_msg_to(["wss://auth.nostr1.com"], receiver, message, None)
        .await
    {
        Ok(output) => println!("Ok: {:?}", output),
        Err(e) => println!("Err: {}", e),
    }

    Ok(())
}

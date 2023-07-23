use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = dna::cli::Options::parse();

    let socket = options.bind().await?;
    let message = options.message().await;

    let data = message.as_bytes()?;
    let sent = socket.send(&data).await?;
    if options.verbose() {
        println!("sent {sent} bytes");
    }

    let mut buffer = vec![0; 512];
    let received = socket.recv(&mut buffer).await?;
    if options.verbose() {
        println!("received {received} bytes");
        println!("{buffer:?}");
    }

    let message = dna::protocols::dns::Message::from_bytes(&buffer)?;

    for data in message.answers() {
        println!("{data}");
    }
    for data in message.authorities() {
        println!("{data}");
    }

    Ok(())
}

use clap::Parser;

#[derive(Parser)]
pub struct Options {
    #[clap(short, long, default_value = "1.1.1.1")]
    /// which nameserver to query
    nameserver: std::net::IpAddr,
    #[clap(short, long, default_value = "A")]
    r#type: crate::protocols::dns::RecordType,
    #[clap(short, long, default_value = "false")]
    verbose: bool,
    #[clap(index = 1)]
    /// hostnames to query
    hostnames: Vec<String>,
}

impl Options {
    pub async fn bind(&self) -> std::io::Result<tokio::net::UdpSocket> {
        let socket = tokio::net::UdpSocket::bind(std::net::SocketAddr::V4(
            std::net::SocketAddrV4::new(std::net::Ipv4Addr::UNSPECIFIED, 8080),
        ))
        .await?;
        socket
            .connect(std::net::SocketAddr::new(self.nameserver, 53))
            .await?;
        Ok(socket)
    }

    pub async fn message(&self) -> crate::protocols::dns::Message {
        crate::protocols::dns::Message::question(
            crate::protocols::dns::Header::default()
                .with_qd_count(self.hostnames.len() as u16)
                .with_recursion_desired(),
            self.hostnames
                .iter()
                .map(|hostname| {
                    crate::protocols::dns::Question::new(hostname.to_string())
                        .with_record_type(self.r#type)
                })
                .collect(),
        )
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}

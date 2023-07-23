use super::{DnsError, RecordType, SoaData};

#[derive(Debug)]
pub enum RecordData {
    A(std::net::Ipv4Addr),
    AAAA(std::net::Ipv6Addr),
    CNAME(String),
    SOA(SoaData),
    NS(String),
}

impl std::fmt::Display for RecordData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordData::A(ip) => write!(f, "{ip}"),
            RecordData::AAAA(ip) => write!(f, "{ip}"),
            RecordData::CNAME(_) => todo!(),
            RecordData::NS(_) => todo!(),
            RecordData::SOA(soa) => {
                let refresh = std::time::Duration::from_secs(soa.refresh as _);
                let retry = std::time::Duration::from_secs(soa.retry as _);
                let expire = std::time::Duration::from_secs(soa.expire as _);
                let minimum = std::time::Duration::from_secs(soa.minimum as _);
                write!(
                    f,
                    "{} {} {} {} {} {} {}",
                    soa.mname,
                    soa.rname,
                    soa.serial,
                    humantime::format_duration(refresh),
                    humantime::format_duration(retry),
                    humantime::format_duration(expire),
                    humantime::format_duration(minimum),
                )
            }
        }
    }
}

impl bitstream_io::FromBitStreamWith for RecordData {
    type Error = DnsError;
    type Context = (Vec<u8>, RecordType);

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(
        r: &mut R,
        context: &Self::Context,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let _len: u16 = r.read(16)?;
        let (context, ty) = context;
        match ty {
            RecordType::A => Ok(Self::A(std::net::Ipv4Addr::new(
                r.read(8)?,
                r.read(8)?,
                r.read(8)?,
                r.read(8)?,
            ))),
            RecordType::NS => todo!(),
            RecordType::MD => todo!(),
            RecordType::MF => todo!(),
            RecordType::CNAME => todo!(),
            RecordType::SOA => Ok(Self::SOA(r.parse_with(context)?)),
            RecordType::MB => todo!(),
            RecordType::MG => todo!(),
            RecordType::MR => todo!(),
            RecordType::NULL => todo!(),
            RecordType::WKS => todo!(),
            RecordType::PTR => todo!(),
            RecordType::HINFO => todo!(),
            RecordType::MINFO => todo!(),
            RecordType::MX => todo!(),
            RecordType::TXT => todo!(),
            RecordType::AAAA => Ok(Self::AAAA(std::net::Ipv6Addr::new(
                r.read(16)?,
                r.read(16)?,
                r.read(16)?,
                r.read(16)?,
                r.read(16)?,
                r.read(16)?,
                r.read(16)?,
                r.read(16)?,
            ))),
        }
    }
}

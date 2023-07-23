use super::DnsError;

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum RecordType {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    AAAA = 28,
}

impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordType::A => write!(f, "A"),
            RecordType::NS => todo!(),
            RecordType::MD => todo!(),
            RecordType::MF => todo!(),
            RecordType::CNAME => todo!(),
            RecordType::SOA => write!(f, "SOA"),
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
            RecordType::AAAA => write!(f, "AAAA"),
        }
    }
}

impl bitstream_io::ToBitStream for RecordType {
    type Error = DnsError;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        w.write(16, *self as u16)?;
        Ok(())
    }
}

impl bitstream_io::FromBitStream for RecordType {
    type Error = DnsError;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let code = r.read::<u16>(16)?;
        match code {
            1 => Ok(Self::A),
            2 => Ok(Self::NS),
            3 => Ok(Self::MD),
            4 => Ok(Self::MF),
            5 => Ok(Self::CNAME),
            6 => Ok(Self::SOA),
            7 => Ok(Self::MB),
            8 => Ok(Self::MG),
            9 => Ok(Self::MR),
            10 => Ok(Self::NULL),
            11 => Ok(Self::WKS),
            12 => Ok(Self::PTR),
            13 => Ok(Self::HINFO),
            14 => Ok(Self::MINFO),
            15 => Ok(Self::MX),
            16 => Ok(Self::TXT),
            28 => Ok(Self::AAAA),
            // TODO
            _ => Err(DnsError::InvalidRecordTypeCode(code)),
        }
    }
}

impl std::str::FromStr for RecordType {
    type Err = DnsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RecordType::A),
            "AAAA" => Ok(RecordType::AAAA),
            "CNAME" => Ok(RecordType::CNAME),
            _ => Err(DnsError::InvalidRecordType(s.into())),
        }
    }
}

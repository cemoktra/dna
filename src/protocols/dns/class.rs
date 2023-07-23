use super::DnsError;

#[derive(Clone, Copy, Debug)]
pub enum Class {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl bitstream_io::ToBitStream for Class {
    type Error = DnsError;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        w.write(16, *self as u16)?;
        Ok(())
    }
}

impl bitstream_io::FromBitStream for Class {
    type Error = DnsError;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let code = r.read::<u16>(16)?;
        match code {
            1 => Ok(Self::IN),
            2 => Ok(Self::CS),
            3 => Ok(Self::CH),
            4 => Ok(Self::HS),
            _ => Err(DnsError::InvalidClass(code)),
        }
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::IN => write!(f, "IN"),
            Class::CS => write!(f, "CS"),
            Class::CH => write!(f, "CH"),
            Class::HS => write!(f, "HS"),
        }
    }
}

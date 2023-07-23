use super::DnsError;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NXDomain = 3,
    NotImplmented = 4,
    Refused = 5,
}

impl bitstream_io::ToBitStream for ResponseCode {
    type Error = DnsError;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        w.write(4, *self as u8)?;
        Ok(())
    }
}

impl bitstream_io::FromBitStream for ResponseCode {
    type Error = DnsError;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let code = r.read::<u8>(4)?;
        match code {
            0 => Ok(ResponseCode::NoError),
            1 => Ok(ResponseCode::FormatError),
            2 => Ok(ResponseCode::ServerFailure),
            3 => Ok(ResponseCode::NXDomain),
            4 => Ok(ResponseCode::NotImplmented),
            5 => Ok(ResponseCode::Refused),
            _ => Err(DnsError::InvalidResponseCode(code)),
        }
    }
}

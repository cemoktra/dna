#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum OpCode {
    Query = 0,
    InverseQuery = 1,
    Status = 2,
    Notify = 4,
    Update = 5,
    DSO = 6,
}

impl bitstream_io::ToBitStream for OpCode {
    type Error = anyhow::Error;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        w.write(4, *self as u8)?;
        Ok(())
    }
}

impl bitstream_io::FromBitStream for OpCode {
    type Error = anyhow::Error;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let op_code = r.read::<u8>(4)?;
        match op_code {
            0 => Ok(OpCode::Query),
            1 => Ok(OpCode::InverseQuery),
            2 => Ok(OpCode::Status),
            4 => Ok(OpCode::Notify),
            5 => Ok(OpCode::Update),
            6 => Ok(OpCode::DSO),
            _ => Err(anyhow::anyhow!("{op_code} is not a valid OpCode")),
        }
    }
}

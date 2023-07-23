use super::QName;

#[derive(Debug)]
pub struct SoaData {
    pub mname: String,
    pub rname: String,
    pub serial: u32,
    pub refresh: u32,
    pub retry: u32,
    pub expire: u32,
    pub minimum: u32,
}

impl bitstream_io::FromBitStreamWith for SoaData {
    type Error = anyhow::Error;
    type Context = Vec<u8>;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(
        r: &mut R,
        context: &Self::Context,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let mname: QName = r.parse_with(context)?;
        let rname: QName = r.parse_with(context)?;
        Ok(Self {
            mname: mname.into(),
            rname: rname.into(),
            serial: r.read(32)?,
            refresh: r.read(32)?,
            retry: r.read(32)?,
            expire: r.read(32)?,
            minimum: r.read(32)?,
        })
    }
}

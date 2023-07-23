use super::{Class, QName, RecordData, RecordType};

#[derive(Debug)]
pub struct Record {
    name: String,
    ty: RecordType,
    class: Class,
    ttl: u32,
    data: RecordData,
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration = std::time::Duration::from_secs(self.ttl as _);
        write!(
            f,
            "{} {} {} {} {}",
            self.ty,
            self.name,
            self.class,
            humantime::format_duration(duration),
            self.data
        )
    }
}

impl bitstream_io::FromBitStreamWith for Record {
    type Error = anyhow::Error;
    type Context = Vec<u8>;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(
        r: &mut R,
        context: &Self::Context,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let qname: QName = r.parse_with(context)?;
        let ty: RecordType = r.parse()?;
        Ok(Self {
            name: qname.into(),
            ty,
            class: r.parse()?,
            ttl: r.read(32)?,
            data: r.parse_with(&(context.clone(), ty))?,
        })
    }
}

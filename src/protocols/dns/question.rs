use super::{Class, QName, RecordType, DnsError};

#[derive(Debug)]
pub struct Question {
    name: QName,
    ty: RecordType,
    class: Class,
}

impl bitstream_io::ToBitStream for Question {
    type Error = DnsError;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        w.build(&self.name)?;
        w.build(&self.ty)?;
        w.build(&self.class)?;

        Ok(())
    }
}

impl bitstream_io::FromBitStreamWith for Question {
    type Error = DnsError;
    type Context = Vec<u8>;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(
        r: &mut R,
        context: &Self::Context,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            name: r.parse_with(context)?,
            ty: r.parse()?,
            class: r.parse()?,
        })
    }
}

impl Question {
    pub fn new(hostname: String) -> Self {
        Self {
            name: QName::new(hostname),
            ty: RecordType::A,
            class: Class::IN,
        }
    }

    pub fn with_record_type(self, ty: RecordType) -> Self {
        Self { ty, ..self }
    }
}

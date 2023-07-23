use bitstream_io::{BigEndian, BitRead, BitReader};

#[derive(Debug)]
pub struct QName(String);

impl bitstream_io::ToBitStream for QName {
    type Error = anyhow::Error;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        for name in self.0.split('.') {
            let len = name.len();
            let len = u8::try_from(len).map_err(|err| anyhow::anyhow!(err.to_string()))?;
            if len >= 64 {
                return Err(anyhow::anyhow!("QNAME too long"));
            }

            w.write(8, len)?;
            for byte in name.bytes() {
                w.write(8, byte)?;
            }
        }

        Ok(())
    }
}

impl bitstream_io::FromBitStreamWith for QName {
    type Error = anyhow::Error;
    type Context = Vec<u8>;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(
        r: &mut R,
        context: &Self::Context,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        const POINTER_HEADER: u8 = 0b11000000;

        let mut len = r.read::<u8>(8)?;
        let mut names = Vec::new();
        while len > 0 {
            if len >= POINTER_HEADER {
                let second_byte = r.read::<u8>(8)?;
                let offset =
                    u16::from_be_bytes([len, second_byte]) - ((POINTER_HEADER as u16) << 8);

                let mut cursor = std::io::Cursor::new(&context[offset as usize..]);
                let mut reader = BitReader::endian(&mut cursor, BigEndian);
                let name: QName = reader.parse_with(context)?;
                names.push(name.into());
                return Ok(Self::new(names.join(".")));
            } else {
                let mut name = String::new();
                for _ in 0..len {
                    let byte = r.read::<u8>(8)?;
                    name.push(byte as _);
                }
                names.push(name);
                len = r.read::<u8>(8)?;
            }
        }
        Ok(Self::new(names.join(".")))
    }
}

impl QName {
    pub fn new(mut hostname: String) -> QName {
        // TODO: any part must be max 255 bytes
        if !hostname.ends_with('.') {
            hostname.push('.');
        }
        QName(hostname)
    }
}

impl From<QName> for String {
    fn from(value: QName) -> Self {
        value.0
    }
}

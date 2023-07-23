use super::{OpCode, ResponseCode, DnsError};

#[derive(Debug)]
pub struct Header {
    id: u16,
    query: bool,
    op_code: OpCode,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    z: u8,
    response_code: ResponseCode,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16,
}

impl bitstream_io::ToBitStream for Header {
    type Error = DnsError;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        w.write(16, self.id)?;
        w.write_bit(self.query)?;
        w.build(&self.op_code)?;
        w.write_bit(self.aa)?;
        w.write_bit(self.tc)?;
        w.write_bit(self.rd)?;
        w.write_bit(self.ra)?;
        w.write(3, self.z)?;
        w.build(&self.response_code)?;
        w.write(16, self.qd_count)?;
        w.write(16, self.an_count)?;
        w.write(16, self.ns_count)?;
        w.write(16, self.ar_count)?;

        Ok(())
    }
}

impl bitstream_io::FromBitStream for Header {
    type Error = DnsError;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            id: r.read(16)?,
            query: r.read_bit()?,
            op_code: r.parse()?,
            aa: r.read_bit()?,
            tc: r.read_bit()?,
            rd: r.read_bit()?,
            ra: r.read_bit()?,
            z: r.read(3)?,
            response_code: r.parse()?,
            qd_count: r.read(16)?,
            an_count: r.read(16)?,
            ns_count: r.read(16)?,
            ar_count: r.read(16)?,
        })
    }
}

impl Default for Header {
    fn default() -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen(),
            query: false,
            op_code: OpCode::Query,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: 0,
            response_code: ResponseCode::NoError,
            qd_count: 0,
            an_count: 0,
            ns_count: 0,
            ar_count: 0,
        }
    }
}

impl Header {
    pub fn with_op_code(self, op_code: OpCode) -> Self {
        Self { op_code, ..self }
    }

    pub fn with_recursion_desired(self) -> Self {
        Self { rd: true, ..self }
    }

    pub fn with_qd_count(self, qd_count: u16) -> Self {
        Self { qd_count, ..self }
    }

    pub fn qd_count(&self) -> u16 {
        self.qd_count
    }

    pub fn an_count(&self) -> u16 {
        self.an_count
    }

    pub fn ns_count(&self) -> u16 {
        self.ns_count
    }
}

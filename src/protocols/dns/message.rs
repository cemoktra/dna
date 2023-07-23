use super::{Header, Question, Record};
use bitstream_io::{BigEndian, BitRead, BitReader, BitWrite, BitWriter};

#[derive(Debug)]
pub struct Message {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Record>,
    authorities: Vec<Record>,
    //    additional: Vec<u8>,
}

impl bitstream_io::ToBitStream for Message {
    type Error = anyhow::Error;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        w.build(&self.header)?;
        for question in &self.questions {
            w.build(question)?;
        }

        Ok(())
    }
}

impl bitstream_io::FromBitStreamWith for Message {
    type Error = anyhow::Error;
    type Context = Vec<u8>;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(
        r: &mut R,
        context: &Self::Context,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let header = r.parse::<Header>()?;
        let mut questions = Vec::new();
        for _ in 0..header.qd_count() {
            let question = r.parse_with::<Question>(context)?;
            questions.push(question);
        }

        let mut answers = Vec::new();
        for _ in 0..header.an_count() {
            let answer = r.parse_with::<Record>(context)?;
            answers.push(answer);
        }

        let mut authorities = Vec::new();
        for _ in 0..header.ns_count() {
            let authority = r.parse_with::<Record>(context)?;
            authorities.push(authority);
        }

        Ok(Self {
            header,
            questions,
            answers,
            authorities,
        })
    }
}

impl Message {
    pub fn question(header: Header, questions: Vec<Question>) -> Self {
        Self {
            header,
            questions,
            answers: vec![],
            authorities: vec![],
            //            additional: vec![],
        }
    }

    pub fn answers(&self) -> &[Record] {
        &self.answers
    }

    pub fn authorities(&self) -> &[Record] {
        &self.authorities
    }

    pub fn as_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut data = Vec::with_capacity(512);
        let mut writer = BitWriter::endian(&mut data, BigEndian);
        writer.build(self)?;
        Ok(data)
    }

    pub fn from_bytes(data: &[u8]) -> anyhow::Result<Self> {
        let mut cursor = std::io::Cursor::new(&data);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);
        reader.parse_with(&data.to_vec())
    }
}

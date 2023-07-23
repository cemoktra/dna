mod class;
mod header;
mod message;
mod op_code;
mod qname;
mod question;
mod record;
mod record_data;
mod record_type;
mod response_code;
mod soa;

pub use class::Class;
pub use header::Header;
pub use message::Message;
pub use op_code::OpCode;
pub use qname::QName;
pub use question::Question;
pub use record::Record;
pub use record_data::RecordData;
pub use record_type::RecordType;
pub use response_code::ResponseCode;
pub use soa::SoaData;

#[derive(Debug, thiserror::Error)]
pub enum DnsError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
    #[error("{0} is not a valid class")]
    InvalidClass(u16),
    #[error("{0} is not a valid response code")]
    InvalidResponseCode(u8),
    #[error("{0} is not a valid record type code")]
    InvalidRecordTypeCode(u16),
    #[error("{0} is not a valid record type")]
    InvalidRecordType(String),
    #[error("{0} is not a valid op code")]
    InvalidOpCode(u8),
    #[error("QNAME '{0}' too long")]
    QNameTooLong(String),
}

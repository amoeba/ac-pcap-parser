// Re-export protocol types from acprotocol
pub use acprotocol::network::packet::{PacketHeader, PacketHeaderFlags};
pub use acprotocol::network::reader::BinaryReader;

// Keep local fragment implementation for now (will replace with FragmentAssembler later)
pub mod fragment;
pub use fragment::{Fragment, FragmentGroup, FragmentHeader};

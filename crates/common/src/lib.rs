//! Asheron's Call PCAP Viewer Library
//!
//! This library provides functionality to parse PCAP files containing
//! Asheron's Call network traffic.

pub use acprotocol::enums::PacketHeaderFlags;
use acprotocol::network::packet::PacketHeader;
use acprotocol::network::packet_parser::FragmentAssembler;
use acprotocol::network::pcap::PcapIterator;
use anyhow::{Context, Result};
use serde::Serialize;
use std::io::Read;

pub mod messages;
pub mod packet_flags;
pub mod serialization;
pub mod tree;
pub mod weenie;
pub mod weenie_extractor;

/// UI tab selection
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    #[default]
    Messages,
    Weenies,
}

/// UI view mode
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum ViewMode {
    #[default]
    Tree,
    JSON,
    Binary,
}

/// Sort field options
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum SortField {
    #[default]
    Id,
    Type,
    Direction,
    OpCode,
}

/// Fragment info as stored in packets
#[derive(Debug, Clone, Serialize)]
pub struct FragmentInfo {
    #[serde(rename = "Data")]
    pub data: String, // Base64 encoded
    #[serde(rename = "Count")]
    pub count: u16,
    #[serde(rename = "Received")]
    pub received: usize,
    #[serde(rename = "Length")]
    pub length: usize,
    #[serde(rename = "Sequence")]
    pub sequence: u32,
}

/// A parsed packet with all its data
#[derive(Debug, Clone, Serialize)]
pub struct ParsedPacket {
    #[serde(rename = "Header")]
    pub header: PacketHeader,
    #[serde(rename = "Direction")]
    pub direction: String,
    #[serde(rename = "Messages")]
    pub messages: Vec<serde_json::Value>,
    #[serde(rename = "Fragment")]
    pub fragment: Option<FragmentInfo>,
    #[serde(rename = "Id")]
    pub id: usize,
    #[serde(rename = "Timestamp")]
    pub timestamp: f64, // Seconds since epoch (with microsecond precision)
    #[serde(skip)]
    pub raw_payload: Vec<u8>,
}

/// Main parser for PCAP files
pub struct PacketParser {}

impl PacketParser {
    pub fn new() -> Self {
        Self {}
    }

    /// Parse a PCAP file from a reader
    pub fn parse_pcap<R: Read>(
        &mut self,
        mut reader: R,
    ) -> Result<(
        Vec<ParsedPacket>,
        Vec<messages::ParsedMessage>,
        weenie::WeenieDatabase,
    )> {
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .context("Failed to read pcap data")?;

        self.parse_pcap_bytes(&buffer)
    }

    /// Parse PCAP data from bytes
    pub fn parse_pcap_bytes(
        &mut self,
        buffer: &[u8],
    ) -> Result<(
        Vec<ParsedPacket>,
        Vec<messages::ParsedMessage>,
        weenie::WeenieDatabase,
    )> {
        let mut packets = Vec::new();
        let mut all_messages = Vec::new();
        let mut weenie_db = weenie::WeenieDatabase::new();
        let mut packet_id = 0;
        let mut message_id = 0;

        // Create iterator and assembler
        let iter = PcapIterator::<std::io::Cursor<&[u8]>>::from_bytes(buffer)
            .context("Failed to create pcap iterator")?;
        let mut assembler = FragmentAssembler::new();

        for result in iter {
            let packet = result.context("Failed to read packet")?;

            // Extract timestamp (seconds + microseconds)
            let timestamp = packet.ts_sec as f64 + (packet.ts_usec as f64 / 1_000_000.0);

            // Use FragmentAssembler to parse the packet payload
            // This handles header stripping, fragment assembly, and message parsing
            match assembler.parse_packet_payload(&packet.data) {
                Ok(messages) => {
                    if !messages.is_empty() {
                        // Create a ParsedPacket for this packet
                        // Note: We don't have direct access to the packet header anymore,
                        // so we'll create a minimal packet entry
                        let mut parsed_messages_json = Vec::new();

                        for msg in messages {
                            // Convert acprotocol message to our ParsedMessage format
                            let message_type = msg.message_type.clone();
                            let opcode_str = format!("{:04X}", msg.opcode);

                            // Serialize the message to JSON
                            let data = serde_json::to_value(&msg)
                                .unwrap_or_else(|_| serde_json::json!({}));

                            parsed_messages_json.push(data.clone());

                            // The direction is already a string in the message
                            let direction_str = msg.direction.clone();

                            // Create ParsedMessage
                            let parsed_msg = messages::ParsedMessage {
                                id: message_id,
                                message_type,
                                data,
                                direction: direction_str,
                                opcode: opcode_str,
                                timestamp,
                                raw_bytes: Vec::new(), // We don't have access to raw bytes anymore
                            };

                            all_messages.push(parsed_msg);
                            message_id += 1;
                        }

                        // Create a minimal ParsedPacket
                        // Since we don't parse packet headers directly anymore, we create a stub
                        let parsed_packet = ParsedPacket {
                            header: PacketHeader::with_flags(PacketHeaderFlags::empty()),
                            direction: if !all_messages.is_empty() {
                                all_messages.last().unwrap().direction.clone()
                            } else {
                                "Unknown".to_string()
                            },
                            messages: parsed_messages_json,
                            fragment: None, // Fragment info not available with FragmentAssembler
                            id: packet_id,
                            timestamp,
                            raw_payload: Vec::new(),
                        };

                        packets.push(parsed_packet);
                        packet_id += 1;
                    }
                }
                Err(_e) => {
                    // Skip failed packets silently
                }
            }
        }

        // Extract weenie updates from all messages
        let mut type_counts: std::collections::HashMap<String, (usize, usize)> =
            std::collections::HashMap::new();
        for msg in &all_messages {
            let updates = weenie_extractor::extract_weenie_updates(msg);
            let entry = type_counts
                .entry(msg.message_type.clone())
                .or_insert((0, 0));
            entry.0 += 1; // total messages
            entry.1 += updates.len(); // successful extractions
            for update in updates {
                weenie_db.add_or_update(update);
            }
        }

        eprintln!("\n=== Extraction Summary ===");
        eprintln!("Total messages processed: {}", all_messages.len());
        let mut types: Vec<_> = type_counts.iter().collect();
        types.sort_by_key(|(_, (_, extracted))| std::cmp::Reverse(*extracted));
        for (msg_type, (total, extracted)) in types.iter().take(20) {
            if *extracted > 0 {
                eprintln!(
                    "{}: {} extracted from {} messages",
                    msg_type, extracted, total
                );
            }
        }
        eprintln!(
            "Total message types with 0 extractions: {}",
            types.iter().filter(|(_, (_, e))| *e == 0).count()
        );
        eprintln!("Final weenie count: {}\n", weenie_db.count());

        Ok((packets, all_messages, weenie_db))
    }
}

impl Default for PacketParser {
    fn default() -> Self {
        Self::new()
    }
}

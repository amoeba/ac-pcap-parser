use acprotocol::network::packet_parser::FragmentAssembler;
use acprotocol::network::pcap::PcapIterator;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <pcap_file>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let data = fs::read(path).unwrap_or_else(|e| {
        eprintln!("Failed to read file '{}': {}", path, e);
        std::process::exit(1);
    });

    let mut assembler = FragmentAssembler::new();

    match PcapIterator::<std::io::Cursor<&[u8]>>::from_bytes(&data) {
        Ok(iter) => {
            let mut packet_count = 0;
            let mut message_count = 0;

            for result in iter {
                match result {
                    Ok(packet) => {
                        packet_count += 1;
                        println!(
                            "Packet {}: ts={}.{:06}, size={} bytes",
                            packet_count,
                            packet.ts_sec,
                            packet.ts_usec,
                            packet.data.len()
                        );

                        // Use acprotocol's parse_packet_payload which handles all the header stripping
                        // and fragment assembly for us
                        match assembler.parse_packet_payload(&packet.data) {
                            Ok(messages) => {
                                for msg in messages {
                                    message_count += 1;
                                    println!(
                                        "  Message {}: {} (opcode: 0x{:04X})",
                                        message_count, msg.message_type, msg.opcode
                                    );

                                    // Serialize to JSON to see the full parsed data
                                    if let Ok(json) = serde_json::to_string_pretty(&msg) {
                                        println!("{}", json);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("  Error parsing packet: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading packet: {}", e);
                        break;
                    }
                }
            }
            println!("\nTotal packets: {}", packet_count);
            println!("Total messages: {}", message_count);
        }
        Err(e) => {
            eprintln!("Error opening pcap: {}", e);
        }
    }
}

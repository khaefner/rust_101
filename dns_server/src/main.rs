use std::{
    net::UdpSocket,
    str
};

// Basic DNS header flags for a response
//const DNS_RESPONSE_FLAGS: u16 = 0x8180; // Standard query response, recursion available

const DNS_RESPONSE_FLAGS: u16 = {
    // QR (Query/Response): 1 for response
    let qr_response: u16 = 0b1000_0000_0000_0000;

    // Opcode: 0000 for standard query 
    //(in a response, it usually mirrors the query, bit of a hack that we are not building it from query)
    let opcode_standard: u16 = 0b0000_0000_0000_0000; // No bits set for 0

    // AA (Authoritative Answer): 0 for non-authoritative in this example
    let aa_non_authoritative: u16 = 0b0000_0000_0000_0000;

    // TC (Truncation): 0 for not truncated
    let tc_not_truncated: u16 = 0b0000_0000_0000_0000;

    // RD (Recursion Desired): 1 to indicate it was desired (usually copied from query)
    let rd_desired: u16 = 0b0000_0000_0000_0001; // Corrected: Bit at the least significant position

    // RA (Recursion Available): 1 to indicate it's available
    let ra_available: u16 = 0b0000_0000_0000_0001; // Corrected: Bit at the least significant position

    // Z (Reserved): Must be 0
    let reserved: u16 = 0b0000_0000_0000_0000;

    // Rcode (Response code): 0000 for no error
    let rcode_no_error: u16 = 0b0000_0000_0000_0000;

    qr_response |
    (opcode_standard << 11) | // Shift opcode to its position
    (aa_non_authoritative << 10) |
    (tc_not_truncated << 9) |
    (rd_desired << 8) | // Shift RD to its position
    (ra_available << 7) | // Shift RA to its position
    (reserved << 4) | // Shift reserved bits
    rcode_no_error
};

fn parse_query_domain(query: &[u8]) -> Option<String> {
    let mut index = 12; // Skip the 12-byte header
    let mut domain_parts = Vec::new();
    while index < query.len() {
        let length = query[index] as usize;
        index += 1;
        if length == 0 {
            break;
        }
        if index + length > query.len() {
            return None;
        }
        let part = str::from_utf8(&query[index..index + length]).ok()?;
        domain_parts.push(part.to_string());
        index += length;
    }
    if index + 4 > query.len() { // Ensure there's enough space for type and class
        return None;
    }
    let query_type = u16::from_be_bytes([query[index], query[index + 1]]);
    let query_class = u16::from_be_bytes([query[index + 2], query[index + 3]]);

    if query_type == 1 && query_class == 1 { // Only handle A records for IN class
        Some(domain_parts.join("."))
    } else {
        None
    }
}

fn create_response_packet(query: &[u8], _domain: &str, ip_address: &str) -> Option<Vec<u8>> {
    let mut response = Vec::new();

    // Header
    response.extend_from_slice(&query[0..2]); // Transaction ID (2 bytes)
    response.extend_from_slice(&DNS_RESPONSE_FLAGS.to_be_bytes()); // Flags (2 bytes)
    response.extend_from_slice(&query[4..6]); // Questions Count (2 bytes)
    response.extend_from_slice(&[0x00, 0x01]); // Answer RRs Count (2 bytes)
    response.extend_from_slice(&[0x00, 0x00]); // Authority RRs Count (2 bytes)
    response.extend_from_slice(&[0x00, 0x00]); // Additional RRs Count (2 bytes)

    // Copy Question Section
    let mut question_end = 12;
    while question_end < query.len() && query[question_end] != 0 {
        question_end += 1 + query[question_end] as usize;
    }
    question_end += 1; // Skip null terminator
    question_end += 4; // Skip Type and Class (2 bytes each)

    response.extend_from_slice(&query[12..question_end]);

    // Answer Section
    // Name (pointer to question)
    response.extend_from_slice(&[0xc0, 0x0c]); // 2 bytes
    // Type (A record)
    response.extend_from_slice(&[0x00, 0x01]); // 2 bytes
    // Class (IN)
    response.extend_from_slice(&[0x00, 0x01]); // 2 bytes
    // TTL
    response.extend_from_slice(&300_u32.to_be_bytes()); // 4 bytes
    // RDLength
    response.extend_from_slice(&[0x00, 0x04]); // 2 bytes
    // RData (IP Address)
    for part in ip_address.split('.') {
        if let Ok(byte) = part.parse::<u8>() {
            response.push(byte); // 4 bytes total
        } else {
            return None;
        }
    }

    Some(response)
}

fn main() -> Result<(), std::io::Error> {
    let addr = "0.0.0.0:1053"; //So we don't need root
    let socket = UdpSocket::bind(addr)?;
    println!("DNS server listening on {}", addr);

    let mut buf = [0; 512];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amount, src)) => {
                println!("Received {} bytes from {}", amount, src);
                let query = &buf[..amount];

                if let Some(domain) = parse_query_domain(query) {
                    println!("Query for domain: {}", domain);

                    // **Simple hardcoded DNS resolution logic:**
                    let resolved_ip = match domain.as_str() {
                        "www.example.com" => "192.0.2.1",
                        "www.google.com" => "142.250.185.46",
                        "rust-lang.org" => "104.16.53.67",
                        _ => "127.0.0.1", // Default to localhost for unknown domains
                    };

                    if let Some(response_packet) = create_response_packet(query, &domain, resolved_ip) {
                        socket.send_to(&response_packet, src)?;
                        println!("Sent response to {}", src);
                    } else {
                        eprintln!("Failed to create response packet for {}", domain);
                    }
                } else {
                    eprintln!("Failed to parse DNS query");
                }
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
            }
        }
    }
}
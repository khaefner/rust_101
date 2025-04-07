# Chapter 7 Basic DNS Server



```rust,editable
use std::{
    net::{SocketAddr, UdpSocket},
    str,
    sync::mpsc,
    thread,
};

// A very basic DNS query packet structure (for A records)
fn create_query_packet(domain: &str) -> Vec<u8> {
    let mut packet = Vec::new();

    // Header (simplified)
    packet.extend_from_slice(&[0x12, 0x34]); // Transaction ID (just an example)
    packet.extend_from_slice(&[0x01, 0x00]); // Flags: Standard query
    packet.extend_from_slice(&[0x00, 0x01]); // Questions: 1
    packet.extend_from_slice(&[0x00, 0x00]); // Answer RRs: 0
    packet.extend_from_slice(&[0x00, 0x00]); // Authority RRs: 0
    packet.extend_from_slice(&[0x00, 0x00]); // Additional RRs: 0

    // Question section:
    for part in domain.split('.') {
        packet.push(part.len() as u8);
        packet.extend_from_slice(part.as_bytes());
    }
    packet.push(0); // Null terminator

    packet.extend_from_slice(&[0x00, 0x01]); // Type A (IPv4 address)
    packet.extend_from_slice(&[0x00, 0x01]); // Class IN (Internet)

    packet
}

fn parse_response(response: &[u8]) -> Option<String> {
    // This is a very simplified parsing logic and might not handle all cases.
    // A proper DNS parsing library would be much more robust.

    // Skip header (12 bytes)
    if response.len() < 12 {
        return None;
    }
    let mut index = 12;

    // Skip question section (we need to parse the domain name length)
    while index < response.len() && response[index] != 0 {
        index += response[index] as usize + 1;
    }
    index += 1; // Skip the null terminator
    index += 4; // Skip type and class

    // Now we should be at the answer section
    if index >= response.len() {
        return None;
    }

    // Skip name pointer (usually 0xc0 followed by an offset)
    if response[index] == 0xc0 {
        index += 2;
    } else {
        // Handle cases where the full name is repeated (more complex)
        return None; // For simplicity, we'll just handle the pointer case
    }

    index += 2; // Skip type
    index += 2; // Skip class
    let ttl = u32::from_be_bytes([response[index], response[index + 1], response[index + 2], response[index + 3]]);
    index += 4; // Skip TTL
    let rd_length = u16::from_be_bytes([response[index], response[index + 1]]) as usize;
    index += 2; // Skip RDLength

    if response.len() >= index + rd_length && rd_length == 4 {
        let ip_bytes = &response[index..index + rd_length];
        Some(format!(
            "{}.{}.{}.{}",
            ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]
        ))
    } else {
        None
    }
}

fn resolve(domain: &str) -> Result<Option<String>, std::io::Error> {
    let server_address: SocketAddr = "8.8.8.8:53".parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let query_packet = create_query_packet(domain);

    socket.send_to(&query_packet, server_address)?;

    let mut buffer = [0; 512];
    let (amount, _) = socket.recv_from(&mut buffer)?;

    Ok(parse_response(&buffer[..amount]))
}

fn main() {
    let domains_to_resolve = vec!["www.example.com", "www.google.com", "rust-lang.org"];
    let num_domains = domains_to_resolve.len();
    let (tx, rx) = mpsc::channel();

    for domain in domains_to_resolve {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            println!("Resolving {}...", domain);
            match resolve(domain) {
                Ok(Some(ip)) => tx_clone.send(Ok((domain, ip))).unwrap(),
                Ok(None) => tx_clone.send(Err((domain, "No IP found".to_string()))).unwrap(),
                Err(e) => tx_clone.send(Err((domain, format!("Error: {}", e)))).unwrap(),
            }
        });
    }

    for _ in 0..num_domains {
        match rx.recv().unwrap() {
            Ok((domain, ip)) => println!("{}: {}", domain, ip),
            Err((domain, error)) => println!("{}: {}", domain, error),
        }
    }
}
```
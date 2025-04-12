<link rel="stylesheet" href="star.css">

# Chapter 7 Basic DNS Server

Now we are going to build a basic DNS server.  



Let's break down this Rust code step by step. This program implements a very basic DNS resolver that sends UDP queries to a DNS server (specifically Google's public DNS server at 8.8.8.8) and attempts to parse the response to find the IP address for a given domain.

1. Import Statements:
Rust

use std::{
    net::{SocketAddr, UdpSocket},
    str,
    sync::mpsc,
    thread,
};

    std::net::{SocketAddr, UdpSocket}: This imports necessary networking types.
        SocketAddr: Represents a network address (IP address and port number).
        UdpSocket: Provides functionality for sending and receiving data over UDP (User Datagram Protocol). DNS queries are typically sent over UDP.
    std::str: This module provides utilities for working with strings. Although not directly used in this snippet, it's often used for string manipulation.
    std::sync::mpsc: This imports the multi-producer, single-consumer channel type, which is used for communication between threads.
    std::thread: This module provides the ability to create and manage threads, allowing for concurrent execution.

2. create_query_packet Function:
Rust

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

This function takes a domain name as input and constructs a basic DNS query packet in the format expected by DNS servers.

    let mut packet = Vec::new();: Initializes an empty vector of bytes to store the DNS query packet.
    Header (Simplified): The first 12 bytes of a DNS packet form the header. This code creates a very simplified header:
        packet.extend_from_slice(&[0x12, 0x34]);: Sets a Transaction ID. This is a unique identifier for the query, which the server will include in its response. The value 0x1234 is just an example.
        packet.extend_from_slice(&[0x01, 0x00]);: Sets the Flags. 0x0100 indicates a standard query.
        packet.extend_from_slice(&[0x00, 0x01]);: Sets the Questions count to 1, indicating that we are asking one question (for the IP address of the domain).
        The remaining header fields (Answer RRs, Authority RRs, Additional RRs) are set to 0 in this simplified query.
    Question Section: This section specifies the question being asked (i.e., the domain name).
        for part in domain.split('.') { ... }: The domain name is split into its parts (e.g., "www", "example", "com") based on the dots.
        packet.push(part.len() as u8);: For each part, its length is pushed as a single byte.
        packet.extend_from_slice(part.as_bytes());: The bytes of the domain part are then added to the packet.
        packet.push(0);: The question section ends with a null byte.
        packet.extend_from_slice(&[0x00, 0x01]);: Specifies the Query Type. 0x0001 represents an "A" record, which asks for the IPv4 address of the domain.
        packet.extend_from_slice(&[0x00, 0x01]);: Specifies the Query Class. 0x0001 represents the "IN" (Internet) class, which is the most common.
    packet: The function returns the constructed DNS query packet as a vector of bytes.

3. parse_response Function:
Rust

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

This function takes the raw byte response from the DNS server and attempts to parse it to extract the IP address. It's important to note that this is a very simplified parser and won't handle all possible DNS response formats. A robust DNS client would use a dedicated DNS parsing library.

    if response.len() < 12 { return None; }: Checks if the response is at least 12 bytes long (the minimum size of a DNS header). If not, it's an invalid response.
    let mut index = 12;: Initializes an index to keep track of the current position in the response byte array, starting after the header.
    Skip Question Section: This part of the code tries to skip over the question section of the response, which mirrors the question we sent.
        while index < response.len() && response[index] != 0 { ... }: It iterates through the bytes until it finds a null byte, which marks the end of the domain name in the question section. Each label in the domain name is preceded by its length.
        index += response[index] as usize + 1;: Increments the index by the length of the current label plus one (for the length byte itself).
        index += 1;: Skips the null terminator.
        index += 4;: Skips the Query Type and Query Class (2 bytes each) in the question section.
    Answer Section: This is where the IP address is located.
        if index >= response.len() { return None; }: Checks if the index has gone beyond the bounds of the response.
        Skip Name Pointer: In DNS responses, the domain name in the answer section is often compressed using a pointer back to the question section. This code only handles the common case where the first two bytes of the answer record are 0xc0 followed by an offset.
            if response[index] == 0xc0 { index += 2; } else { return None; }: If the first byte is 0xc0, it assumes a pointer and skips the two pointer bytes. Otherwise, it returns None for simplicity (as handling full name repetition is more complex).
        index += 2; // Skip type: Skips the Type field (should be 0x0001 for A record).
        index += 2; // Skip class: Skips the Class field (should be 0x0001 for IN).
        let ttl = u32::from_be_bytes(...); index += 4;: Reads the Time To Live (TTL) value as a 32-bit unsigned integer (in big-endian order) and advances the index. The TTL indicates how long the DNS record is valid.
        let rd_length = u16::from_be_bytes(...); index += 2;: Reads the Resource Data Length as a 16-bit unsigned integer (in big-endian order) and advances the index. This indicates the length of the actual data (the IP address in this case).
        if response.len() >= index + rd_length && rd_length == 4 { ... }: Checks if there are enough bytes remaining in the response for the data and if the data length is 4 (which is the size of an IPv4 address).
        let ip_bytes = &response[index..index + rd_length];: Extracts the 4 bytes representing the IP address.
        Some(format!("{}.{}.{}.{}", ...)): Formats the 4 bytes into a standard dotted-decimal IPv4 address string.
        else { None }: If the data length is not 4 or there aren't enough bytes, it returns None.

4. resolve Function:
Rust

fn resolve(domain: &str) -> Result<Option<String>, std::io::Error> {
    let server_address: SocketAddr = "8.8.8.8:53".parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let query_packet = create_query_packet(domain);

    socket.send_to(&query_packet, server_address)?;

    let mut buffer = [0; 512];
    let (amount, _) = socket.recv_from(&mut buffer)?;

    Ok(parse_response(&buffer[..amount]))
}

This function takes a domain name and performs the DNS resolution process.

    let server_address: SocketAddr = "8.8.8.8:53".parse().unwrap();: Defines the address of the DNS server to query. Here, it's set to Google's public DNS server at IP address 8.8.8.8 and port 53 (the standard DNS port). parse().unwrap() converts the string "8.8.8.8:53" into a SocketAddr and panics if the parsing fails.
    let socket = UdpSocket::bind("0.0.0.0:0")?;: Creates a new UDP socket and binds it to an arbitrary local address and port. The ? operator handles potential errors during socket creation.
    let query_packet = create_query_packet(domain);: Calls the create_query_packet function to generate the DNS query packet for the given domain.
    socket.send_to(&query_packet, server_address)?;: Sends the generated DNS query packet to the specified DNS server address. The ? operator handles potential errors during sending.
    let mut buffer = [0; 512];: Creates a buffer of 512 bytes to store the response from the DNS server. This size is usually sufficient for typical DNS responses.
    let (amount, _) = socket.recv_from(&mut buffer)?;: Receives data from the socket and stores it in the buffer.
        amount: The number of bytes received.
        _: The address of the sender (which we don't need in this simple case).
        The ? operator handles potential errors during receiving.
    Ok(parse_response(&buffer[..amount])): Calls the parse_response function to process the received data (up to the number of bytes received). The result of parse_response (an Option<String>) is wrapped in Ok() to indicate a successful operation. The function returns a Result which can either be Ok containing an Option<String> (Some(IP address) or None if no IP was found in the simplified parsing) or an Err containing an std::io::Error if an I/O error occurred.

5. main Function:
Rust

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

This is the entry point of the program. It demonstrates how to use the resolve function to look up the IP addresses of multiple domains concurrently.

    let domains_to_resolve = vec!["www.example.com", "www.google.com", "rust-lang.org"];: Creates a vector of domain names to resolve.
    let num_domains = domains_to_resolve.len();: Gets the number of domains to resolve.
    let (tx, rx) = mpsc::channel();: Creates a multi-producer, single-consumer channel.
        tx (transmitter): Used to send messages to the channel.
        rx (receiver): Used to receive messages from the channel.
    for domain in domains_to_resolve { ... }: Iterates through each domain in the domains_to_resolve vector.
        let tx_clone = tx.clone();: Clones the transmitter tx. This is necessary because each thread will need its own copy of the transmitter to send results back to the main thread.
        thread::spawn(move || { ... });: Spawns a new thread for each domain to be resolved. The move keyword moves ownership of captured variables (like domain and tx_clone) into the closure that will be executed by the new thread.
        println!("Resolving {}...", domain);: Prints a message indicating which domain is being resolved in the current thread.
        match resolve(domain) { ... }: Calls the resolve function to get the IP address for the current domain.
        tx_clone.send(Ok((domain, ip))).unwrap();: If the resolution is successful and an IP address is found, it sends an Ok result containing the domain and the IP address through the channel. unwrap() is used here, assuming the send operation will not fail.
        tx_clone.send(Err((domain, "No IP found".to_string()))).unwrap();: If the resolution is successful but the simplified parsing didn't find an IP, it sends an Err result with a "No IP found" message.
        tx_clone.send(Err((domain, format!("Error: {}", e)))).unwrap();: If the resolve function returns an error, it sends an Err result containing the error message.
    for _ in 0..num_domains { ... }: This loop in the main thread waits for each of the spawned threads to finish and send their results through the channel.
        match rx.recv().unwrap() { ... }: Receives a message from the channel. unwrap() is used here, assuming all threads will send a result.
        Ok((domain, ip)) => println!("{}: {}", domain, ip),: If the received message is an Ok result containing a domain and an IP address, it prints the domain and its resolved IP.
        Err((domain, error)) => println!("{}: {}", domain, error),: If the received message is an Err result containing a domain and an error message, it prints the domain and the error.






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
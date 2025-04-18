<link rel="stylesheet" href="star.css">

# Chapter 7 Basic DNS Server

Now we are going to build a basic DNS server.  

```bash 
cargo new dns_server
cd dns_server
```

Now let's build/run it.

```bash
cargo run 
```

You should see this output from the server:

```bash
DNS server listening on 0.0.0.0:1053
Received 56 bytes from 127.0.0.1:51895
Query for domain: www.example.com
Sent response to 127.0.0.1:51895
```

And this output on dig:

```bash
dig @127.0.0.1 -p 1053 www.example.com

; <<>> DiG 9.20.0-2ubuntu3.1-Ubuntu <<>> @127.0.0.1 -p 1053 www.example.com
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 2841
;; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0

;; QUESTION SECTION:
;www.example.com.		IN	A

;; ANSWER SECTION:
www.example.com.	300	IN	A	192.0.2.1

;; Query time: 0 msec
;; SERVER: 127.0.0.1#1053(127.0.0.1) (UDP)
;; WHEN: Sun Apr 13 07:57:04 MDT 2025
;; MSG SIZE  rcvd: 49

``` 

This Rust code implements a very basic Domain Name System (DNS) server that listens for UDP requests on port 1053 and responds with hardcoded IP addresses for a few specific domain names. Let's break down the code step by step:

## Imports:


```rust
use std::{
    net::{SocketAddr, UdpSocket},
    str,
};
```

`std::net::{SocketAddr, UdpSocket}`: This line imports necessary networking types from the standard library.

`SocketAddr`: Represents a network address (IP address and port number).

`UdpSocket`: Provides functionality for sending and receiving UDP packets.

`std::str`: This imports the str module, which provides functionality for working with UTF-8 encoded strings.

## DNS_RESPONSE_FLAGS Constant:


```rust
// Basic DNS header flags for a response
const DNS_RESPONSE_FLAGS: u16 = 0x8180; // Standard query response, recursion available
```

This constant defines the flags that will be set in the header of the DNS response packet.

```rust
qr_response |
(opcode_standard << 11) | // Shift opcode to its position
(aa_non_authoritative << 10) |
(tc_not_truncated << 9) |
(rd_desired << 8) | // Shift RD to its position
(ra_available << 7) | // Shift RA to its position
(reserved << 4) | // Shift reserved bits
rcode_no_error
```

Let's break down the meaning of these bits in the context of a DNS response:
1. The first bit (most significant) being 1 indicates that this is a response.
2. The next four bits (0000) represent the Opcode, which is 0 for a standard query.
3. The next bit (0) indicates that the server is not authoritative for the requested domain in this simplified example.
4. The T bit (Truncation) is 0, meaning the message is not truncated.
5. The R bit (Recursion Desired) in the query is usually copied to the response. Here, the second to last bit is 1, indicating that recursion is available on this server (though this simple implementation doesn't actually perform recursion).
6. The last three bits (000) represent the Rcode (Response code). 0 means no error.

## parse_query_domain Function:

```rust

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
```
This function takes a slice of bytes (query) representing a DNS query packet and attempts to extract the requested domain name.
let mut index = 12;: DNS queries have a fixed 12-byte header. This line initializes an index to skip past this header to reach the question section.
let mut domain_parts = Vec::new();: This creates an empty vector to store the individual parts of the domain name (e.g., "www", "example", "com").
The while loop iterates through the question section of the query:
let length = query[index] as usize;: The first byte of each domain name part indicates its length.
index += 1;: Move the index to the start of the domain name part.
if length == 0 { break; }: A zero length byte indicates the end of the domain name.
if index + length > query.len() { return None; }: Checks if the announced length goes beyond the bounds of the received query.
let part = str::from_utf8(&query[index..index + length]).ok()?;: This attempts to interpret the next length bytes as a UTF-8 string. The ok()? part handles potential errors during UTF-8 decoding by returning None if it fails.
domain_parts.push(part.to_string());: The decoded part of the domain name is added to the domain_parts vector.
index += length;: The index is advanced to the next length byte.
if index + 4 > query.len() { return None; }: After parsing the domain name, the query should contain the query type (2 bytes) and query class (2 bytes). This checks if there are enough bytes remaining.
let query_type = u16::from_be_bytes([query[index], query[index + 1]]);: Reads the next two bytes as a big-endian unsigned 16-bit integer, representing the query type. A value of 1 indicates an "A" record query (request for an IPv4 address).
let query_class = u16::from_be_bytes([query[index + 2], query[index + 3]]);: Reads the next two bytes as a big-endian unsigned 16-bit integer, representing the query class. A value of 1 indicates the "IN" (Internet) class.
if query_type == 1 && query_class == 1 { Some(domain_parts.join(".")) } else { None }: This checks if the query is for an "A" record in the "IN" class. If so, it joins the parts of the domain name with "." as a separator and returns it as an Option<String>. Otherwise, it returns None, indicating that this server only handles this specific type of query.

## create_response_packet Function:
```rust

fn create_response_packet(query: &[u8], domain: &str, ip_address: &str) -> Option<Vec<u8>> {
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
```
This function takes the original query (query), the resolved domain (domain), and the corresponding IP address (ip_address) as input and constructs a DNS response packet.
let mut response = Vec::new();: Creates an empty vector to store the bytes of the DNS response.
Header:
response.extend_from_slice(&query[0..2]);: Copies the 2-byte Transaction ID from the query to the response. This helps the client match the response to its original query.
response.extend_from_slice(&DNS_RESPONSE_FLAGS.to_be_bytes());: Adds the pre-defined response flags.
response.extend_from_slice(&query[4..6]);: Copies the 2-byte Questions Count from the query to the response.
response.extend_from_slice(&[0x00, 0x01]);: Sets the Answer RRs Count (Resource Records) to 1, as this response will contain one IP address.
response.extend_from_slice(&[0x00, 0x00]);: Sets the Authority RRs Count and Additional RRs Count to 0 in this simple response.
Copy Question Section:
This part copies the question section from the original query into the response. This is necessary for a valid DNS response.
It finds the end of the question section by iterating until it encounters the null terminator (0 byte) that marks the end of the domain name, and then skips the 4 bytes for the query type and class.
response.extend_from_slice(&query[12..question_end]);: Copies the bytes of the question section.
Answer Section:
This section contains the actual resolved IP address.
response.extend_from_slice(&[0xc0, 0x0c]);: This is a pointer to the domain name in the question section. 0xc0 indicates a pointer, and 0x0c (decimal 12) is the offset to the beginning of the question section in the DNS packet. This avoids repeating the full domain name in the answer.
response.extend_from_slice(&[0x00, 0x01]);: Sets the Type to 1, indicating an "A" record (IPv4 address).
response.extend_from_slice(&[0x00, 0x01]);: Sets the Class to 1, indicating the "IN" (Internet) class.
response.extend_from_slice(&300_u32.to_be_bytes());: Sets the Time-to-Live (TTL) to 300 seconds. This indicates how long the client should cache this DNS record. The value is converted to big-endian bytes.
response.extend_from_slice(&[0x00, 0x04]);: Sets the Resource Data Length (RDLength) to 4, as an IPv4 address is 4 bytes long.
The for loop iterates through the parts of the ip_address string (separated by "."). Each part is parsed as a u8 (unsigned 8-bit integer) and pushed into the response vector. If parsing fails, the function returns None.
Some(response): If the response packet is successfully created, it is returned as an Option<Vec<u8>>.

## main Function:
```rust

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
```

fn main() -> Result<(), std::io::Error>: This is the entry point of the program. It returns a Result to indicate success or an std::io::Error if an I/O error occurs.
let addr = "0.0.0.0:1053";: Defines the address and port on which the DNS server will listen. 0.0.0.0 means listen on all available network interfaces, and 1053 is the standard port for DNS servers. Using a port above 1024 (like 1053) typically doesn't require root privileges on Unix-like systems.
let socket = UdpSocket::bind(addr)?;: Creates a new UDP socket and binds it to the specified address and port. The ? operator handles potential errors during binding.
println!("DNS server listening on {}", addr);: Prints a message indicating that the server has started.
let mut buf = [0; 512];: Creates a mutable byte array of size 512 to store incoming UDP packets. This is a common maximum size for DNS packets.
loop { ... }: This starts an infinite loop, making the server continuously listen for incoming requests.
match socket.recv_from(&mut buf): This attempts to receive a UDP packet from the socket and stores the received data in the buf array. It returns a Result containing the number of bytes received and the source address (src) of the sender if successful, or an error if it fails.
Ok((amount, src)) => { ... }: If a packet is received successfully:
println!("Received {} bytes from {}", amount, src);: Prints information about the received packet.
let query = &buf[..amount];: Creates a slice of the buffer containing only the received data.
if let Some(domain) = parse_query_domain(query) { ... }: Calls the parse_query_domain function to extract the domain name from the query. If successful (returns Some(domain)):
println!("Query for domain: {}", domain);: Prints the requested domain.
let resolved_ip = match domain.as_str() { ... };: This is the hardcoded DNS resolution logic. It checks the requested domain against a few predefined domains and assigns a corresponding IP address. If the domain doesn't match any of these, it defaults to 127.0.0.1 (localhost).
if let Some(response_packet) = create_response_packet(query, &domain, resolved_ip) { ... }: Calls the create_response_packet function to create the DNS response packet. If successful:
socket.send_to(&response_packet, src)?;: Sends the generated response packet back to the source address from which the query was received. The ? handles potential errors during sending.
println!("Sent response to {}", src);: Prints a message indicating that the response has been sent.
else { eprintln!("Failed to create response packet for {}", domain); }: If create_response_packet returns None, an error message is printed.
else { eprintln!("Failed to parse DNS query"); }: If parse_query_domain returns None, indicating that the query couldn't be parsed or wasn't an "A" record query for the "IN" class, an error message is printed.
Err(e) => { eprintln!("Error receiving data: {}", e); }: If an error occurs while receiving data, an error message is printed.
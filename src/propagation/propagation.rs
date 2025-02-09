pub use crate::payload::payload;

use std::net::TcpStream;

pub fn main() {
    let open_ports: Vec<u16> = scan("127.0.0.1");
    println!("Open Ports: {:?}", open_ports);
    payload::main();
}

fn scan(ip: &str) -> Vec<u16> {
    let port_range: Vec<u16> = (1..65535).collect();

    let mut handles = vec![];

    let mut open_ports = Vec::new();

    for port in port_range {
        let ip = ip.to_string();
        let handle = std::thread::spawn(move || {
            let connection_with_timeout = TcpStream::connect_timeout(
                &format!("{}:{}", ip, port).parse().unwrap(),
                std::time::Duration::from_secs(1),
            );
            match connection_with_timeout {
                Ok(_) => {
                    Some(port)
                }
                Err(_) => {
                    None
                }
            }
        });
        handles.push(handle);

        if handles.len() == 1000 {
            for handle in handles {
                if let Some(port) = handle.join().unwrap() {
                    open_ports.push(port);
                }
            }
            handles = vec![];
        }
    }

    for handle in handles {
        if let Some(port) = handle.join().unwrap() {
            open_ports.push(port);
        }
    }
    
    return open_ports
}

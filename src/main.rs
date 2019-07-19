use crossbeam_channel::select;
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use std::env;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), ErrorKind> {
    match env::args().nth(1) {
        Some(ref arg) if arg == "-s" => server(),
        _ => client(),
    }
}

fn server() -> Result<(), ErrorKind> {
    // create the socket
    let destination: SocketAddr = "127.0.0.1:12346".parse().unwrap();
    let mut socket = Socket::bind("127.0.0.1:12345")?;
    let packet_sender = socket.get_packet_sender();
    let event_receiver = socket.get_event_receiver();
    // this will start the socket, which will start a poll mechanism to receive and send messages.
    let _thread = thread::spawn(move || socket.start_polling());
    let mut i = 0;
    loop {
        select! {
            recv(event_receiver) -> event => {
                match event {
                    Ok(socket_event) => match socket_event {
                        SocketEvent::Packet(packet) => {
                            let endpoint: SocketAddr = packet.addr();
                            // let received_data: &[u8] = packet.payload();
                            println!(
                                "Received: {:?} from {}",
                                String::from_utf8_lossy(packet.payload()),
                                endpoint
                            );
                        }
                        SocketEvent::Connect(connect_event) => {
                            println!("Connect: {}", connect_event);
                        }
                        SocketEvent::Timeout(timeout_event) => {
                            println!("Timeout: {}", timeout_event);
                        }
                    },
                    Err(e) => {
                        println!("Something went wrong when receiving, error: {:?}", e);
                    }
                }
            }
            default(Duration::from_secs(1)) => {
                let string = format!("Hello client! {}", i);
                println!("==>: {}", string);
                let bytes = string.into_bytes();
                let reliable_ordered = Packet::reliable_ordered(destination, bytes, Some(1));
                packet_sender.send(reliable_ordered).unwrap();
                i += 1;
            }
        }
    }
}

fn client() -> Result<(), ErrorKind> {
    // create the socket
    let destination: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let mut socket = Socket::bind("127.0.0.1:12346")?;
    let event_receiver = socket.get_event_receiver();
    let packet_sender = socket.get_packet_sender();
    // this will start the socket, which will start a poll mechanism to receive and send messages.
    let _thread = thread::spawn(move || {
        socket.start_polling();
        println!("Exiting polling");
    });
    let mut i = 0;
    loop {
        // wait until a socket event occurs
        select! {
            recv(event_receiver) -> event => {
                match event {
                    Ok(socket_event) => match socket_event {
                        SocketEvent::Packet(packet) => {
                            let endpoint: SocketAddr = packet.addr();
                            // let received_data: &[u8] = packet.payload();
                            println!(
                                "Received: {:?} from {}",
                                String::from_utf8_lossy(packet.payload()),
                                endpoint
                            );
                        }
                        SocketEvent::Connect(connect_event) => {
                            println!("Connect: {}", connect_event);
                        }
                        SocketEvent::Timeout(timeout_event) => {
                            println!("Timeout: {}", timeout_event);
                        }
                    },
                    Err(e) => {
                        println!("Something went wrong when receiving, error: {:?}", e);
                    }
                }
            }
            default(Duration::from_secs(1)) => {
                let string = format!("Hello server! {}", i);
                println!("==>: {}", string);
                let bytes = string.into_bytes();
                let reliable_ordered = Packet::reliable_ordered(destination, bytes, Some(1));
                packet_sender.send(reliable_ordered).unwrap();
                i += 1;
            }
        }
    }
}

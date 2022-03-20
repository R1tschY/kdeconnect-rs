use serde::{Deserialize, Serialize};
use serde_json::{from_reader, Value};
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::from_utf8;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::net::{TcpListener, UdpSocket};
use tokio::sync::mpsc::unbounded_channel;
use tokio_native_tls::native_tls::Identity;

const UDP_PORT: u16 = 1716;
const TCP_PORT: u16 = 1716;

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentityPacket {
    #[serde(rename = "deviceId")]
    device_id: String,
    #[serde(rename = "deviceName")]
    device_name: String,
    #[serde(rename = "deviceType")]
    device_type: String,
    #[serde(rename = "incomingCapabilities")]
    incoming_capabilities: Vec<String>,
    #[serde(rename = "outgoingCapabilities")]
    outgoing_capabilities: Vec<String>,
    #[serde(rename = "protocolVersion")]
    protocol_version: u32,
    #[serde(rename = "tcpPort")]
    tcp_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "body")]
pub enum NetworkPacket {
    #[serde(rename = "kdeconnect.identity")]
    Identity(IdentityPacket),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let new_device_channel = unbounded_channel();

    let tcp = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, TCP_PORT));
    Identity



    tokio::spawn(move || {
        let udp_socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, UDP_PORT)).await?;
        let mut datagramm_buf = [0u8; 1024 * 64];

        loop {
            let bytes = udp_socket.recv(&mut datagramm_buf).await?;

            let np: NetworkPacket = serde_json::from_reader(&datagramm_buf[0..bytes])?;
            println!("Got UDP datagramm: {:?}", np);
            if let NetworkPacket::Identity(identity) = np {
                println!(
                    "Received device broadcast: {} ({})",
                    identity.device_name, identity.device_id
                );
            }
        }
    });

    Ok(())
}

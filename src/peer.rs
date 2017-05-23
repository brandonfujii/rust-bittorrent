use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16
}

impl Peer {
    pub fn from_bytes(v: &[u8]) -> Self {
        let ip = Ipv4Addr::new(v[0], v[1], v[2], v[3]);
        let port = v[4] as u16 * 256 + v[5] as u16;
        Peer {
            ip: ip,
            port: port
        }
    }
}

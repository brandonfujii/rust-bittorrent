use std::net::Ipv4Addr;

#[derive(Debug, PartialEq, Clone)]
pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16
}

/// Represents a peer from which a client can request data
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

#[cfg(test)]
mod peer_tests {
    use super::Peer;
    use std::net::Ipv4Addr;

    #[test]
    fn peer_from_bytes_test() {
        let bytes = [127, 0, 0, 1, 31, 144];
        let p = Peer::from_bytes(&bytes);
        assert_eq!(p, Peer {
            ip: Ipv4Addr::new(127, 0, 0, 1),
            port: 8080
        })
    }
}

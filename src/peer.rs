use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, PartialEq, Clone)]
pub struct Peer {
    pub ip: IpAddr,
    pub port: u16,
    pub have: Option<Vec<bool>>,
    pub choked: Option<bool>,
    pub interested: Option<bool>,
}

/// Represents a peer from which a client can request data
impl Peer {
    pub fn from_bytes(v: &[u8]) -> Self {
        let ip = IpAddr::V4(Ipv4Addr::new(v[0], v[1], v[2], v[3]));
        let port = v[4] as u16 * 256 + v[5] as u16;
        Peer {
            ip: ip,
            port: port,
            have: None,
            choked: None,
            interested: None
        }
    }

    pub fn register(&mut self, pieces: usize) {
        match self.have {
            None => self.have = Some(vec![false; pieces]),
            _ => {}
        }

        match self.choked {
            None => self.choked = Some(false),
            _ => {}
        }

        match self.interested {
            None => self.interested = Some(false),
            _ => {}
        }
    }
}

#[cfg(test)]
mod peer_tests {
    use super::Peer;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn peer_from_bytes_test() {
        let bytes = [127, 0, 0, 1, 31, 144];
        let p = Peer::from_bytes(&bytes);
        assert_eq!(p, Peer {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            have: None,
            choked: None,
            interested: None,
        })
    }
}

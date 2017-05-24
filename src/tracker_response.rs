use bencode;
use bencode::{Bencode, FromBencode};
use util::*;
use peer::Peer;

#[derive(Debug)]
pub struct TrackerResponse {
    pub interval: u32,
    pub complete: u32,
    pub incomplete: u32,
    pub peers: Vec<Peer>,
}

impl FromBencode for TrackerResponse {
    type Err = Error;

    /// Attempts to construct a TrackerResponse object from a Bencode object. Returns a Result
    /// containing either
    ///     1) a TrackerResponse object, if a proper Bencode object was passed in
    ///     2) DictMatchErr otherwise
    fn from_bencode(bn: &bencode::Bencode) -> Result<TrackerResponse, Error> {
        match bn {
            &Bencode::Dict(ref m) => {
                let interval = decode_field_as_string(m, "interval")?;
                let complete = decode_field_as_string(m, "complete")?;
                let incomplete = decode_field_as_string(m, "incomplete")?;
                let mut peers_bytes = decode_field_as_bytes(m, "peers")?;

                if let Some(i) = peers_bytes.iter().position(|&b| b == 58) {
                    peers_bytes = peers_bytes.split_off(i).split_off(1);
                }

                let mut peers = vec![];
                for i in 0..(peers_bytes.len() / 6) {
                    let offset = i * 6;
                    let peer = Peer::from_bytes(&peers_bytes[offset..offset+6]);
                    peers.push(peer);
                }

                let tracker_response = TrackerResponse {
                    interval: interval.parse::<u32>().unwrap(),
                    complete: complete.parse::<u32>().unwrap(),
                    incomplete: incomplete.parse::<u32>().unwrap(),
                    peers: peers
                };

                Ok(tracker_response)
            }
            _ => Err(Error::DictMatchErr)
        }
    }
}


#[cfg(test)]
mod tracker_response_tests {
    use super::{TrackerResponse, FromBencode};
    use peer::Peer;
    use bencode;
    use util::*;

    #[test]
    fn flagfromserver_torrent_test() {
        let s = vec![100, 56, 58, 99, 111, 109, 112, 108, 101, 116, 101, 105, 49, 101, 49, 48, 58, 100, 111, 119, 110, 108, 111, 97, 100, 101, 100, 105, 51, 101, 49, 48, 58, 105, 110, 99, 111, 109, 112, 108, 101, 116, 101, 105, 50, 101, 56, 58, 105, 110, 116, 101, 114, 118, 97, 108, 105, 49, 56, 51, 56, 101, 49, 50, 58, 109, 105, 110, 32, 105, 110, 116, 101, 114, 118, 97, 108, 105, 57, 49, 57, 101, 53, 58, 112, 101, 101, 114, 115, 49, 56, 58, 98, 227, 182, 253, 31, 144, 165, 124, 144, 88, 31, 144, 96, 126, 104, 219, 238, 9, 101];

        let torrent: bencode::Bencode = bencode::from_vec(s).unwrap();
        let decoded: Result<TrackerResponse, Error> = FromBencode::from_bencode(&torrent);

        match decoded {
            Ok(response) => {
                assert_eq!(response.interval, 1838);
                assert_eq!(response.complete, 1);
                assert_eq!(response.incomplete, 2);
                assert_eq!(response.peers, vec![
                    Peer::from_bytes(&[98, 227, 182, 253, 31, 144]),
                    Peer::from_bytes(&[165, 124, 144, 88, 31, 144]),
                    Peer::from_bytes(&[96, 126, 104, 219, 238, 9]),
                ]);
            }
            _ => panic!("Decoded bencode incorrectly")
        }
    }
}

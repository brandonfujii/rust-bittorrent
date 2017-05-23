use bencode;
use bencode::{Bencode, FromBencode};
use util::*;

#[derive(Debug)]
pub struct TrackerResponse {
    // pub failure_reason: String,
    // pub warning_message: String,
    pub interval: u32,
    // pub min_interval: u32,
    // pub tracker_id: String,
    pub complete: u32,
    pub incomplete: u32,
    pub peers: Vec<u8>,
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
                let peers = decode_field_as_bytes(m, "peers")?;

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

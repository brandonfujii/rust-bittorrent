use bencode;
use bencode::{Bencode, FromBencode};
use hyper::{Client, client, header};
use metainfo::MetaInfo;
use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
use std::io::Read;
use tracker_response::TrackerResponse;
use util::Error;

#[allow(dead_code)]
pub enum TrackerError {
    RetrievePeerError
}

/// Encodes parameters into a url
///
/// # Example
/// ```
/// # use tracker;
/// let params: Vec<(&str, &str)> = vec![("peer_id", "l33t"), ("port", "8080")];
/// assert_eq!("peer_id=l33t&port=8080".to_string(), parameterize(params));
/// ```
pub fn parameterize(parameters: Vec<(&str, &str)>) -> String {
    let query_params: Vec<String> = parameters.iter()
            .map(|&kv| format!("{}={}", kv.0, kv.1))
            .collect();

    query_params.join("&")
}

#[cfg(test)]
mod parameterize_tests {
    use super::parameterize;

    #[test]
    fn sample_params_test() {
        let params: Vec<(&str, &str)> = vec![("peer_id", "l33t"), ("port", "8080")];
        assert_eq!("peer_id=l33t&port=8080".to_string(), parameterize(params));
    }
}

/// Sends a request to the tracker specified by the MetaInfo's announce attribute and returns a
/// list of `peer`s and `peer_id`s.
pub fn retrieve_peers(metainfo: &MetaInfo, peer_id: &str, port: &str) -> Result<client::response::Response, TrackerError> {
    let uploaded = 0.to_string();
    let downloaded = 0.to_string();
    let left = metainfo.info.length.to_string();
    let compact = 1.to_string();
    let percent_encoded_hash: String = percent_encode(&metainfo.info_hash, DEFAULT_ENCODE_SET).collect();

    let params: Vec<(&str, &str)> = vec![
        ("info_hash", percent_encoded_hash.as_ref()),
        ("peer_id", peer_id),
        ("port", port),
        ("uploaded", uploaded.as_ref()),
        ("downloaded", downloaded.as_ref()),
        ("left", left.as_ref()),
        ("compact", compact.as_ref()),
        ("event", "started")
    ];
    let query_params = parameterize(params);
    let query_url = format!("{}?{}", metainfo.announce, query_params);
    let client = Client::new();

    match client.get(&query_url).header(header::Connection::close()).send() {
        Ok(mut response) => {
            let mut s = Vec::new();
            response.read_to_end(&mut s).unwrap();

            let trackers: Bencode = bencode::from_vec(s).unwrap();
            let decoded: Result<TrackerResponse, Error> = FromBencode::from_bencode(&trackers);
            println!("{:?}", decoded.unwrap());

            Ok(response)
        }
        Err(_) => Err(TrackerError::RetrievePeerError)
    }
}

#[test]
fn retrieve_peers_test() {
    use metainfo;
    let m = metainfo::from_file(&String::from("data/flagfromserver.torrent")).unwrap();
    let res = retrieve_peers(&m, "tovatovatovatovatova", "8080");
}

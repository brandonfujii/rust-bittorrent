use hyper::{Client, client, header};
use metainfo::MetaInfo;

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
/// assert_eq!("peer_id=l33t&port=8080".to_string(), url_encode(params));
/// ```
pub fn url_encode(parameters: Vec<(&str, &str)>) -> String {
    let query_params: Vec<String> = parameters.iter()
            .map(|&kv| format!("{}={}", kv.0, kv.1))
            .collect();

    query_params.join("&")
}

#[cfg(test)]
mod url_encoding_tests {
    use super::url_encode;

    #[test]
    fn sample_params_test() {
        let params: Vec<(&str, &str)> = vec![("peer_id", "l33t"), ("port", "8080")];
        assert_eq!("peer_id=l33t&port=8080".to_string(), url_encode(params));
    }
}

/// Sends a request to the tracker specified by the MetaInfo's announce attribute and returns a
/// list of `peer`s and `peer_id`s.
#[allow(dead_code)]
fn retrieve_peers(metainfo: &MetaInfo, peer_id: &str, port: &str) -> Result<client::response::Response, TrackerError> {
    let params: Vec<(&str, &str)> = vec![("peer_id", peer_id), ("port", port)];
    let query_params = url_encode(params);
    let query_url = format!("{}?{}", metainfo.announce, query_params);
    let client = Client::new();

    match client.get(&query_url).header(header::Connection::close()).send() {
        Ok(response) => {
            // TODO: parse response body
            // return peers in response
            Ok(response)
        }
        Err(_) => Err(TrackerError::RetrievePeerError)
    }
}

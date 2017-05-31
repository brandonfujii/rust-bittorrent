## Bittorrent

A Bittorrent client written in Rust.

To run:
```
cargo run <path/to/bittorrent-file>
```

### What the project does so far:

- [x] The program takes in a torrent file, decodes it, and reads it into metainfo
- [x] It has a client that creates a url based on the details in the metainfo and a peer id
	- [x] The client can send a request to the url and receive a response, which contains a list of peers
- [x] The tracker parses the response to retrieve a list of peers
- [x] Contains a download file struct comprised of pieces that it expects to download from its list of peers
- [x] Makes a network request to a list of peers
- [x] Initiate handshaking protocol with client and peers
- [x] Coordinate a download of a file among peers
- [x] Implement block storage by designating each requested block of data to the correct index in our Torrent struct and saving it
- [x] Writing the target file to a disk


### What's left to do:
- [ ] Handle multiple concurrent requests to peers

### Reflection
From this experience, we learned a lot about the bittorrent protocol and networking in general, especially because neither of us have any extensive computer networking knowledge. Something we had difficulty with was testing network requests and connections because we could run the same code and receive different results. We also underestimated the amount of time and effort it would take to implement block storage and message passing, which took longer than anticipated.

What we've accomplished, in respect to our initial proposal, was most of the client-side portion of the bittorrent protocol, including decoding the torrent file, retrieving a list of peers, initiating the handshaking protocol with each peer, retrieving requested blocks, storing blocks, assembling pieces, and writing those pieces to a file on disk.

Some functionality that we haven't yet implemented is handling multiple concurrent connections with peers. Also, we haven't implemented a listener to standby and accept incoming peer connections.

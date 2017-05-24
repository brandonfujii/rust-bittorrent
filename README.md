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

### What's left to do:

- [ ] Initiate handshaking protocol with client and peers (In progress)
- [ ] Coordinate a download of a file among peers
- [ ] Implement block storage by designating each requested block of data to the correct index in our Torrent struct and saving it
- [ ] Writing the target file to a disk


### What's changed from the initial proposal
	
- We're going to try to implement responding to requests to send blocks of a file
- However, seeding is still a reach goal

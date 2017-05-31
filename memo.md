# Rust BitTorrent Client Revised Memo
 
## Changes Since Original Proposal
We've made some changes to reflect the progress made on the project since our original proposal, including: 
- We've figured out a good way of storing blocks and writing pieces to disk

Most of the original proposal still stands because we're simply implementing the Bittorrent protocol, which hasn't changed at all.

## Introduction & Motivation
The BitTorrent protocol provides a method for distributing files among a peer-to-peer (P2P) network, making it faster and less bandwidth-intensive to share large files with many clients than regular HTTP [1]. Our project will be to implement a BitTorrent client which is capable of parsing a torrent file, connecting to peers, and downloading the file associated with a torrent.
We chose this project because we think that implementing a torrent client would be a good way to explore concurrency in Rust. Since the main advantage of torrents is that they allow clients to download from multiple peers, a useful BitTorrent client would be able to handle multiple concurrent bytestreams of data and use multi-threaded sockets to maintain many connections at once.
For the scope of this course, we are choosing to focus on the downloading portion of the BitTorrent client and will treat support for file-seeding as a nice-to-have requirement. 
 
## Use cases & Examples 
A typical use case should go as follows: 
1. The user passes in a .torrent file to the client.
2. The client parses the torrent.
3. The client makes a request to the tracker specified in the torrent.
4. The client parses the response from the tracker, which contains a collection of peer IDs and corresponding IP addresses of your peers.
5. The client connects to peers and initiates downloads of various pieces of the requested file from peers.
 
## Must-have Requirements
- Must implement file-sharing using a BitTorrent protocol in which a user can join a swarm of other hosts to download files.
- The program must be able to read in a torrent file and parse it in order find the tracker and peers from which it can download the file. To do this, it must be able to parse bencoded data, BitTorrent’s primary method of encoding data as strings [3].
- The BitTorrent protocol should adhere to the specifications outlined in BitTorrent Protocol Specification [3], including the different stages of message passing:
- Bitfield/Have, in which the client gets information about which pieces of the file the peer has.
- Interested/Unchoke, in which the client initiates a connection with a peer using an Interested flag and the peer responds with an Unchoke flag to indicate that it will supply the requested piece of the file.
- Request/Piece, in which the client asks for some piece of the file and the peer supplies it to the client.
- A user should be able to download and assemble the contents of a torrent through small data requests to different IP connections to multiple different hosts or machines
 
## Nice-to-have Requirements
- GUI for displaying progress on downloading torrents
- Implement the “rarest first” algorithm for selecting pieces of a file to download from peers [1] 
	- The “rarest first” algorithm is the primary policy for deciding which piece is downloaded from a given seed. It means when a client selects the next piece of the file to download, it selects the piece which the fewest of the peers have [1]
- Sharing files over network: support seeding to other peers
- The “End Game,” where once the file has been almost completely downloaded the client sends requests to many peers at once requesting the last few pieces, then sends “Cancel” messages once it has received those pieces from another peer.
 
## Difficulties
Some difficulties we anticipate are:
- Keeping track of multiple, concurrent connections with peers
- Testing networked parts of the client

## Questions We Still Have
Some questions we still have are: 
- What are the best practices to test network protocols in Rust?
 
## References
[1] Johnsen, Jahn Arne, Lars Erik Karlsen, and Sebjørn Sæther Birkeland. Peer-to-peer Networking with BitTorrent. UCLA Engineering. Department of Telematics, NTNU, May 2005. Web. 9 May 2017. http://web.cs.ucla.edu/classes/cs217/05BitTorrent.pdf 
 
[2] "BitTorrent Protocol -- BTP/1.0." BitTorrent Protocol Version 1.0 Revision: 1.33 : BitTorrent Protocol -- BTP/1.0. N.p., n.d. Web. 09 May 2017. http://jonas.nitro.dk/bittorrent/bittorrent-rfc.html 
 
[3] "BitTorrent Protocol -- BTP/1.0." BitTorrent Protocol Version 1.0 $Revision: 1.33 $: BitTorrent Protocol -- BTP/1.0. N.p., n.d. Web. 09 May 2017. http://www.bittorrent.org/beps/bep_0003.html  

use::std::fmt;
use util::{bytes_to_u32, u32_to_bytes};

#[derive(PartialEq)]
pub enum Message {
    KeepAlive,
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have(u32),
    Bitfield(Vec<u8>),
    Request(u32, u32, u32),
    Piece(u32, u32, Vec<u8>),
    Cancel,
    Port,
}

/// Constructs messages to be passed between peers. Messages are structured as arrays of bytes:
/// where bytes:
///     1-4 represent the length of the message as a u32
///     5 holds the id of the message
///     6-* contains the payload
impl Message {
    pub fn new(id: &u8, body: &[u8]) -> Message {
        match *id {
            0 => Message::Choke,
            1 => Message::Unchoke,
            2 => Message::Interested,
            3 => Message::NotInterested,
            4 => Message::Have(bytes_to_u32(body)),
            5 => Message::Bitfield(body.to_owned()),
            6 => {
                let index = bytes_to_u32(&body[0..4]);
                let offset = bytes_to_u32(&body[4..8]);
                let length = bytes_to_u32(&body[8..12]);
                Message::Request(index, offset, length)
            },
            7 => {
                let index = bytes_to_u32(&body[0..4]);
                let offset = bytes_to_u32(&body[4..8]);
                let data = body[8..].to_owned();
                Message::Piece(index, offset, data)
            },
            8 => Message::Cancel,
            9 => Message::Port,
            _ => panic!("Bad message id: {}", id)
        }
    }

    pub fn serialize(self) -> Vec<u8> {
        let mut payload = vec![];
        match self {
            Message::KeepAlive => {},
            Message::Choke => payload.push(0),
            Message::Unchoke => payload.push(1),
            Message::Interested => payload.push(2),
            Message::NotInterested => payload.push(3),
            Message::Have(index) => {
                payload.push(4);
                payload.extend(u32_to_bytes(index).into_iter());
            },
            Message::Bitfield(bytes) => {
                payload.push(5);
                payload.extend(bytes);
            },
            Message::Request(index, offset, amount) => {
                payload.push(6);
                payload.extend(u32_to_bytes(index).into_iter());
                payload.extend(u32_to_bytes(offset).into_iter());
                payload.extend(u32_to_bytes(amount).into_iter());
            },
            Message::Piece(index, offset, data) => {
                payload.push(7);
                payload.extend(u32_to_bytes(index).into_iter());
                payload.extend(u32_to_bytes(offset).into_iter());
                payload.extend(data);
            },
            Message::Cancel => payload.push(8),
            Message::Port => payload.push(9),
        };

        let mut size = u32_to_bytes(payload.len() as u32);
        size.extend(payload);
        size
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
             Message::KeepAlive => write!(f, "KeepAlive"),
             Message::Choke => write!(f, "Choke"),
             Message::Unchoke => write!(f, "Unchoke"),
             Message::Interested => write!(f, "Interested"),
             Message::NotInterested => write!(f, "NotInterested"),
             Message::Have(ref index) => write!(f, "Have({})", index),
             Message::Bitfield(ref bytes) => write!(f, "Bitfield({:?})", bytes),
             Message::Request(ref index, ref offset, ref length) => write!(f, "Request({}, {}, {})", index, offset, length),
             Message::Piece(ref index, ref offset, ref data) => write!(f, "Piece({}, {}, size={})", index, offset, data.len()),
             Message::Cancel => write!(f, "Cancel"),
             Message::Port => write!(f, "Port"),
        }
    }
}

#[cfg(test)]
mod block_tests {
    use super::Message;

    #[test]
    fn make_and_serialize_message_test() {
        let mut msg = Message::new(&0, &[]);
        assert_eq!(msg, Message::Choke);
        assert_eq!(msg.serialize(), vec![
            0, 0, 0, 1,
            0,
        ]);

        msg = Message::new(&1, &[]);
        assert_eq!(msg, Message::Unchoke);
        assert_eq!(msg.serialize(), vec![
            0, 0, 0, 1,
            1,
        ]);

        msg = Message::new(&2, &[]);
        assert_eq!(msg, Message::Interested);
        assert_eq!(msg.serialize(), vec![
            0, 0, 0, 1,
            2,
        ]);

        msg = Message::new(&3, &[]);
        assert_eq!(msg, Message::NotInterested);
        assert_eq!(msg.serialize(), vec![
            0, 0, 0, 1,
            3,
        ]);

        msg = Message::new(&4, &[0, 0, 1, 1]);
        assert_eq!(msg, Message::Have(257));
        assert_eq!(msg.serialize(), vec![
            0, 0, 0, 5,
            4,
            0, 0, 1, 1,
        ]);

        msg = Message::new(&5, &[0, 0, 1, 1]);
        assert_eq!(msg, Message::Bitfield(vec![0, 0, 1, 1]));
        assert_eq!(msg.serialize(), vec![
            0, 0, 0, 5,
            5,
            0, 0, 1, 1,
        ]);

        msg = Message::new(&6, &[0, 0, 1, 1, 0, 0, 1, 2, 0, 0, 1, 3]);
        assert_eq!(msg, Message::Request(257, 258, 259));
        assert_eq!(msg.serialize(), vec![
            0, 0, 0, 13,
            6,
            0, 0, 1, 1,
            0, 0, 1, 2,
            0, 0, 1, 3,
        ]);

        msg = Message::new(&7, &[0, 0, 1, 1, 0, 0, 1, 2, 0, 0, 1, 3, 4, 5]);
        assert_eq!(msg, Message::Piece(257, 258, vec![0, 0, 1, 3, 4, 5]));
        assert_eq!(msg.serialize(), vec![
            0, 0, 0, 15,
            7,
            0, 0, 1, 1,
            0, 0, 1, 2,
            0, 0, 1, 3, 4, 5,
        ]);

        msg = Message::new(&8, &[]);
        assert_eq!(msg, Message::Cancel);
        assert_eq!(msg.serialize(), vec![
            0, 0, 0, 1,
            8,
        ]);

        msg = Message::new(&9, &[]);
        assert_eq!(msg, Message::Port);
        assert_eq!(msg.serialize(), vec![
            0, 0, 0, 1,
            9,
        ]);

        msg = Message::KeepAlive;
        assert_eq!(msg.serialize(), vec![0, 0, 0, 0]);
    }
}

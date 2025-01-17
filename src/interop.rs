pub enum Message {
    PoHHash(String),     // A hash generated by PoH
    PoSFinality(String), // A finalized PoS block
}

pub struct Interop {
    pub poh_to_pos: std::sync::mpsc::Sender<Message>,
    pub pos_to_poh: std::sync::mpsc::Receiver<Message>,
}

impl Interop {
    pub fn new() -> (Self, std::sync::mpsc::Sender<Message>, std::sync::mpsc::Receiver<Message>) {
        let (poh_tx, pos_rx) = std::sync::mpsc::channel();
        let (pos_tx, poh_rx) = std::sync::mpsc::channel();

        (
            Self {
                poh_to_pos: poh_tx,
                pos_to_poh: poh_rx,
            },
            pos_tx,
            pos_rx,
        )
    }

    pub fn send_poh_to_pos(&self, hash: String) {
        self.poh_to_pos.send(Message::PoHHash(hash)).unwrap();
    }

    pub fn send_pos_to_poh(&self, hash: String) {
        self.poh_to_pos.send(Message::PoSFinality(hash)).unwrap();
    }
}


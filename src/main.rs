mod poh;      // Import the `poh` module
mod pos;      // Import the `pos` module
mod interop;  // Import the `interop` module

use interop::{Interop, Message};  // Use the Interop struct and Message enum
use poh::ProofOfHistory;          // Use the ProofOfHistory struct
use pos::ProofOfStake;            // Use the ProofOfStake struct

use std::thread;

fn main() {
    let (interop, pos_tx, pos_rx) = Interop::new();

    // Thread for PoH simulation
    let poh_thread = thread::spawn(move || {
        let mut poh = ProofOfHistory::new();
        let hash = poh.generate_entry("".to_string());
        println!("PoH Generated: {}", hash);
        interop.send_poh_to_pos(hash.clone());

        // Simulate waiting for a finality message from PoS
        if let Ok(Message::PoSFinality(received_hash)) = interop.pos_to_poh.recv() {
            println!("PoH Received Finality from PoS: {}", received_hash);
        }
    });

    // Thread for PoS simulation
    let pos_thread = thread::spawn(move || {
        let mut pos = ProofOfStake::new();

        // Simulate receiving a PoH message
        if let Ok(Message::PoHHash(received_hash)) = pos_rx.recv() {
            println!("PoS Received Hash from PoH: {}", received_hash);
            pos.finalize_block(received_hash.clone());
            println!("PoS Finalized: {}", received_hash);

            // Send finality back to PoH
            pos_tx.send(Message::PoSFinality(received_hash)).unwrap();
        }
    });

    poh_thread.join().unwrap();
    pos_thread.join().unwrap();
}


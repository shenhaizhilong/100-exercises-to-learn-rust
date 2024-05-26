use std::error::Error;
use std::sync::mpsc::{Receiver, RecvError, Sender};
use crate::data::TicketDraft;
use crate::store::TicketStore;

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server the thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        let v = receiver.recv();
        match v {
            Ok(cmd) => {
                match cmd {
                    Command::Insert(t) => {
                        println!("received: {:?}", t);
                        store.add_ticket(t);
                    }
                }
            }
            Err(e) => {
                println!("recv error {:?}", e.to_string());
            }
        }
    }
}

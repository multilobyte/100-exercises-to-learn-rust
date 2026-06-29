use std::sync::mpsc::{Receiver, Sender};
use crate::store::TicketStore;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert{draft: data::TicketDraft, response_sender: Sender<store::TicketId>},
    Get{id: store::TicketId, response_sender: Sender<Option<data::Ticket>>},
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    while let Ok(msg) = receiver.recv() {
        match msg {
            Command::Insert{draft, response_sender} => {
                let ticket_id = store.add_ticket(draft);
                response_sender.send(ticket_id).expect("send response");
            },
            Command::Get{id, response_sender} => {
                let ticket = store.get(id).cloned();
                response_sender.send(ticket).expect("send response");
            }
        }
    }
}

mod instruction;

use crate::database::ExecutionContext;
use crate::query::{Request, Response};
use instruction::Handler as InstructionHandler;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub struct Handler {
    producer: Sender<Response>,
    consumer: Receiver<Request>,
}

impl Handler {
    /// Create a new handler on the given query link.
    pub fn new(query_link: (Sender<Response>, Receiver<Request>)) -> Handler {
        Handler {
            producer: query_link.0,
            consumer: query_link.1,
        }
    }

    /// Handle the query link.
    pub fn handle(&mut self, context: &mut ExecutionContext) {
        self.receive_requests(context);
    }

    /// Pull all received queries and handle them.
    fn receive_requests(&mut self, context: &mut ExecutionContext) {
        let mut requests = Vec::new();

        loop {
            match self.consumer.try_recv() {
                Ok(request) => requests.push(request),
                Err(error) => match error {
                    TryRecvError::Empty => break,
                    TryRecvError::Disconnected => panic!("Query channel disconnected."),
                },
            };
        };

        for request in &requests {
            let result = InstructionHandler::handle(request.get_instruction(), context);
            if let Err(_) = self.producer.send(Response::new(request.clone(), result)) {
                panic!("Query channel disconnected.");
            };
        };
    }
}

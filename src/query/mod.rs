pub mod instruction;

use instruction::Instruction;

pub type Client = u128;

#[derive(Clone, Debug)]
pub struct Request {
    client: Client,
    instruction: Instruction,
}

impl Request {
    /// Create a new request with the given instruction, originating from the given client.
    pub fn new(client: Client, instruction: Instruction) -> Request {
        Request {
            client: client,
            instruction: instruction,
        }
    }

    /// Get the client at the origin of this request.
    pub fn get_client(&self) -> Client {
        self.client
    }

    /// Get the instruction associated with this request.
    pub fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }
}

#[derive(Debug)]
pub struct Response {
    request: Request,
    result: Result<(), ()>,
}

impl Response {
    pub fn new(request: Request, result: Result<(), ()>) -> Response {
        Response {
            request: request,
            result: result,
        }
    }

    pub fn get_result(&self) -> &Result<(), ()> {
        &self.result
    }

    pub fn get_request(&self) -> &Request {
        &self.request
    }
}

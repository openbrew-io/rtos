use super::Handle;
use super::PhantomData;

pub struct Context<R: Copy + Send,S: Copy + Send> {
    handle: Handle<R>,
    inbox: [R; 10],
    _phantom: PhantomData<S>,
}

pub enum SendError {
    NotFound,
    InboxFull,
}

pub enum ReceiveError {
    InboxEmpty,
}

#[allow(unused_variables)]
impl<R: Copy + Send,S: Copy + Send> Context<R,S> {

    /// Returns Handle for the current actor.
    pub fn handle(&self) -> &Handle<R> {
        &self.handle
    }

    /// Causes the actor to yield execution.
    pub fn release(&mut self) {}

    /// Removes a message from the actors's Inbox and returns it. In case
    /// the inbox is empty, the actor is blocked until a new message is
    /// received.
    pub fn receive(&mut self) -> R {
        self.inbox[0]
    }

    /// Removes a message from the actors's Inbox and returns it. In case
    /// the inbox is empty, the actor is blocked until a new message is
    /// received.
    pub fn try_receive(&mut self) -> Result<R, ReceiveError> {
        Err(ReceiveError::InboxEmpty)
    }

    /// Send a message to actor with the given Handle. The method yield
    /// control for the calling actor in the following cases
    /// * The inbox for receiver is Full.
    pub fn send(&mut self, handle: Handle<S>, msg: S) -> Result<(), SendError> {
        Err(SendError::NotFound)
    }

    /// Send a message to actor with given Handle. The method differs
    /// from send since it doesn't block in the event that the destination
    /// actor's inbox is full.
    pub fn try_send(&mut self, handle: Handle<S>, msg: S) -> Result<(), SendError> {
        Err(SendError::NotFound)
    }
}

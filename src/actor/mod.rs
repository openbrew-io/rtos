use core::marker::PhantomData;

pub mod system;
pub mod context;

#[allow(unused_imports)]
use system::ActorSystem;
use context::Context;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Handle<T> {
    id: u32,
    phantom: PhantomData<T>,
}

impl<T> Handle<T> {
    pub(crate) fn new(id: u32, phantom: PhantomData<T>) -> Self {
        Self {
            id,
            phantom,
        }
    }
}

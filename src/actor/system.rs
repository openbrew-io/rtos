use super::Context;
use super::PhantomData;
use super::Handle;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SpawnOpts {
    priority: i8,
    inbox_size: usize,
}

pub struct ActorSystem {
    default_spawn_opts: SpawnOpts,
}

#[allow(unused_variables)]
impl ActorSystem {
    pub fn default() -> Self {
        Self {
            default_spawn_opts: SpawnOpts {
                priority: 5,
                inbox_size: 10,
            },
        }
    }

    /// Spawns a new actor using the given function and returns the Handle.
    /// The actor receives the state as the initial argument.
    pub fn spawn<F,State,R,S>(&mut self, f: F, state: State) -> Handle<R>
    where
        R: Copy + Send,
        S: Copy + Send,
        F: FnMut(Context<R,S>,State) -> ()
    {
        let opts = self.default_spawn_opts.clone();
        self.spawn_with_opts(f, state, opts)
    }

    /// Spawns a new actor with the passed opts and initial state.
    pub fn spawn_with_opts<F,State,R,S>(&mut self, f: F, state: State, opts: SpawnOpts) -> Handle<R>
    where
        R: Copy + Send,
        S: Copy + Send,
        F: FnMut(Context<R,S>,State) -> ()
    {
        Handle::new(0, PhantomData::<R>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::*;

    #[test]
    fn test_spawn() {
        fn actor(mut _c: Context<i32, i32>, _state: i32) {

        }

        let mut actor_system = ActorSystem::default();
        let handle = actor_system.spawn(actor, 10);

        assert_eq!(handle, Handle::new(0, PhantomData::<i32>))
    }
}

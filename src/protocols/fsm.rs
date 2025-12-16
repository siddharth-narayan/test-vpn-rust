// Manages a Finite State Machine builder so that protcols can be easily defined
// It should only transition on input received from clients

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::network::openssl::SslWrite;
use crate::protocols::util::TunWrite;

// Where Action takes place functionally _during_ the transition, and if it fails, the transition doesn't occur
pub type TransitionTable<State, Input, Data> = HashMap<(State, Input), (State, Box<dyn Fn(Arc<Mutex<SslWrite>>, Data) -> bool + 'static + Send + Sync>)>;

pub struct FSM<State, Input, AssociatedData>
{
    client_write: Arc<Mutex<SslWrite>>, // Weird, but for quick writes to the client
    tun_write: TunWrite,
    current: State,
    transitions: TransitionTable<State, Input, AssociatedData>,
}

impl<State, Input, AssociatedData> FSM<State, Input, AssociatedData>
where
    State: Eq + Hash + Copy,
    Input: Eq + Hash + Copy,
{
    pub fn new(initial: State, transitions: TransitionTable<State, Input, AssociatedData>, ssl_write: Arc<Mutex<SslWrite>>) -> Self {
        Self {
            client_write: ssl_write,
            current: initial,
            transitions,
        }
    }

    pub fn transition(mut self, input: Input, data: AssociatedData) -> Option<State> {
        let (next_state, action) = self.transitions.get(&(self.current, input))?;

        let success = action(self.client_write, data);

        if success {
            self.current = *next_state;
            return Some(*next_state)
        }
        
        None
    }

    pub fn state(&self) -> State {
        self.current
    }
}

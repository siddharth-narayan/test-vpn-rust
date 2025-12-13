// Manages a Finite State Machine builder so that protcols can be easily defined

use std::collections::HashMap;
use std::hash::Hash;

// Where Action takes place functionally _during_ the transition, and if it fails, the transition doesn't occur
pub type TransitionTable<State, Input, Action> = HashMap<(State, Input), (State, Action)>;

pub trait FSMAction<State, Input> {
    fn execute<AssociatedData>(&self, state: State, input: Input, data: AssociatedData) -> bool;
}

pub struct FSM<State, Input, Action>
where
    State: Eq + Hash + Copy,
    Input: Eq + Hash + Copy,
    Action: FSMAction<State, Input>,
{
    current: State,
    transitions: TransitionTable<State, Input, Action>,
}

impl<State, Input, Action> FSM<State, Input, Action>
where
    State: Eq + Hash + Copy,
    Input: Eq + Hash + Copy,
    Action: FSMAction<State, Input>
{
    pub fn new(initial: State, transitions: TransitionTable<State, Input, Action>) -> Self {
        Self {
            current: initial,
            transitions,
        }
    }

    pub fn transition<AssociatedData>(&mut self, input: Input, data: AssociatedData) -> Option<State> {
        let (next_state, action) = self.transitions.get(&(self.current, input))?;

        let success = action.execute(self.current, input,  data);

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

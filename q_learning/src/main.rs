#![allow(dead_code)]

use std::rc::Rc;

fn main() {
}

// position
struct Coordinate2d {
    x: i8,
    y: i8,
}

struct QValue {
    value: [f32; 36], // this time 6x6
    reinforcement_signal: Vec<[i8; 4]>
}

// decide next action with ε-Greedy
struct EpsironGreedy {
    epsiron: f32,
    q_value: Rc<QValue>
}

impl DecideAction for EpsironGreedy {
    fn decide_action(&self, now_position: Coordinate2d) -> Coordinate2d{
        unimplemented!()
    }
}

// decide next action using ε-Greedy, softmax, and so-on.
trait DecideAction {
    fn decide_action(&self, now_position: Coordinate2d) -> Coordinate2d;
}

// decide state depending on subject
trait DecideState {
    fn decide_state(&self, now_position: Coordinate2d, next_position: Coordinate2d) -> Result<Coordinate2d, usize>;
}

fn q_search_goal(
    start: Coordinate2d,
    goal: Coordinate2d,
    action_determiner: Rc<DecideAction>,
    state_determiner: Rc<DecideState>
) {
    unimplemented!();
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(1 + 2, 3)
    }
}
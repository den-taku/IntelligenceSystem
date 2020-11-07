#![allow(dead_code)]
#![allow(unused_imports)]

use crate::maze;

use std::rc::Rc;
use std::cell::RefCell;
use maze::get_maze1;
use num_traits::ToPrimitive;


// position
#[derive(Debug, Copy, Clone)]
struct Coordinate2d {
    x: i8,
    y: i8,
}

impl Coordinate2d {
    fn new<T>(cdn1: T, cdn2: T) -> Coordinate2d 
    where
        T: ToPrimitive
    {
        Coordinate2d{ x: T::to_i8(&cdn1).unwrap(), y: T::to_i8(&cdn2).unwrap() }
    }
}

#[derive(Copy, Clone)]
struct QValue {
    value: [f32; 36], // this time 6x6
    reinforcement_signals: [[f32; 4]; 36] // A = {N, E, S, W}
}

impl QValue {
    fn new(signals: &[[f32; 4]; 36]) -> Self {
        QValue {value: [0.0; 36], reinforcement_signals: *signals}
    }
}

// decide next action with ε-Greedy
struct EpsironGreedy {
    epsiron: f32,
    q_value: Rc<RefCell<QValue>>
}

impl EpsironGreedy {
    fn new(epsiron: f32, q_value: Rc<RefCell<QValue>>) -> Self {
        EpsironGreedy{epsiron, q_value}
    }
    fn update_epsiron(&mut self, new_value: f32) {
            if !(0.0 < new_value && new_value < 1.0) {
                panic!("EpsironGreedy::update_epsiron needs epsion: 0 < ε < 1");
            }
            self.epsiron = new_value;
    }
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

#[derive(Debug, Clone, Copy)]
struct SearchGoal {}

impl DecideState for SearchGoal {
    fn decide_state(&self, now_position: Coordinate2d, next_position: Coordinate2d) -> Result<Coordinate2d, usize> {
        unimplemented!()
    }
}

// decide state depending on subject
trait DecideState {
    fn decide_state(&self, now_position: Coordinate2d, next_position: Coordinate2d) -> Result<Coordinate2d, usize>;
}

fn q_search_goal(
    start: Coordinate2d,
    goal: Coordinate2d,
    action_determiner: Rc<dyn DecideAction>,
    state_determiner: Rc<dyn DecideState>
) {
    unimplemented!();
}

fn serch_goal_1() {
    //initialize Q value
    let reinforcement_signals = maze::get_maze1();
    let q = Rc::new(RefCell::new(QValue::new(&reinforcement_signals)));
    let start = Coordinate2d::new(0, 0);
    let goal = Coordinate2d::new(4, 3);
    let epsiron = 0.1;
    let action_determiner: Rc<dyn DecideAction> = Rc::new(EpsironGreedy::new(epsiron, q.clone()));
    let state_determiner: Rc<dyn DecideState> = Rc::new(SearchGoal{});
    
    
    unimplemented!();
}

#[cfg(test)]
mod test{
    use crate::search_goal::EpsironGreedy;
    use crate::search_goal::QValue;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    #[should_panic(expected = "EpsironGreedy::update_epsiron needs epsion: 0 < ε < 1")]
    fn test_epsiron_greedy_update_epsiron() {
        let mut dummy_epsiron_greedy = EpsironGreedy::new(0.1, Rc::new(RefCell::new(QValue::new(&[[0.0; 4]; 36]))));
        dummy_epsiron_greedy.update_epsiron(8.0);
    }
}
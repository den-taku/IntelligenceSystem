mod search_goal;
mod maze;

use std::rc::Rc;
use std::cell::RefCell;

use maze::get_maze1;

use search_goal::*;

fn main() {
    let reinforcement_signals = maze::get_maze1();
    let q = Rc::new(RefCell::new(QValue::new(0.0, &reinforcement_signals, 0.9)));
    let start = Coordinate2d::new(0, 0);
    let goal1 = Coordinate2d::new(4, 3);
    let mut epsiron = 0.75;
    let action_determiner = Rc::new(RefCell::new(EpsironGreedy::new(epsiron)));
    let next_state_determiner = Rc::new(SearchGoal::new());
    let mut learning_rate1 = LearningRate::new(1.0, 700.0);
    let mut times = 1isize;
    q.borrow_mut().q_search_goal(start, goal1, action_determiner.clone(), next_state_determiner.clone(), &mut learning_rate1, times);

}
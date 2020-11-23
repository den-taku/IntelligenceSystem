mod maze;
mod search_goal;

use std::cell::RefCell;
use std::rc::Rc;

use maze::get_maze1;

use search_goal::*;

fn main() {
    let reinforcement_signals1 = get_maze1();
    let q1 = Rc::new(RefCell::new(QValue::new(0.0, &reinforcement_signals1, 0.9)));
    let start1 = Coordinate2d::new(0, 0);
    let goal11 = Coordinate2d::new(4, 3);
    let mut epsiron1 = 0.75;
    let action_determiner1 = Rc::new(RefCell::new(EpsironGreedy::new(epsiron1)));
    let next_state_determiner1 = Rc::new(SearchGoal::new());
    let mut learning_rate11 = LearningRate::new(1.0, 700.0);
    let mut times1 = 1usize;
    {
        for _ in 0..100 {
            q1.borrow_mut().q_search_goal(
                start1,
                goal11,
                action_determiner1.clone(),
                next_state_determiner1.clone(),
                &mut learning_rate11,
                &mut times1,
            );
            epsiron1 -= 0.0065;
            action_determiner1.borrow_mut().update_epsiron(epsiron1);
        }
    }
    {
        q1.borrow_mut().q_search_goal_print(
            start1,
            goal11,
            action_determiner1.clone(),
            next_state_determiner1.clone(),
            &mut learning_rate11,
            &mut times1,
        );
    }
    println!("{}", q1.borrow());

    let reinforcement_signals2 = maze::get_maze2();
    let q2 = Rc::new(RefCell::new(QValue::new(0.0, &reinforcement_signals2, 0.9)));
    let start2 = Coordinate2d::new(0, 0);
    let goal12 = Coordinate2d::new(4, 3);
    let goal22 = Coordinate2d::new(1, 4);
    let mut epsiron2 = 0.75;
    let action_determiner2 = Rc::new(RefCell::new(EpsironGreedy::new(epsiron2)));
    let next_state_determiner2 = Rc::new(SearchGoal::new());
    let mut learning_rate12 = LearningRate::new(1.0, 700.0);
    let mut times2 = 1usize;
    {
        for _ in 0..100 {
            q2.borrow_mut().q_search_goal2(
                start2,
                goal12,
                goal22,
                action_determiner2.clone(),
                next_state_determiner2.clone(),
                &mut learning_rate12,
                &mut times2,
            );
            epsiron2 -= 0.0065;
            action_determiner2.borrow_mut().update_epsiron(epsiron2);
        }
    }
    {
        q2.borrow_mut().q_search_goal_print2(
            start2,
            goal12,
            goal22,
            action_determiner2.clone(),
            next_state_determiner2.clone(),
            &mut learning_rate12,
            &mut times2,
        );
    }
    println!("{}", q2.borrow());
}

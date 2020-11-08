#![allow(dead_code)]
#![allow(unused_imports)]

use crate::maze;

use std::rc::Rc;
use std::cell::RefCell;
use maze::get_maze1;
use num_traits::ToPrimitive;
use rand::Rng;
use std::fmt;
use std::fmt::{Display, Formatter};


// position
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coordinate2d {
    x: usize,
    y: usize,
}

impl Coordinate2d {
    pub fn new<T>(cdn1: T, cdn2: T) -> Coordinate2d 
    where
        T: ToPrimitive
    {
        Coordinate2d{ x: T::to_usize(&cdn1).unwrap(), y: T::to_usize(&cdn2).unwrap() }
    }
}

#[derive(Copy, Clone)]
pub struct QValue {
    value: [[f64; 4]; 36], // this time 6x6
    reinforcement_signals: [[f64; 4]; 36], // A = {N, E, S, W}
    discount_rate: f64
}

impl QValue {
    pub fn new(init: f64, signals: &[[f64; 4]; 36], discount_rate: f64) -> Self {
        QValue {value: [[init; 4]; 36], reinforcement_signals: *signals, discount_rate}
    }
    fn q_learning(&mut self, leaning_rate: LearningRate, now_position: &Coordinate2d, next_action: usize, next_position: Coordinate2d, reinforcment_signal: f64) {
        let past_value = self.value[now_position.x + now_position.y * 6][
            next_action
        ];
        let alpha = leaning_rate.value();
        // println!("in q_learning {} {}", next_position.x, next_position.y);
        let next_value = self.value[(next_position.x + next_position.y * 6) as usize].iter().fold(0.0, |max, e| { if max >= *e { max } else { *e } });
        // println!("next q_value is {}.", (1.0 - alpha) * past_value + alpha * (reinforcment_signal + self.discount_rate * next_value));
        // println!("(1.0 - {}) * {} + {} * ({} + {} * {})",alpha, past_value, alpha, reinforcment_signal, self.discount_rate, next_value);
        self.value[(now_position.x + now_position.y * 6) as usize][
            next_action
        ] = (1.0 - alpha) * past_value + alpha * (reinforcment_signal + self.discount_rate * next_value);
    }
    fn get_reiforcement_signal(&self, from_position: &Coordinate2d, to_position: &usize) -> f64 {
        self.reinforcement_signals[from_position.x + from_position.y * 6][
            *to_position
        ]
    }
    pub fn q_search_goal(
        &mut self,
        start: Coordinate2d,
        goal: Coordinate2d,
        action_determiner: Rc<RefCell<dyn DecideAction>>,
        next_state_determiner: Rc<dyn DecideNextState>,
        learning_rate: &mut LearningRate,
        times: &mut usize // initialize as 1usize
    ) {
        let mut now_position = start;
        self.q_serch_goal_inner(&mut now_position, goal, action_determiner.clone(), next_state_determiner.clone(), learning_rate, times);
    }
    
    fn q_serch_goal_inner(
        &mut self,
        now_position: &mut Coordinate2d,
        goal: Coordinate2d,
        action_determiner: Rc<RefCell<dyn DecideAction>>,
        next_state_determiner: Rc<dyn DecideNextState>,
        learning_rate: &mut LearningRate,
        mut times: &mut usize
    ) {
        loop{
            println!("times: {}", times);
            if *now_position == goal { 
                println!("Goal!!");
                return; 
            }
            // println!("loop");
            let next_action = action_determiner.borrow().decide_action(&now_position, self.clone());
            // println!("get next action");
            let next_position = next_state_determiner.decide_next_state(&now_position, next_action, self.clone());
            // println!("get next position");
            let reinfocement_signal = self.get_reiforcement_signal(&now_position, &next_action);
            // println!("get reignforcement signal");
            self.q_learning(*learning_rate, now_position, next_action, next_position, reinfocement_signal);
            *now_position = next_position;
            *times += 1;
            learning_rate.update(*times)
        }
    }
    pub fn q_search_goal_print(
        &mut self,
        start: Coordinate2d,
        goal: Coordinate2d,
        action_determiner: Rc<RefCell<dyn DecideAction>>,
        next_state_determiner: Rc<dyn DecideNextState>,
        learning_rate: &mut LearningRate,
        times: &mut usize // initialize as 1usize
    ) {
        println!("ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ");
        println!("Show Route");
        let mut now_position = start;
        self.q_serch_goal_inner_print(&mut now_position, goal, action_determiner.clone(), next_state_determiner.clone(), learning_rate, times);
    }
    
    fn q_serch_goal_inner_print(
        &mut self,
        now_position: &mut Coordinate2d,
        goal: Coordinate2d,
        action_determiner: Rc<RefCell<dyn DecideAction>>,
        next_state_determiner: Rc<dyn DecideNextState>,
        learning_rate: &mut LearningRate,
        mut times: &mut usize
    ) {
        loop{
            println!("times: {}", *times);
            if *now_position == goal { 
                println!("Goal!!");
                return; 
            }
            // println!("loop");
            let next_action = action_determiner.borrow().decide_action(&now_position, self.clone());
            // println!("get next action");
            let next_position = next_state_determiner.decide_next_state(&now_position, next_action, self.clone());
            println!("{:?}", next_position); // Show
            // println!("get next position");
            let reinfocement_signal = self.get_reiforcement_signal(&now_position, &next_action);
            // println!("get reignforcement signal");
            self.q_learning(*learning_rate, now_position, next_action, next_position, reinfocement_signal);
            *now_position = next_position;
            *times += 1;
            learning_rate.update(*times)
        }
    }
    pub fn q_search_goal2(
        &mut self,
        start: Coordinate2d,
        goal1: Coordinate2d,
        goal2: Coordinate2d,
        action_determiner: Rc<RefCell<dyn DecideAction>>,
        next_state_determiner: Rc<dyn DecideNextState>,
        learning_rate: &mut LearningRate,
        times: &mut usize // initialize as 1usize
    ) {
        let mut now_position = start;
        self.q_serch_goal_inner2(&mut now_position, goal1, goal2, action_determiner.clone(), next_state_determiner.clone(), learning_rate, times);
    }
    
    fn q_serch_goal_inner2(
        &mut self,
        now_position: &mut Coordinate2d,
        goal1: Coordinate2d,
        goal2: Coordinate2d,
        action_determiner: Rc<RefCell<dyn DecideAction>>,
        next_state_determiner: Rc<dyn DecideNextState>,
        learning_rate: &mut LearningRate,
        mut times: &mut usize
    ) {
        loop{
            println!("times: {}", times);
            if *now_position == goal1 || *now_position == goal2 { 
                println!("Goal!!");
                return; 
            }
            // println!("loop");
            let next_action = action_determiner.borrow().decide_action(&now_position, self.clone());
            // println!("get next action");
            let next_position = next_state_determiner.decide_next_state(&now_position, next_action, self.clone());
            // println!("get next position");
            let reinfocement_signal = self.get_reiforcement_signal(&now_position, &next_action);
            // println!("get reignforcement signal");
            self.q_learning(*learning_rate, now_position, next_action, next_position, reinfocement_signal);
            *now_position = next_position;
            *times += 1;
            learning_rate.update(*times)
        }
    }
    pub fn q_search_goal_print2(
        &mut self,
        start: Coordinate2d,
        goal1: Coordinate2d,
        goal2: Coordinate2d,
        action_determiner: Rc<RefCell<dyn DecideAction>>,
        next_state_determiner: Rc<dyn DecideNextState>,
        learning_rate: &mut LearningRate,
        times: &mut usize // initialize as 1usize
    ) {
        println!("ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ");
        println!("Show Route");
        let mut now_position = start;
        self.q_serch_goal_inner_print2(&mut now_position, goal1, goal2, action_determiner.clone(), next_state_determiner.clone(), learning_rate, times);
    }
    
    fn q_serch_goal_inner_print2(
        &mut self,
        now_position: &mut Coordinate2d,
        goal1: Coordinate2d,
        goal2: Coordinate2d,
        action_determiner: Rc<RefCell<dyn DecideAction>>,
        next_state_determiner: Rc<dyn DecideNextState>,
        learning_rate: &mut LearningRate,
        mut times: &mut usize
    ) {
        loop{
            println!("times: {}", *times);
            if *now_position == goal1 || *now_position == goal2 { 
                println!("Goal!!");
                return; 
            }
            // println!("loop");
            let next_action = action_determiner.borrow().decide_action(&now_position, self.clone());
            // println!("get next action");
            let next_position = next_state_determiner.decide_next_state(&now_position, next_action, self.clone());
            println!("{:?}", next_position); // Show
            // println!("get next position");
            let reinfocement_signal = self.get_reiforcement_signal(&now_position, &next_action);
            // println!("get reignforcement signal");
            self.q_learning(*learning_rate, now_position, next_action, next_position, reinfocement_signal);
            *now_position = next_position;
            *times += 1;
            learning_rate.update(*times)
        }
    }
}

impl Display for QValue {
    fn fmt(&self, dest: &mut Formatter) -> fmt::Result {
        let mut buffer = "".to_string();
        for x in 0..6 {
            for y in 0..6 {
                buffer = format!("{}({}, {}): N: {}, E: {}, S: {}, W: {}\n", 
                    buffer, x, y, 
                    self.value[x + y * 6][0],
                    self.value[x + y * 6][1],
                    self.value[x + y * 6][2],
                    self.value[x + y * 6][3])
            }
        }
        write!(dest, "{}", buffer)
    }
}

// 0: learinig rate, 1: τ s.t. learing rate = 2 - exp(t/τ)
#[derive(Debug, Copy, Clone)]
pub struct LearningRate(f64, f64);

impl LearningRate {
    pub fn new(init: f64, time_constant: f64) -> Self {
        LearningRate(init, time_constant)
    }
    fn update(&mut self, times: usize) {
        let e = 2.71828182846f64;
        let new_rate = 2.0 - e.powf(times as f64 / self.1);
        self.0 = if new_rate > 0.0 { new_rate } else { 0.001 }; // learning rate defined
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

// decide next action with ε-Greedy
pub struct EpsironGreedy {
    epsiron: f64,
    // q_value: Rc<RefCell<QValue>>
}

impl EpsironGreedy {
    pub fn new(epsiron: f64/* , q_value: Rc<RefCell<QValue>>*/) -> Self {
        if !(0.0 < epsiron && epsiron < 1.0) {
            panic!("EpsironGreedy::new needs epsiron: 0 < ε < 1");
        }
        EpsironGreedy{epsiron, /* q_value*/ }
    }
    pub fn update_epsiron(&mut self, new_value: f64) {
            if !(0.0 < new_value && new_value < 1.0) {
                panic!("EpsironGreedy::update_epsiron needs epsion: 0 < ε < 1");
            }
            self.epsiron = new_value;
    }
}

impl DecideAction for EpsironGreedy {
    fn decide_action(&self, now_position: &Coordinate2d, q_value: QValue) -> usize {
        
        let mut rng = rand::thread_rng();
        let probability = rng.gen::<f64>();
        // println!("get values");
        let mut values = [
            (q_value.value[now_position.x + now_position.y * 6][0].clone(), 0usize),
            (q_value.value[now_position.x + now_position.y * 6][1].clone(), 1usize),
            (q_value.value[now_position.x + now_position.y * 6][2].clone(), 2usize),
            (q_value.value[now_position.x + now_position.y * 6][3].clone(), 3usize),
        ];
        // println!("end get value");
        values.sort_by(|b, a| a.0.partial_cmp(&b.0).unwrap());

        let first_range = 1.0 - self.epsiron;
        let second_range = 1.0 - self.epsiron * 2.0 / 3.0;
        let third_range = 1.0 - self.epsiron * 1.0 / 3.0;
        let _fourth_range = 1.0;

        for i in 0..4 {
            // println!("{:?}", values[i]);
        }
        // println!("probability is {}.", probability);

        // println!("end method");

        if 0.0 <= probability && probability <= first_range {
            // println!("choise is {}", values[0].1);
            values[0].1
            // f(values[0].1, now_position)
        } else if first_range < probability && probability <= second_range {
            // println!("choise is {}", values[1].1);
            values[1].1
            // f(values[1].1, now_position)
        } else if second_range < probability && third_range <= third_range {
            // println!("choise is {}", values[2].1);
            values[2].1
            // f(values[2].1, now_position)
        } else {
            // println!("choise is {}", values[3].1);
            values[3].1
            // f(values[3].1, now_position)
        }
    }
    fn update_parameter(&mut self, new_value: f64) {
        if !(0.0 < new_value && new_value < 1.0) {
            panic!("EpsironGreedy::update_parameter needs epsion: 0 < ε < 1");
        }
        self.epsiron = new_value
    }
}

// decide next action using ε-Greedy, softmax, and so-on.
pub trait DecideAction {
    fn decide_action(&self, now_position: &Coordinate2d, q_value: QValue) -> usize;
    fn update_parameter(&mut self, new_value: f64);
}

#[derive(Clone)]
pub struct SearchGoal {
    // q_value: Rc<RefCell<QValue>>
}

impl SearchGoal {
    pub fn new(/*q_value: Rc<RefCell<QValue>>*/) -> Self {
        SearchGoal{}
    }
}

impl DecideNextState for SearchGoal {
    fn decide_next_state(&self, now_position: &Coordinate2d, next_action: usize, q_value: QValue) -> Coordinate2d {
        // println!("{:?}", now_position);
        // println!("{:?}", next_action);
        let reinforcement_signal = /*q_value.value[now_position.x + now_position.y * 6][
            // if now_position.x == next_action.x { (next_action.y - now_position.y + 1) as usize }
            // else { (2 + now_position.x - next_action.x) as usize }
            next_action
        ];*/q_value.get_reiforcement_signal(now_position, &next_action);
        // println!("re-sig = {}", reinforcement_signal);
        if reinforcement_signal == -0.1 {
            *now_position
        } else {
            // println!("{:?} {}", now_position, next_action);
            // println!("{}", reinforcement_signal);
            if next_action % 2 == 0 {
                // println!("{} - 1 + {}", now_position.y, next_action);
                Coordinate2d::new(now_position.x, now_position.y + next_action - 1)
            } else {
                Coordinate2d::new(now_position.x + 2 - next_action, now_position.y)
            }
            // next_action
        }
    }
}

// decide state depending on subject
pub trait DecideNextState {
    fn decide_next_state(&self, now_position: &Coordinate2d, next_action: usize, q_value: QValue) -> Coordinate2d;
}

// q learning for 1 episode


// episode : 1-index
// fn serch_goal_1(episode: usize) {
//     //initialize Q value
//     let reinforcement_signals = maze::get_maze1();
//     let q = Rc::new(RefCell::new(QValue::new(&reinforcement_signals)));
//     let start = Coordinate2d::new(0, 0);
//     let goal = Coordinate2d::new(4, 3);
//     let epsiron = 0.75;
//     let action_determiner = Rc::new(RefCell::new(EpsironGreedy::new(epsiron, q.clone())));
//     let state_determiner = Rc::new(SearchGoal{});
//     let mut alpha = 0.5;

//     // q_search_goal(start, goal, action_determiner.clone(), state_determiner, &mut alpha);

//     for _ in 1..episode {
//         //
//         let new_epsiron = 0.1;
//         action_determiner.borrow_mut().update_parameter(new_epsiron);
//     }
    
    
//     unimplemented!();
// }

#[cfg(test)]
mod test{
    use crate::search_goal::EpsironGreedy;
    use crate::search_goal::QValue;
    use crate::search_goal::DecideAction;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    #[should_panic(expected = "EpsironGreedy::update_epsiron needs epsion: 0 < ε < 1")]
    fn test_epsiron_greedy_update_epsiron() {
        let mut dummy_epsiron_greedy = EpsironGreedy::new(0.1, /*Rc::new(RefCell::new(QValue::new(0.0, &[[0.0; 4]; 36], 0.9)))*/);
        dummy_epsiron_greedy.update_epsiron(8.0);
    }

    #[test]
    #[should_panic(expected = "EpsironGreedy::update_parameter needs epsion: 0 < ε < 1")]
    fn test_epsiron_greedy_update_parameter_inner() {
        let mut dummy_epsiron_greedy = EpsironGreedy::new(0.1, /*Rc::new(RefCell::new(QValue::new(0.0, &[[0.0; 4]; 36], 0.9)))*/);
        dummy_epsiron_greedy.update_parameter(8.0);
    }
    
    #[test]
    #[should_panic(expected = "EpsironGreedy::new needs epsiron: 0 < ε < 1")]
    fn test_epsiron_greedy_new() {
        let _ = EpsironGreedy::new(1.0, /*Rc::new(RefCell::new(QValue::new(0.0, &[[0.0; 4]; 36], 0.9)))*/);
    }
}
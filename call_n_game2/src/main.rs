#![allow(dead_code)]

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
// use std::time::Instant;

fn judge_winner_call_n_game2(n: usize, m: usize) -> String {
    if the_first_caller_is_winner2(n, m) {
        "The first caller".to_string()
    } else {
        "The second caller".to_string()
    }
}

fn the_first_caller_is_winner2(n: usize, m: usize) -> bool {
    let first_note = Rc::new(RefCell::new(HashMap::new()));
    let second_note = Rc::new(RefCell::new(HashMap::new()));
    let count = Rc::new(Cell::new(0usize));
    let value = rec_value2(n, m, 0, false, first_note, second_note, count.clone());
    println!("count : {}", count.get());
    value
}

// true : the first caller, false : the seconed caller
fn rec_value2(
    n: usize,
    m: usize,
    parant: usize,
    parant_turn_is_first: bool,
    first_note: Rc<RefCell<HashMap<usize, bool>>>,
    second_note: Rc<RefCell<HashMap<usize, bool>>>,
    count: Rc<Cell<usize>>,
) -> bool {
    let mut children = Vec::new();

    if parant_turn_is_first {
        for i in 0..m {
            {
                if parant + i + 1 == n {
                    children.push(parant_turn_is_first);
                    let new = (*count).get() + 1;
                    count.set(new);
                    break;
                }
            }
            {
                let new = count.get() + 1;
                count.set(new);
            }
            let mut flag = true;
            if let Some(value) = first_note.borrow().get(&(parant + i + 1)) {
                children.push(*value);
                flag = false;
            }
            if flag {
                let value = rec_value2(
                    n,
                    m,
                    parant + i + 1,
                    !parant_turn_is_first,
                    first_note.clone(),
                    second_note.clone(),
                    count.clone(),
                );
                first_note.borrow_mut().insert(parant + i + 1, value);
                children.push(value);
            }
        }
    } else {
        for i in 0..m {
            if parant + i + 1 == n {
                children.push(parant_turn_is_first);
                let new = (*count).get() + 1;
                count.set(new);
                break;
            }
            {
                let new = count.get() + 1;
                count.set(new);
            }
            let mut flag = true;
            if let Some(value) = second_note.borrow().get(&(parant + i + 1)) {
                children.push(*value);
                flag = false;
            }
            if flag {
                let value = rec_value2(
                    n,
                    m,
                    parant + i + 1,
                    !parant_turn_is_first,
                    first_note.clone(),
                    second_note.clone(),
                    count.clone(),
                );
                second_note.borrow_mut().insert(parant + i + 1, value);
                children.push(value);
            }
        }
    }

    if parant == 0 {
        second_judge_result(children)
    } else if parant_turn_is_first {
        first_judge_result(children)
    } else {
        second_judge_result(children)
    }
}

fn first_judge_result(v: Vec<bool>) -> bool {
    !v.iter().any(|e| !*e)
}

fn second_judge_result(v: Vec<bool>) -> bool {
    v.iter().any(|e| *e)
}
fn main() {
    for i in 6..22 {
        println!(
            "When N is {:2.} and M is 4, the Winner is {}",
            i,
            judge_winner_call_n_game2(i, 4)
        );
    }
    // let mut sum_sec = vec![0.0; 16];
    // for _ in 0..100 {
    //     for i in 6..22 {
    //         let start = Instant::now();
    //         let _ = judge_winner_call_n_game2(i, 4);
    //         let end = start.elapsed();
    //         sum_sec[i - 6] += end.as_secs_f32();
    //     }
    // }
    // for i in 0..16 {
    //     sum_sec[i] /= 100.0;
    //     println!(
    //         "When N: {:2.} and M: 4, Average Run Time is {}s",
    //         i + 6,
    //         sum_sec[i]
    //     )
    // }
}

#[cfg(test)]
mod tests_call_n_game {
    use super::the_first_caller_is_winner2;
    #[test]
    fn test_the_first_caller_is_winner2() {
        // check N =  6,..,21 and M = 4
        for i in 6..22 {
            println!("{}", i);
            assert_eq!(the_first_caller_is_winner2(i, 4), !(i % 5 == 1));
        }
    }
}

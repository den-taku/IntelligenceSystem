#![allow(dead_code)]

// use std::time::Instant;

fn judge_winner_call_n_game(n: usize, m: usize) -> String {
    if the_first_caller_is_winner(n, m) {
        "The first caller".to_string()
    } else {
        "The second caller".to_string()
    }
}

fn the_first_caller_is_winner(n: usize, m: usize) -> bool {
    rec_value(n, m, 0, false)
}

// true : the first caller, false : the seconed caller
fn rec_value(n: usize, m: usize, parant: usize, parant_turn_is_first: bool) -> bool {
    let mut children = Vec::new();
    for i in 0..m {
        if parant + i + 1 == n {
            children.push(parant_turn_is_first);
            break;
        }
        let buf = rec_value(n, m, parant + i + 1, !parant_turn_is_first);
        children.push(buf);
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
            judge_winner_call_n_game(i, 4)
        );
    }

    // let mut sum_sec = vec![0.0; 16];
    // for _ in 0..100{
    //     for i in 6..22 {
    //         let start = Instant::now();
    //         let _ = judge_winner_call_n_game(i, 4);
    //         let end = start.elapsed();
    //         sum_sec[i-6] += end.as_secs_f32();
    //     }
    // }
    // for i in 0..16 {
    //     sum_sec[i] /= 100.0;
    //     println!("When N: {:2.} and M: 4, Average Run Time is {}s", i + 6, sum_sec[i]);
    // }
    // println!("average run time cost is {}s", sum_sec / 100.);
}

#[cfg(test)]
mod tests_call_n_game {
    use super::the_first_caller_is_winner;
    #[test]
    fn test_the_first_caller_is_winner() {
        // check N =  6,..,21 and M = 4
        for i in 6..22 {
            println!("{}", i);
            assert_eq!(the_first_caller_is_winner(i, 4), !(i % 5 == 1));
        }
    }
}

// fn minimize<T: Ord + Copy>(v: Vec<T>) -> Option<T> {
//     if v.len() == 0 {
//         None
//     } else {
//         Some(*v.iter().min().unwrap())
//     }
// }

// fn maximize<T: Ord + Copy>(v: Vec<T>) -> Option<T> {
//     if v.len() == 0 {
//         None
//     } else {
//         Some(*v.iter().max().unwrap())
//     }
// }

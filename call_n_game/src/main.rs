#![allow(dead_code)]

fn judge_winner_call_n_game(n: usize, m: usize) -> String {
    if the_first_caller_is_winner(n, m) {
        "The first caller".to_string()
    } else {
        "The second caller".to_string()
    }
}

fn the_first_caller_is_winner(n: usize, m: usize) -> bool {
    rec_value(n, m, 0, true)
}

// true : the first caller, false : the seconed caller
fn rec_value(n: usize, m: usize, parant: usize, turn_is_first: bool) -> bool {
    let mut children = Vec::new();
    for i in 0..m {
        // println!("{}, {}", parant, i);
        if parant + i + 1 == n {
            children.push(!turn_is_first);
            break;
        }
        children.push(rec_value(n, m, parant + i + 1, !turn_is_first))
    }
    if turn_is_first {
        first_judge(children)
    } else {
        second_judge(children)
    }
}

fn first_judge(v: Vec<bool>) -> bool {
    v.iter().any(|e| !*e)
}

fn second_judge(v: Vec<bool>) -> bool {
    v.iter().any(|e| *e)
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

fn main() {
    the_first_caller_is_winner(6, 4);
}

#[cfg(test)]
mod tests_call_n_game {
    use super::the_first_caller_is_winner;
    #[test]
    fn test_the_first_caller_is_winner() {
        // check N =  6,..,21 and M = 4
        for i in 6..22 {
            println!("{}", i);
            assert_eq!(the_first_caller_is_winner(i, 4), !i % 5 == 1);
        }
        // assert_eq!(the_first_caller_is_winner(6, 4), false)
    }
}

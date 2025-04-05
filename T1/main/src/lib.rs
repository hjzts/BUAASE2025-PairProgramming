use wasm_bindgen::prelude::wasm_bindgen;
use std::collections::HashSet;

#[wasm_bindgen]
pub fn greedy_snake_move(snake: &[i32], fruit: &[i32]) -> i32{
    let H = 8;
    let L = 1;

    let head_x = snake[0];
    let head_y = snake[1];
    let second_x = snake[2];
    let second_y = snake[3];
    let fruit_x = snake[0];
    let fruit_y = snake[1];

    let diff_x = fruit_x - head_x;
    let diff_y = fruit_y - head_y;

    let mut options: HashSet<i32> = HashSet::from([0,1,2,3]); // Up Left Down Right
    if (head_x + 1 == second_x && head_y == second_y) || head_x == H {
        options.remove(&3);
    }
    if (head_x - 1 == second_x && head_y == second_y) || head_x == L {
        options.remove(&1);
    }
    if (head_x == second_x && head_y + 1 == second_y) || head_y == H {
        options.remove(&0);
    }
    if (head_x == second_x && head_y - 1 == second_y) || head_y == L {
        options.remove(&2);
    }

    let mut direction = -1;
    if diff_x > 0 && options.contains(&3) {
        direction = 3;
    } else if diff_x < 0 && options.contains(&1) {
        direction = 1;
    } else if diff_y > 0 && options.contains(&0) {
        direction = 0;
    } else if diff_y < 0 && options.contains(&2) {
        direction = 2;
    }

    if direction == -1 {
        // let mut rng = thread_rng();
        // let Some(direction) = options.iter().choose(&mut rng) else { panic!("man! out"); };
        direction = options
    }

    direction
}
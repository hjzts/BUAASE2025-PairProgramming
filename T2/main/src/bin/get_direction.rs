use main::greedy_snake_move_barriers;
fn main() {
    let snake = [1, 4, 1, 3, 1, 2, 1, 1];
    let fruit = [5, 5];
    let barriers = [
        2, 7, 2, 6, 3, 7, 3, 6, 4, 6, 5, 6, 6, 6, 7, 6, 4, 5, 4, 4, 4, 3, 5, 4,
    ];
    let direction = greedy_snake_move_barriers(&snake, &fruit, &barriers);
    println!("Direction: {}", direction);
}
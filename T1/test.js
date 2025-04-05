import assert from "assert";

// Choose proper "import" depending on your PL.
// import { greedy_snake_move } from "./t1-as/build/release.js";
import { greedy_snake_move } from "./main/pkg/main.js";
// [Write your own "import" for other PLs.]

function greedy_snake_fn_checker (snake, food) {
    let now_snake = [
        snake[0], snake[1], snake[2], snake[3], snake[4], snake[5], snake[6], snake[7]
    ];
    let turn = 1;
    while (true) {
        let result = greedy_snake_move(now_snake, food);
        let new_snake = [
            now_snake[0] + (result == 3) - (result == 1),
            now_snake[1] + (result == 0) - (result == 2),
            now_snake[0],
            now_snake[1],
            now_snake[2],
            now_snake[3],
            now_snake[4],
            now_snake[5],
        ];
        if (new_snake[0] < 1 || new_snake[0] > 8 || new_snake[1] < 1 || new_snake[1] > 8) {
            return -1;
        }
        if (new_snake[0] == new_snake[4] && new_snake[1] == new_snake[5]) {
            return -2;
        }
        if (new_snake[0] == food[0] && new_snake[1] == food[1]) {
            console.log("Total turn: " + turn);
            return turn;
        }
        now_snake = [
            new_snake[0], new_snake[1], new_snake[2], new_snake[3], new_snake[4], new_snake[5], new_snake[6], new_snake[7]
        ];
        if (turn > 200) {
            return -3;
        }
        turn += 1;
    }
}

// Test cases
assert.strictEqual(greedy_snake_fn_checker([4,4,4,5,4,6,4,7], [1,1], greedy_snake_move) >= 0, true);
assert.strictEqual(greedy_snake_fn_checker([1,1,1,2,1,3,1,4], [1,5], greedy_snake_move) >= 0, true);
assert.strictEqual(greedy_snake_fn_checker([1,1,1,2,2,2,2,1], [1,5], greedy_snake_move) >= 0, true);
assert.strictEqual(greedy_snake_fn_checker([1,1,2,1,2,2,1,2], [1,5], greedy_snake_move) >= 0, true);

console.log("🎉 You have passed all the tests provided.");
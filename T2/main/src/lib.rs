use wasm_bindgen::prelude::wasm_bindgen;
use std::collections::{VecDeque, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct SnakeState {
    head: Point,
    body: VecDeque<Point>,
    tail: Point,
    fruit: Point,
    barriers: HashSet<Point>,
}

impl SnakeState {
    fn new(snake: &[i32], fruit: &[i32], barriers: &[i32]) -> Self {
        let mut body = VecDeque::new();
        let head = Point { x: snake[0], y: snake[1] };
        for i in (2..8).step_by(2) {
            body.push_back(Point { x: snake[i], y: snake[i + 1] });
        }
        let tail = body.back().copied().unwrap_or(head);

        let mut barrier_set = HashSet::new();
        for i in (0..24).step_by(2) {
            barrier_set.insert(Point { x: barriers[i], y: barriers[i + 1] });
        }

        SnakeState {
            head,
            body,
            tail,
            fruit: Point { x: fruit[0], y: fruit[1] },
            barriers: barrier_set,
        }
    }

    fn is_valid(&self, point: Point) -> bool {
        const L: i32 = 1;
        const H: i32 = 8;
        if point.x < L || point.x > H || point.y < L || point.y > H {
            return false;
        }
        if self.barriers.contains(&point) || self.body.contains(&point) {
            return false;
        }
        true
    }
}

fn bfs(mut state: SnakeState) -> Option<i32> {
    let directions = [
        (0, 1, 0),  // Up
        (-1, 0, 1), // Left
        (0, -1, 2), // Down
        (1, 0, 3),  // Right
    ];
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((state.head, Vec::new(), state.body.clone()));
    visited.insert(state.head);

    while let Some((current, path, mut snake_body_clone)) = queue.pop_front() {
        if current == state.fruit {
            return path.first().copied();
        }

        for &(dx, dy, dir) in &directions {
            let next_point = Point { x: current.x + dx, y: current.y + dy };

            if visited.contains(&next_point) || !state.is_valid(next_point) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(dir);

            snake_body_clone.pop_back(); // 移除尾部
            snake_body_clone.push_front(next_point); // 移动蛇头

            queue.push_back((next_point, new_path, snake_body_clone.clone()));
            visited.insert(next_point);
        }
    }
    None
}

#[wasm_bindgen]
pub fn greedy_snake_move_barriers(snake: &[i32], fruit: &[i32], barriers: &[i32]) -> i32 {
    let state = SnakeState::new(snake, fruit, barriers);

    if !state.is_valid(state.fruit) {
        return -1;
    }

    bfs(state).unwrap_or(-1)
}

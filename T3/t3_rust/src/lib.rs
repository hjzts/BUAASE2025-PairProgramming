use std::collections::{BinaryHeap, HashMap, HashSet};
use wasm_bindgen::prelude::*;
use std::cmp::Ordering;

/// 定义棋盘上的点
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// 计算曼哈顿距离
    fn manhattan(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// A* 搜索节点结构体
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    point: Point,
    f: i32, // f = g + h
    g: i32, // 起点到当前点的实际代价
}

// 为了使用 BinaryHeap，我们需要实现 Ord 和 PartialOrd（这里构造最小堆）
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// 定义蛇的数据结构，蛇体由四个点构成，头部为第一个坐标
#[derive(Debug)]
struct Snake {
    body: Vec<(i32, i32)>, // 四个坐标，顺序为头、第二节、第三节、蛇尾
}

impl Snake {
    fn head(&self) -> (i32, i32) {
        self.body[0]
    }
    fn second(&self) -> (i32, i32) {
        self.body[1]
    }
}

/// 解析传入的蛇坐标数组，返回 Snake 结构体
fn parse_snake(data: &[i32]) -> Snake {
    let body = data.chunks_exact(2)
        .map(|c| (c[0], c[1]))
        .collect();
    Snake { body }
}

/// 解析其他蛇的坐标以及自身蛇（除头外）生成障碍集合
fn parse_obstacles(n: i32, snake_num: i32, my_snake: &Snake, other_snakes: &[i32]) -> HashSet<(i32, i32)> {
    let mut obstacles = HashSet::new();
    // 把自己蛇身（除头）作为障碍
    obstacles.insert(my_snake.second());
    // 其他蛇，每条蛇数据长度为 8，分成四个点；这里选择忽略每条蛇的蛇尾（每4个点的最后一个）
    for snake_data in other_snakes.chunks_exact(8) {
        let mut i = 0;
        for chunk in snake_data.chunks_exact(2) {
            i += 1;
            // 忽略蛇尾（假设每第四个点为蛇尾，不作为障碍）
            if i % 4 == 0 {
                continue;
            }
            let obstacle = (chunk[0], chunk[1]);
            obstacles.insert(obstacle);
        }
    }
    // 将棋盘边界也视为障碍：设定 x, y 的合法范围为 [1, n]，因此边界点为 0 和 n+1
    for x in 0..=n+1 {
        obstacles.insert((x, 0));
        obstacles.insert((x, n+1));
    }
    for y in 0..=n+1 {
        obstacles.insert((0, y));
        obstacles.insert((n+1, y));
    }
    //如果大于两条蛇，则将其他蛇头下一步可能到的地方也加入障碍物里，防止两条蛇头撞在一起
    if snake_num > 2 {
        for snake_data in other_snakes.chunks_exact(8) {
            let mut i = 0;
            for chunk in snake_data.chunks_exact(2) {
                i += 1;
                // 忽略蛇尾（假设每第四个点为蛇尾，不作为障碍）
                if i % 4 == 1 {
                    // 解析蛇头的坐标
                    let head_x = chunk[0];
                    let head_y = chunk[1];

                    // 计算蛇头可能移动的四个方向
                    let possible_moves = [
                        (head_x, head_y + 1), // 上
                        (head_x, head_y - 1), // 下
                        (head_x - 1, head_y), // 左
                        (head_x + 1, head_y), // 右
                    ];

                    // 将可能的位置加入障碍物集合（避免重复检查）
                    for &(x, y) in &possible_moves {
                        if !obstacles.contains(&(x, y)) { 
                            obstacles.insert((x, y));
                        }
                    }
                }
            }
        }
    }
    obstacles
}

/// A* 算法：在 n x n 的棋盘上，从 start 到 goal 寻找一条可行路径
/// obstacles 为障碍集合（不能通过的点）
fn a_star(n: i32, start: Point, goal: Point, obstacles: &HashSet<(i32, i32)>) -> Option<Vec<Point>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();
    let mut f_score: HashMap<(i32, i32), i32> = HashMap::new();

    g_score.insert((start.x, start.y), 0);
    f_score.insert((start.x, start.y), start.manhattan(&goal));

    open_set.push(Node {
        point: start,
        g: 0,
        f: start.manhattan(&goal),
    });

    let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    while let Some(current) = open_set.pop() {
        if current.point == goal {
            // 还原路径
            let mut path = vec![current.point];
            let mut cur = (current.point.x, current.point.y);
            while let Some(&prev) = came_from.get(&cur) {
                path.push(Point { x: prev.0, y: prev.1 });
                cur = prev;
            }
            path.reverse();
            return Some(path);
        }

        for &(dx, dy) in &directions {
            let neighbor = Point {
                x: current.point.x + dx,
                y: current.point.y + dy,
            };

            if neighbor.x <= 0 || neighbor.x > n || neighbor.y <= 0 || neighbor.y > n {
                continue;
            }
            if obstacles.contains(&(neighbor.x, neighbor.y)) {
                continue;
            }

            let tentative_g = g_score.get(&(current.point.x, current.point.y)).unwrap_or(&i32::MAX) + 1;

            if tentative_g < *g_score.get(&(neighbor.x, neighbor.y)).unwrap_or(&i32::MAX) {
                came_from.insert((neighbor.x, neighbor.y), (current.point.x, current.point.y));
                g_score.insert((neighbor.x, neighbor.y), tentative_g);
                f_score.insert((neighbor.x, neighbor.y), tentative_g + neighbor.manhattan(&goal));
                open_set.push(Node {
                    point: neighbor,
                    g: tentative_g,
                    f: tentative_g + neighbor.manhattan(&goal),
                });
            }
        }
    }
    None
}


/// 根据 A* 搜索结果和当前蛇头位置返回移动方向
/// 方向数字对应：0->上，1->左，2->下，3->右
fn decide_direction(head: Point, next: Point) -> i32 {
    if next.x == head.x && next.y == head.y + 1 {
        0 // 上
    } else if next.x == head.x - 1 && next.y == head.y {
        1 // 左
    } else if next.x == head.x && next.y == head.y - 1 {
        2 // 下
    } else if next.x == head.x + 1 && next.y == head.y {
        3 // 右
    } else {
        -1 // 非法方向
    }
}



/// 使用 wasm_bindgen 对外暴露接口，决策函数根据当前局面返回蛇的移动方向
#[wasm_bindgen]
pub fn greedy_snake_step(
    n: i32,
    snake: &[i32],
    snake_num: i32,
    other_snakes: &[i32],
    food_num: i32,
    foods: &[i32],
    round: i32,
) -> i32 {
    let my_snake = parse_snake(snake);
    let obstacles = parse_obstacles(n, snake_num, &my_snake, other_snakes);
    let head = Point { x: my_snake.head().0, y: my_snake.head().1 };

    let mut best_path: Option<Vec<Point>> = None;
    let mut best_distance = i32::MAX;//设置一个很大的值，使其大于任何路径长度即可

    for food_chunk in foods.chunks_exact(2) {
        let food_point = Point { x: food_chunk[0], y: food_chunk[1] };
        if let Some(path) = a_star(n, head, food_point, &obstacles) {
            if path.len() < best_distance.try_into().unwrap() {
                best_distance = path.len() as i32;
                best_path = Some(path);
            }
        }
    }

    if let Some(path) = best_path {
        if path.len() > 1 {
            let next = path[1];
            print!["{}", decide_direction(head, next)];
            return decide_direction(head, next);
        }
    }

    0
}



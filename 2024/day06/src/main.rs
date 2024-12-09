use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./inputs/day06.txt").unwrap();

    let (starting_position, starting_direction, grid) = parse_input(&input);
    let mut guard = Guard::new(grid, starting_position, starting_direction);

    while guard.walking && !guard.looping {
        guard.walk();
    }

    println!("{}", guard.history.len());
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq)]
enum Case {
    Free(),
    Obstacle(),
}

#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

struct Guard {
    direction: Direction,
    position: Position,
    history: HashMap<Position, Vec<Direction>>,
    grid: HashMap<Position, Case>,
    walking: bool,
    looping: bool,
}

impl Guard {
    fn new(grid: HashMap<Position, Case>, position: Position, direction: Direction) -> Guard {
        let mut guard = Guard{
            position,
            direction,
            history: HashMap::new(),
            grid,
            walking: true,
            looping: false
        };

        guard.history.entry(position).or_insert(Vec::new()).push(direction);

        guard
    }

    fn can_walk_to(&mut self, position: Position) -> bool {
        if let Some(case) = self.grid.get(&position) {
            *case == Case::Free()
        } else {
            self.walking = false;
            false
        }
    }

    fn walk(&mut self) {
        let next_position =
            match self.direction {
                Direction::Up => self.move_up(),
                Direction::Down => self.move_down(),
                Direction::Left => self.move_left(),
                Direction::Right => self.move_right(),
            };

        if self.can_walk_to(next_position) {
            let case_history = self.history.entry(next_position).or_insert(Vec::new());

            if case_history.contains(&self.direction) {
                self.looping = true;
            } else {
               case_history.push(self.direction);
               self.position = next_position;
            }
        } else {
            self.rotate();
        }
    }

    fn move_up(&self) -> Position {
        Position{x: self.position.x, y: self.position.y - 1}
    }

    fn move_down(&self) -> Position {
        Position{x: self.position.x, y: self.position.y + 1}
    }

    fn move_left(&self) -> Position {
        Position{x: self.position.x - 1, y: self.position.y}
    }

    fn move_right(&self) -> Position {
        Position{x: self.position.x + 1, y: self.position.y}
    }

    fn rotate(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }
}

fn parse_input(input: &str) -> (Position, Direction, HashMap<Position, Case>) {
    let mut grid = HashMap::new();
    let mut starting_position = Position{x: 0, y: 0};
    let mut starting_direction = Direction::Up;

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let position = Position{x: col as i32, y: row as i32};

            match char {
                '#' => grid.entry(position).or_insert(Case::Obstacle()),
                '^' => {
                  starting_position = position;
                  starting_direction = Direction::Up;
                  grid.entry(position).or_insert(Case::Free())
                },
                '>' => {
                  starting_position = position;
                  starting_direction = Direction::Right;
                  grid.entry(position).or_insert(Case::Free())
                },
                'v' => {
                  starting_position = position;
                  starting_direction = Direction::Down;
                  grid.entry(position).or_insert(Case::Free())
                },
                '<' => {
                  starting_position = position;
                  starting_direction = Direction::Left;
                  grid.entry(position).or_insert(Case::Free())
                },
                _other => grid.entry(position).or_insert(Case::Free()),
            };
        }
    }

    (starting_position, starting_direction, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....\n\
                         .........#\n\
                         ..........\n\
                         ..#.......\n\
                         .......#..\n\
                         ..........\n\
                         .#..^.....\n\
                         ........#.\n\
                         #.........\n\
                         ......#...";

     #[test]
     fn parse_input_test() {
         let mut expected_grid: HashMap<Position, Case> = HashMap::new();
         expected_grid.insert(Position { x: 0, y: 0 }, Case::Free());
         expected_grid.insert(Position { x: 1, y: 0 }, Case::Obstacle());
         expected_grid.insert(Position { x: 2, y: 0 }, Case::Free());
         expected_grid.insert(Position { x: 3, y: 0 }, Case::Obstacle());
         expected_grid.insert(Position { x: 4, y: 0 }, Case::Free());

         let (starting_position, starting_direction, grid) = parse_input(&".#^#.");
         assert_eq!(starting_position, Position{x: 2, y: 0});
         assert_eq!(starting_direction, Direction::Up);
         assert_eq!(grid, expected_grid);
     }

    #[test]
    fn test_patrol() {
        let (starting_position, starting_direction, grid) = parse_input(INPUT);
        let mut guard = Guard::new(grid, starting_position, starting_direction);
        let mut grid = [['.'; 10]; 10];

        for (pos, case) in &guard.grid {
            if *case == Case::Free() {
                grid[pos.y as usize][pos.x as usize] = '.';
            } else {
                grid[pos.y as usize][pos.x as usize] = '#';
            }
        }

        while guard.walking && !guard.looping {
            guard.walk();
        }

        assert_eq!(guard.history.len(), 41);
    }
}

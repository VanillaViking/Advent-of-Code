use std::{fs, collections::{VecDeque, HashSet}};

#[derive(Debug, Clone)]
enum Lean {
    Forward,
    Back,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn get_next(&self, current: &Coordinate) -> Option<Coordinate> {
        match self {
            Direction::Up => Some(Coordinate { x: current.x, y: current.y.checked_sub(1)?, direction: current.direction.to_owned() }),
            Direction::Down => Some(Coordinate { x: current.x, y: current.y + 1, direction: current.direction.to_owned() }),
            Direction::Left => Some(Coordinate { x: current.x.checked_sub(1)?, y: current.y, direction: current.direction.to_owned() }),
            Direction::Right => Some(Coordinate { x: current.x + 1, y: current.y, direction: current.direction.to_owned() }),
        }
    }
}

#[derive(Debug, Clone)]
enum SplitterOrientation {
    Vertical,
    Horizontal
}

#[derive(Debug, Clone)]
enum TileType {
    Empty,
    Mirror(Lean),
    Splitter(SplitterOrientation)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
    direction: Direction
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    energy: u32,
    symbol: char,
}

impl Tile {
    fn get_next_tiles(&self, current: &Coordinate) -> impl Iterator<Item = Coordinate> {
        let mut next_tiles = Vec::new();

        match (self.tile_type.to_owned(), current.direction.to_owned()) {

            (TileType::Mirror(Lean::Back), Direction::Up) | (TileType::Mirror(Lean::Forward), Direction::Down) => {
                if let Some(x) = current.x.checked_sub(1) {
                    next_tiles.push(Coordinate { x, y: current.y, direction: Direction::Left})
                }
            },

            (TileType::Mirror(Lean::Back), Direction::Down) | (TileType::Mirror(Lean::Forward), Direction::Up) => next_tiles.push(Coordinate { x: current.x +1, y: current.y, direction: Direction::Right}),

            (TileType::Mirror(Lean::Back), Direction::Left) | (TileType::Mirror(Lean::Forward), Direction::Right) => {
                if let Some(y) = current.y.checked_sub(1) {
                    next_tiles.push(Coordinate { x: current.x, y, direction: Direction::Up})
                }
            },

            (TileType::Mirror(Lean::Back), Direction::Right) => next_tiles.push(Coordinate { x: current.x, y: current.y + 1, direction: Direction::Down}),
            (TileType::Mirror(Lean::Forward), Direction::Left) => next_tiles.push(Coordinate { x: current.x, y: current.y + 1, direction: Direction::Down}),

            (TileType::Splitter(SplitterOrientation::Vertical), Direction::Left) | (TileType::Splitter(SplitterOrientation::Vertical), Direction::Right) => {
                if let Some(y) = current.y.checked_sub(1) {
                    next_tiles.push(Coordinate { x: current.x, y, direction: Direction::Up });
                }
                next_tiles.push(Coordinate { x: current.x, y: current.y +1, direction: Direction::Down });
            },

            (TileType::Splitter(SplitterOrientation::Horizontal), Direction::Up) | (TileType::Splitter(SplitterOrientation::Horizontal), Direction::Down) => {
                if let Some(x) = current.x.checked_sub(1) {
                    next_tiles.push(Coordinate { x, y: current.y, direction: Direction::Left });
                }
                next_tiles.push(Coordinate { x: current.x +1, y: current.y, direction: Direction::Right });
            }

            _ => {
                if let Some(coord) = current.direction.get_next(current) {
                    next_tiles.push(coord);
                }
            },
        } 
        next_tiles.into_iter()
    }

    fn inc_energy(&mut self) {
        self.energy = self.energy + 1;
    }
}

#[derive(Debug)]
struct Contraption {
    grid: Vec<Vec<Tile>>
}

impl Contraption {
    fn build(input: &str) -> Contraption {
        let grid = input.lines()
            .map(|line| {
                line.chars().map(|ch| {
                    match ch {
                        '.' => Tile { tile_type: TileType::Empty, energy: 0, symbol: '.'},
                        '/' => Tile { tile_type: TileType::Mirror(Lean::Forward), energy: 0, symbol: '/' },
                        '\\' => Tile { tile_type: TileType::Mirror(Lean::Back), energy: 0, symbol: '\\' },
                        '|' => Tile { tile_type: TileType::Splitter(SplitterOrientation::Vertical), energy: 0, symbol: '|' },
                        '-' => Tile { tile_type: TileType::Splitter(SplitterOrientation::Horizontal), energy: 0, symbol: '-' },
                        _ => panic!()
                    }
                }).collect()
            })
            .collect();

        Contraption { grid }
    }

    fn draw(&self) {
        self.grid.iter().for_each(|line| {
            line.iter().for_each(|tile| {
                if tile.energy > 0 {
                    print!("#");
                } else {
                    print!("{}", tile.symbol);
                }
            });
            print!("\n");
        })
    }

    fn get(&self, coord: &Coordinate) -> Option<Tile> {
        Some(self.grid.get(coord.y)?.get(coord.x)?.to_owned())
    }

    fn get_mut(&mut self, coord: &Coordinate) -> Option<&mut Tile> {
        self.grid.get_mut(coord.y)?.get_mut(coord.x)
    }

    fn calculate_energies(&mut self) {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        
        queue.push_back(Coordinate {x: 0, y: 0, direction: Direction::Right});

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if seen.contains(&current) {
                continue;
            }

            seen.insert(current.to_owned());
            if let None = self.get(&current) {
                continue;
            }

            let tile = self.get_mut(&current).unwrap();
            tile.inc_energy();
            tile.get_next_tiles(&current).for_each(|coord| {
                queue.push_back(coord)
            });

        }
    }

    fn get_energized(&self) -> u32 {
        self.grid.iter().map(|row| {
            row.iter().filter_map(|tile| (tile.energy > 0).then_some(1)).sum::<u32>()
        }).sum()
    }
}


fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut contraption = Contraption::build(&input);

    contraption.calculate_energies();

    contraption.draw();

    println!("{}", contraption.get_energized());
}

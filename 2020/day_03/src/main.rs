use std::fs;

struct Simulation {
    map: Map,
    entity: Entity,
}

impl Simulation {
    fn new(map: &str, dx: usize, dy: usize) -> Self {
        Self {
            map: Map::new(map),
            entity: Entity::new(dx, dy),
        }
    }

    fn simulate(&mut self) -> usize {
        let mut trees = 0;
        loop {
            let (cx, cy) = self.entity.position;
            let (dx, dy) = self.entity.delta;

            if cy >= self.map.height {
                break;
            }

            if self.map.check_position(cx, cy) {
                trees += 1;
            }

            self.entity.position.0 = (cx + dx) % self.map.width;
            self.entity.position.1 = cy + dy;
        }

        trees
    }
}

#[derive(Copy, Clone)]
struct Entity {
    position: (usize, usize),
    delta: (usize, usize),
}

impl Entity {
    fn new(dx: usize, dy: usize) -> Self {
        Self {
            position: (dx, dy),
            delta: (dx, dy),
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    map: String,
}

impl Map {
    fn new(raw_map: &str) -> Self {
        let height = raw_map.lines().count();
        let map = raw_map.lines().map(|line| line.trim()).collect::<String>();

        Self {
            height,
            width: map.len() / height,
            map,
        }
    }

    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn check_position(&self, x: usize, y: usize) -> bool {
        let index = self.coords_to_index(x, y);
        *self.map.as_bytes().get(index).unwrap() == b'#'
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let a = Simulation::new(&input, 3, 1).simulate();
    let b = Simulation::new(&input, 1, 1).simulate();
    let c = Simulation::new(&input, 5, 1).simulate();
    let d = Simulation::new(&input, 7, 1).simulate();
    let e = Simulation::new(&input, 1, 2).simulate();

    let result = a * b * c * d * e;

    println!("Result Part A: {}", a);
    println!("Result Part B: {}", result);
}

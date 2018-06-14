extern crate librobots;

use std::io::Write;
use librobots::{Robots, PlayerTrait, EnemyTrait, Operations, CellStates, Vec2};

struct SimplePlayer {
    pos: Vec2
}

impl SimplePlayer {
    fn new(x: u64, y: u64) -> SimplePlayer {
        SimplePlayer { pos: Vec2::new(x, y) }
    }
}

impl PlayerTrait for SimplePlayer {
    fn pos(&self) -> Vec2 { self.pos }

    fn next(&mut self, size: Vec2, op: Operations, enemies: &Vec<Box<EnemyTrait>>) {
        let upper_bound = self.pos.y == 0;
        let lower_bound = self.pos.y == size.y;
        let left_bound = self.pos.x == 0;
        let right_bound = self.pos.x == size.x;

        let mut pos = self.pos;
        let (ex, ey) = (Vec2::new(1, 0), Vec2::new(0, 1));

        match op {
            Operations::Up() => {
                if upper_bound { return; }
                pos = self.pos - ey;
            }
            Operations::Down() => {
                if upper_bound { return; }
                pos = self.pos + ey;
            }
            Operations::Left() => {
                if upper_bound { return; }
                pos = self.pos - ex;
            }
            Operations::Right() => {
                if upper_bound { return; }
                pos = self.pos + ex;
            }
            Operations::UpperLeft() => {
                if upper_bound { return; }
                pos = self.pos - ex - ey;
            }
            Operations::UpperRight() => {
                if upper_bound { return; }
                pos = self.pos + ex - ey;
            }
            Operations::LowerLeft() => {
                if upper_bound { return; }
                pos = self.pos - ex + ey;
            }
            Operations::LowerRight() => {
                if upper_bound { return; }
                pos = self.pos + ex + ey;
            }
            Operations::Warp() => {
            }
            _ => ()
        }

        self.pos = pos;
    }
}

struct SimpleEnemy {
    pos: Vec2
}

impl SimpleEnemy {
    fn new(x: u64, y: u64) -> SimpleEnemy {
        SimpleEnemy { pos: Vec2::new(x, y) }
    }
}

impl EnemyTrait for SimpleEnemy {
    fn id(&self) -> u64 { 1 }
    fn pos(&self) -> Vec2 { self.pos }

    fn next(&mut self, size: Vec2, player: &Box<PlayerTrait>) {
        let (px, py) = (player.pos().x, player.pos().y);
        let mut pos = self.pos;
        let (ex, ey) = (Vec2::new(1, 0), Vec2::new(0, 1));

        if px < self.pos.x { pos = pos - ex; }
        if px > self.pos.x { pos = pos + ex; }
        if py < self.pos.y { pos = pos - ey; }
        if py > self.pos.y { pos = pos + ey; }

        self.pos = pos;
    }
}

fn display(g: &Robots) {
    let (width, height) = (g.size().x, g.size().y);

    println!("{}", "#".repeat((width + 2) as usize));
    for y in 0..height {
        print!("#");
        for x in 0..width {
            match g.at(Vec2::new(x, y)) {
                CellStates::Player() => { print!("P"); }
                CellStates::Enemy(_) => { print!("*"); }
                CellStates::Empty() => { print!(" "); }
                _ => ()
            }
        }
        print!("#\n");
    }
    println!("{}", "#".repeat((width + 2) as usize));
}

fn get_operation(prompt: &str) -> Operations {
    loop {
        let mut s = String::new();

        print!("{}", prompt);
        std::io::stdout().flush();
        std::io::stdin().read_line(&mut s);

        match s.trim_right_matches("\n") {
            "u" | "2" => return Operations::Up(),
            "m" | "8" => return Operations::Down(),
            "h" | "4" => return Operations::Left(),
            "k" | "6" => return Operations::Right(),
            "y" | "1" => return Operations::UpperLeft(),
            "i" | "3" => return Operations::UpperRight(),
            "n" | "7" => return Operations::LowerLeft(),
            "," | "9" => return Operations::LowerRight(),
            "j" | "5" => return Operations::Wait(),
            " " | "o" | "0" => return Operations::Warp(),
            _ => ()
        }
    }
}
fn main() {
    let size = Vec2::new(45, 15);
    let mut p: Box<PlayerTrait> = Box::new(SimplePlayer::new(size.x / 2, size.y / 2));
    let mut es: Vec<Box<EnemyTrait>> = Vec::new();
    for n in 0..5 {
        es.push(Box::new(SimpleEnemy::new(n, n))); 
    }

    let g = Robots::new(size, &mut p, &mut es);

    display(&g);

    let op = get_operation("$ ");
}

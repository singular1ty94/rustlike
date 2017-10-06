extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::*;
use rand::distributions::{IndependentSample,Range};
use std::cmp;
use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;

static MAP_WIDTH: i32 = 80;
static MAP_HEIGHT: i32 = 22;

static ROOM_MIN_W: i32 = 3;
static ROOM_MIN_H: i32 = 3;
static ROOM_MAX_W: i32 = 15;
static ROOM_MAX_H: i32 = 12;
static MAX_ROOMS: i32 = 20;

#[derive(Copy, Clone)]
struct Cell {
    x: i32,
    y: i32,
    c: char,
    passable: bool
}

impl Cell {
    fn draw(&self) {
        mvaddch(self.y, self.x, self.c as u64);
    }
}

fn is_passable(x: i32, y: i32, map: &mut DungeonMap) -> bool {
    return map.cells[x as usize][y as usize].passable;
}

#[derive(Copy, Clone)]
struct Entity {
    x: i32,
    y: i32,
    c: char,
    cl: i16
}

impl Entity {
    pub fn move_entity(&mut self, deltaX: i32, deltaY: i32, dungeon_map: &mut DungeonMap) {
        let newY = self.y + deltaY;
        let newX = self.x + deltaX;

        if is_passable(newX, newY, dungeon_map) {
            self.x = newX;
            self.y = newY;
        }
    }

    pub fn draw_entity(&mut self) {
        mvaddch(self.y, self.x, self.c as u64);
    }

    pub fn rand_char(&mut self) {
        let mut rstr: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(1)
        .collect();
        self.c = rstr.pop().unwrap();
    }
}

struct DungeonMap {
    entities: Vec<Entity>,
    cells: Vec<Vec<Cell>>,
    rooms: Vec<Room>
}

impl DungeonMap {
    fn make_room(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let room = Room {
            x1: x,
            x2: x + w,
            y1: y,
            y2: y + h
        };

        for x in (room.x1)..room.x2 {
            for y in (room.y1)..room.y2 {
                let floor = Cell {
                    x: x,
                    y: y,
                    c: '.',
                    passable: true
                };
                self.cells[x as usize][y as usize] = floor;
            }
        }
    }

    fn make_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
            let cell = Cell {
                x: x,
                y: y,
                c: '.',
                passable: true
            };
            self.cells[x as usize][y as usize] = cell;
        }
    }

    fn make_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
            let cell = Cell {
                x: x,
                y: y,
                c: '.',
                passable: true
            };
            self.cells[x as usize][y as usize] = cell;
        }
    }

    fn map_digger(&mut self, width: i32, height: i32) {
        for w in 1..width {
            for h in 1..height {
                let block = Cell {
                    x: w,
                    y: h,
                    c: '#',
                    passable: false
                };
                self.cells[w as usize][h as usize] = block;
            }
        }

        let mut num_rooms: i32 = 0;

        for r in 0..MAX_ROOMS {
            let randW = rand_int(ROOM_MIN_W, ROOM_MAX_W);
            let randH = rand_int(ROOM_MIN_H, ROOM_MAX_H);

            let randX = rand_int(1, MAP_WIDTH - randW - 1);
            let randY = rand_int(1, MAP_HEIGHT - randH - 1);

            let room = Room {
                x1: randX,
                y1: randY,
                x2: randX + randW,
                y2: randY + randH
            };

            let mut failed: bool = false;
            for other in self.rooms.iter_mut() {
                if room.intersect(other) {
                    failed = true;
                    break;
                }
            };

            if !failed {
                self.make_room(randX, randY, randW, randH);
                self.rooms.push(room);
                num_rooms = num_rooms + 1;
            } else {
                // Get the previous room 
                let (prev_x, prev_y) = self.rooms[(num_rooms - 1) as usize].center();
                let (new_x, new_y) = room.center();

                let coin = rand_int(0, 1);
                match coin {
                    0 => {
                        self.make_h_tunnel(prev_x, new_x, prev_y);
                        self.make_v_tunnel(prev_y, new_y, prev_x);
                    }
                    1 => {
                        self.make_v_tunnel(prev_y, new_y, prev_x);
                        self.make_h_tunnel(prev_x, new_x, prev_y);
                    },
                    _ => ()
                }
            }
        }

        for w in 0..width {
            let cell_top = Cell {
                x: w,
                y: 0,
                c: '#',
                passable: false
            };
            self.cells[w as usize][0 as usize] = cell_top;

            let cell_bottom = Cell {
                x: w,
                y: height,
                c: '#',
                passable: false
            };
            self.cells[w as usize][height as usize] = cell_bottom;
        }

        for h in 0..height {
            let cell_left = Cell {
                x: 0,
                y: h,
                c: '#',
                passable: false
            };
            self.cells[0 as usize][h as usize] = cell_left;

            let cell_right = Cell {
                x: width,
                y: h,
                c: '#',
                passable: false
            };
            self.cells[width as usize][h as usize] = cell_right;
        }
    }
}

// fn print_rl(colorFore: i16, colorBack: i16, text: &str, pair: i16) {
//     init_pair(pair, colorFore, colorBack);
//     attron(COLOR_PAIR(pair));
//     printw(text);
//     attroff(COLOR_PAIR(pair));
// }

// fn print_at(x: i32, y: i32, text: &str) {
//     mvprintw(y, x, text);
// }

fn player_action(dir: i32, player: &mut Entity, dungeon_map: &mut DungeonMap) {
    match dir {
        115 => player.move_entity(0, 1, dungeon_map),
        119 => player.move_entity(0, -1, dungeon_map),
        100 => player.move_entity(1, 0, dungeon_map),
        97 => player.move_entity(-1, 0, dungeon_map),
        _ => ()
    }
}

fn rand_int(low: i32, high: i32) -> i32 {
    let between = Range::new(low, high);
    let mut rng = rand::thread_rng();
    return between.ind_sample(&mut rng);
}

struct Room {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32
}

impl Room {
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        return (center_x, center_y);
    }

    pub fn intersect(&self, other: &Room) -> bool {
        return self.x1 <= other.x2 && self.x2 >= other.x1 && 
                self.y1 <= other.y2 && self.y2 >= other.y2
    }
}

fn main() {
    initscr();
    start_color();
    noecho();
    cbreak();

    curs_set(CURSOR_INVISIBLE);

    let mut player = Entity {
        x: 5,
        y: 5,
        c: '@',
        cl: COLOR_WHITE,
    };

    let mut dungeon_map = DungeonMap {
        entities: Vec::new(),
        cells: vec![vec![Cell { x:-1, y:-1, c: '!', passable: false }; (MAP_HEIGHT+1) as usize]; (MAP_WIDTH+1) as usize],
        rooms: Vec::new()
    };

    let enemy = Entity {
        x: 3,
        y: 3,
        c: 'c',
        cl: COLOR_RED
    };

    dungeon_map.entities.push(enemy);
    dungeon_map.map_digger(MAP_WIDTH, MAP_HEIGHT);

    loop {
        for c in dungeon_map.cells.iter() {
            for cell in c.iter() {
                cell.draw();
            }
        }

        for e in dungeon_map.entities.iter_mut() {
            e.rand_char();
            e.draw_entity();
        };
        
        player.draw_entity();    // Player always gets drawn by themselves
        refresh();

        let input: i32 = getch();
        player_action(input, &mut player, &mut dungeon_map);
        clear();
        player.draw_entity();
        refresh();
    }
}
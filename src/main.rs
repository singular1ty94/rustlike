extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::*;
use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;

static MAP_WIDTH: i32 = 80;
static MAP_HEIGHT: i32 = 22;

#[derive(Copy, Clone)]
struct Entity {
    x: i32,
    y: i32,
    c: char,
    cl: i16
}

#[derive(Copy, Clone)]
struct Cell {
    x: i32,
    y: i32,
    passable: bool
}

fn is_passable(x: i32, y: i32, map: &mut DungeonMap) -> bool {
    let mut iter = map.cells.iter();
    let cell = iter.find(|c| c.x == x && c.y == y).unwrap();
    return cell.passable;
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
    cells: Vec<Cell>
}

fn draw_entity(entity: &Entity) {
    mvaddch(entity.y, entity.x, entity.c as u64);
}

fn print_rl(colorFore: i16, colorBack: i16, text: &str, pair: i16) {
    init_pair(pair, colorFore, colorBack);
    attron(COLOR_PAIR(pair));
    printw(text);
    attroff(COLOR_PAIR(pair));
}

fn player_action(dir: i32, player: &mut Entity, dungeon_map: &mut DungeonMap) {
    match dir {
        115 => player.move_entity(0, 1, dungeon_map),
        119 => player.move_entity(0, -1, dungeon_map),
        100 => player.move_entity(1, 0, dungeon_map),
        97 => player.move_entity(-1, 0, dungeon_map),
        _ => ()
    }
}

fn map_digger(width: i32, height: i32, map: &mut DungeonMap) {
    for w in 0..width {
        for h in 0..height {
            mvaddch(h, w, '.' as u64);
            let cell = Cell {
                x: w,
                y: h,
                passable: false
            };
            map.cells.push(cell);
        }
    }

    for w in 0..width {
        mvaddch(0, w, '#' as u64);
        mvaddch(height, w, '#' as u64);
    }

    for h in 0..height {
        mvaddch(h, 0, '#' as u64);
        mvaddch(h, width, '#' as u64);
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
        cells: Vec::new()
    };

    let enemy = Entity {
        x: 3,
        y: 3,
        c: 'c',
        cl: COLOR_RED
    };

    dungeon_map.entities.push(enemy);

    print_rl(COLOR_RED, COLOR_BLACK, "Hello, Rust!\n", 1);

    loop {
        map_digger(MAP_WIDTH, MAP_HEIGHT, &mut dungeon_map);
        
        for entity in dungeon_map.entities.iter() {
            draw_entity(&entity);
        };
        draw_entity(&player);    // Player always gets drawn by themselves
        refresh();

        for e in dungeon_map.entities.iter_mut() {
            e.rand_char()
        };

        let input: i32 = getch();
        player_action(input, &mut player, &mut dungeon_map);
        clear();
        draw_entity(&player);
        refresh();
    }
}
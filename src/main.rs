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

impl Entity {
    pub fn moveEntity(&mut self, deltaX: i32, deltaY: i32) {
        self.x = self.x + deltaX;
        self.y = self.y + deltaY;
    }

    pub fn randChar(&mut self) {
        let mut rstr: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(1)
        .collect();
        self.c = rstr.pop().unwrap();
    }
}

struct DungeonMap {
    entities: Vec<Entity>
}

fn drawEntity(entity: &Entity) {
    mvaddch(entity.y, entity.x, entity.c as u64);
}

fn printRl(colorFore: i16, colorBack: i16, text: &str, pair: i16) {
    init_pair(pair, colorFore, colorBack);
    attron(COLOR_PAIR(pair));
    printw(text);
    attroff(COLOR_PAIR(pair));
}

fn playerAction(dir: i32, player: &mut Entity) {
    match dir {
        115 => player.moveEntity(0, 1),
        119 => player.moveEntity(0, -1),
        100 => player.moveEntity(1, 0),
        97 => player.moveEntity(-1, 0),
        _ => ()
    }
}

fn mapDigger(width: i32, height: i32) {
    for w in 0..width {
        for h in 0..height {
            mvaddch(h, w, '#' as u64);
        }
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

    let mut dungeonMap = DungeonMap {
        entities: Vec::new()
    };

    let mut enemy = Entity {
        x: 3,
        y: 3,
        c: 'c',
        cl: COLOR_RED
    };

    dungeonMap.entities.push(enemy);

    printRl(COLOR_RED, COLOR_BLACK, "Hello, Rust!\n", 1);

    loop {
        mapDigger(MAP_WIDTH, MAP_HEIGHT);
        
        for entity in dungeonMap.entities.iter() {
            drawEntity(&entity);
        };
        drawEntity(&player);    // Player always gets drawn by themselves
        refresh();

        for e in dungeonMap.entities.iter_mut() {
            e.randChar()
        };

        let input: i32 = getch();
        playerAction(input, &mut player);
        clear();
        drawEntity(&player);
        refresh();
    }   

    endwin();
}
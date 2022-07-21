use std::io;
//use std::ops::Range;
use emoji;
use rand::Rng;
use terminal::Value::TerminalSize;
//use try_catch::catch;
//use io::Error;

//use std::io::Stdout;
//use terminal::{Clear, Action, Value, Retrieved, error, Terminal};


pub struct Player {
    hp: u8,
    mana: u8,
    coords: (i32, i32),
    inventary: Vec<InvItem>,
    emoji_glyph: String
}

pub struct InvItem{
}

pub struct MapGame {
    rows : Vec<Vec<String>>
}

pub struct Monster {
    hp : u8,
    coords : (i32, i32),
    mob_glyph : String,
    mob_is_alive : bool
}

struct Test {
    st : &str
}

fn main() {
    let test = Test{st :"eeee"};

    let mut rng = rand::thread_rng();
    //let terminal: Terminal<Stdout> = terminal::stdout();

    //Create Player
    let player = Player{
        hp: 100,
        mana: 100,
        coords: (
            rng.gen_range(0..10),
            rng.gen_range(0..10)),
        inventary: Vec::new(),
        emoji_glyph: String::from(emoji::people_and_body::person_symbol::BUST_IN_SILHOUETTE
            .glyph)
    };


    // Generate map
    let mut map = MapGame{rows : Vec::new()};
    for _i in 0..11 as usize {
        let mut row = Vec::new();
        for _j in 0..11 as usize {
            let _ = &mut row.push(
                String::from(
                    emoji::animals_and_nature::plant_other::DECIDUOUS_TREE
                        .glyph
                )
            );
        }
        map.rows.push(row);
    }

    let mut mob_vec: Vec<Monster> = Vec::new();
    for _ in 0..3 {
        let monster = Monster{
            hp :100, coords : (
                rng.gen_range(0..10),
                rng.gen_range(0..10)
            ),
            mob_glyph : emoji::animals_and_nature::animal_mammal::BEAR
                .glyph
                .to_string(),
            mob_is_alive : true};
        mob_vec.push(monster);
    }
    //println!("{:?}", mob_vec);

    render_game(&player, &map, &mob_vec);
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
}

fn render_game(player: &Player, map :&MapGame, mob_vec : &Vec<Monster>){
   //terminal.act(Action::ClearTerminal(Clear::All)).unwrap();
    {
        let mut x = 0;
        let mut y = 0;
        for row in &map.rows
        {
            for val in row
            {
                let mut free_cell = true;

                //replaced Mobs
                for mob in mob_vec.iter() {
                    if mob.coords == (x, y) {
                        print!("{}", mob.mob_glyph);
                        free_cell = false;
                    }
                }

                // replaced Player
                if player.coords == (x, y) {
                    print!("{}", player.emoji_glyph);
                    free_cell = false;
                }

                // replaced Tree
                if free_cell {
                    print!("{}", *val);
                }
                x += 1;
            }
            x = 0;
            y += 1;
            println!("{}", emoji::symbols::geometric::DIAMOND_WITH_A_DOT.glyph);
        }
    }
    let mut sst = String::new();
    for i in 0..12{
        sst.push_str(emoji::symbols::geometric::DIAMOND_WITH_A_DOT.glyph);
    }
    println!("{}", sst)
}














    /*
    // let mut map: Matrix = Matrix{rows : [
    //     [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
    //     [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
    //     [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
    //     [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
    //     [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
    //     [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
    //     [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
    //     [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
    //     [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
    //     [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree]
    // ]};

     Создание случайных координат для Главного Героя
    let mut rng = rand::thread_rng();
    let mut coord_gg: [usize; 2] = [rng.gen_range(0..10), rng.gen_range(0..10)];
    println!("{:?}", coord_gg);
    let [x, y] = coord_gg;
    //map.rows[x][y] = &man; // Расположение Главного Героя на карте

    print_map(&map, &coord_gg, &man, &terminal);

    while true {
        let mut motion_str = String::new();
        io::stdin().read_line(&mut motion_str).unwrap();
        let mut split =  motion_str.char_indices();
        //let moved_vec = split[1];

        match split.next() {
            Some(x) => move_gg(x, &map, &mut coord_gg),
            None => println!("введите")
        }

        print_map(&map, &coord_gg, &man, &terminal);
    }

}

fn move_gg(vecmov :(usize, char), map : &Matrix<'_>, coords_gg: &mut [usize]){
    //println!("{vecmov:?}");

    match vecmov.1 {
        'w' => coords_gg[1] = coords_gg[1].wrapping_sub(1),
        's' => coords_gg[1] = coords_gg[1].wrapping_add(1),
        'a' => coords_gg[0] = coords_gg[0].wrapping_sub(1),
        'd' => coords_gg[0] = coords_gg[0].wrapping_add(1),
        _ => print!("vfff")
    }
    //println!("{:?}",coords_gg);

}

fn print_map(map : &Matrix<'_>, coords_gg :&[usize], man: &cell_symbol, terminal :&Terminal<Stdout>){
    terminal.act(Action::ClearTerminal(Clear::All)).unwrap();
    let mut x: usize =0;
    let mut y: usize = 0;
    //print!("{:?}",coords_gg);
    for row in map.rows {
        for val in row {
            if [x, y] == coords_gg {
                print!("{}",man.symbol.glyph);
            }
            else {
                //print!("{:?}",[x,y]);
                print!("{}",val.symbol.glyph);
            }
            x+=1;
        }
        x = 0;
        y+=1;
        println!();
    }
}
*/
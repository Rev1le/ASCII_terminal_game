use std::io;
//use std::ops::Range;
use emoji;
use rand::Rng;
use terminal::Value::TerminalSize;
//use try_catch::catch;


//use std::io::Stdout;
//use terminal::{Clear, Action, Value, Retrieved, error, Terminal};

pub struct Player {
    hp: u8,
    mana: u8,
    coords: (i32, i32),
    inventary: Vec<InvItem>,
    emoji_glyph: String
}

pub struct InvItem{}

pub struct MapGame { rows : Vec<Vec<String>> }

pub struct Monster {
    hp : u8,
    coords : (i32, i32),
    mob_glyph : String,
    mob_is_alive : bool
}


fn main() {

    let (mut player, mut map, mut mob_vec) = generate_static_object();

    render_game(&player, &map, &mob_vec); // начальное расположение карты

    loop {
        let mut motion_str = String::new();
        io::stdin().read_line(&mut motion_str).unwrap();
        let mut split =  motion_str.char_indices();
        //let moved_vec = split[1];

        match split.next() {
            Some(x) => move_gg(x, &mut player, &mob_vec), //Движение персонаджа по карте и регистрация событий
            None => println!("введите")
        }

        render_game(&player, &map, &mob_vec);
    }

    //
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();


}

fn generate_static_object() -> (Player, MapGame, Vec<Monster>){
    let mut rng = rand::thread_rng();
    //let terminal: Terminal<Stdout> = terminal::stdout();

    //Create Player
    let mut player = Player{
        hp: 100,
        mana: 100,
        coords: (
            rng.gen_range(0..10), //  x
            rng.gen_range(0..10)),//  y
        inventary: Vec::new(),
        emoji_glyph: String::from(emoji::people_and_body::person_symbol::BUST_IN_SILHOUETTE
            .glyph)
    };

    // Generate map
    // генерация строк не нужна, есть .clone()
    let mut map = MapGame{rows : Vec::new()};

    let mut row = Vec::new();
    for _j in 0..11 as usize {
        let _ = &mut row.push(
            String::from(
                emoji::animals_and_nature::plant_other::DECIDUOUS_TREE
                    .glyph)
        );
    }

    for _i in 0..11 as usize {
        let copy_row = row.clone();
        map.rows.push(copy_row);
    }

    //Generate mob in Vector
    let mut mob_vec: Vec<Monster> = Vec::new();
    for _ in 0..3 {
        let mut monster = Monster{
            hp :100, coords : (
                rng.gen_range(0..10),
                rng.gen_range(0..10)),
            mob_glyph : emoji::animals_and_nature::animal_mammal::BEAR
                .glyph
                .to_string(),
            mob_is_alive : true};
        mob_vec.push(monster);
    }

    //return all value
    return (player, map, mob_vec)
}


fn move_gg(vecmov :(usize, char), player: &mut Player, mob_vec: &Vec<Monster>){

    let rus;
    match vecmov.1 {
        'w' => rus = check_event(player, ('y', -1), &mob_vec),
        's' => rus = check_event(player, ('y', 1), &mob_vec),
        'a' => rus = check_event(player, ('x', -1), &mob_vec),
        'd' => rus = check_event(player, ('x', 1), &mob_vec),
        _ => println!("Неверное напрвление движения. Введите корректные: (w - вверх, s - вниз, a - влево, d - вправо)")
    }
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

                // replaced Player
                if player.coords == (x, y) {
                    print!("{}", player.emoji_glyph);
                    free_cell = false;
                }

                //replaced Mobs
                for mob in mob_vec.iter() {
                    if mob.coords == (x, y) && free_cell {
                        print!("{}", mob.mob_glyph);
                        free_cell = false;
                    }
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

    //Создание разграничения карты от интерфейса
    let mut sst = String::new();
    for _ in 0..12{
        sst.push_str(emoji::symbols::geometric::DIAMOND_WITH_A_DOT.glyph);
    }
    println!("{}", sst)
}


fn check_event(player: &mut Player, move_coord :(char, i32), mobs_vec : &Vec<Monster>) -> Option<char>{

    let mut has_event = false;

    match move_coord.0 {
        'y' => {
            for mob in mobs_vec{
                if player.coords.1 + move_coord.1 == mob.coords.1 && player.coords.0 == mob.coords.0{
                    println!("{} {}",player.coords.1 + move_coord.1,  mob.coords.1);
                    has_event = true;
                }
            }
            player.coords.1 += move_coord.1;
        },
        'x' => {
            for mob in mobs_vec{
                if player.coords.0 + move_coord.1 == mob.coords.0 && player.coords.1 == mob.coords.1 {
                    println!("{} {}",player.coords.0 + move_coord.1,  mob.coords.0);
                    has_event = true;
                }
            }
            player.coords.0 += move_coord.1;
        },
        _ => {}
    }
    if has_event {
        return Some('f') //возвращает символ 'f' - fight
    }
    return None
}

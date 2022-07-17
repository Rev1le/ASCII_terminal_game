use std::io;
use std::ops::Range;
use emoji;
use rand::Rng;
//use std::error::Error;
use try_catch::catch;
use io::Error;

pub struct cell_symbol {
    symbol: emoji::Emoji
}


pub struct Matrix<'a> {
    rows: [[&'a cell_symbol; 10]; 10]
}


fn main() {
    let man: cell_symbol = cell_symbol{symbol :emoji::people_and_body::person_symbol::BUST_IN_SILHOUETTE};
    let tree: cell_symbol = cell_symbol{symbol : emoji::animals_and_nature::plant_other::DECIDUOUS_TREE};

    let mut map: Matrix = Matrix{rows : [
        [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
        [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
        [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
        [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
        [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
        [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
        [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
        [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
        [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree],
        [&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree,&tree]
    ]};

    /* Создание случайных координат для Главного Героя */
    let mut rng = rand::thread_rng();
    let mut coord_gg: [usize; 2] = [rng.gen_range(0..10), rng.gen_range(0..10)];
    println!("{:?}", coord_gg);
    let [x, y] = coord_gg;
    //map.rows[x][y] = &man; // Расположение Главного Героя на карте

    print_map(&map, &coord_gg, &man);

    while true {
        let mut motion_str = String::new();
        io::stdin().read_line(&mut motion_str).unwrap();
        let mut split =  motion_str.char_indices();
        //let moved_vec = split[1];

        match split.next() {
            Some(x) => move_gg(x, &map, &mut coord_gg),
            None => println!("введите")
        }

        print_map(&map, &coord_gg, &man);
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

fn print_map(map : &Matrix<'_>, coords_gg :&[usize], man: &cell_symbol){
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
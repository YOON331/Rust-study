use std::collections::HashMap;
use std::env;
use std::io;
use std::process;

enum Element {
    Location(i32, i32),
}

fn main() {
    let op_hash = set_op();

    let map_hash = set_map_star();
    let person_hash = set_person();
    let star_hash = set_star();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- stage <stage num>\n(We have two stages: stage 1: easy, stage 2: normal)");
        process::exit(1);
    }

    let stage = match args[2].clone().as_str() {
        "1" => 1,
        "2" => 2,
        _ => 0,
    };

    if stage == 0 {
        eprintln!("Usage: cargo run -- stage <stage num>\n(We have two stages: stage 1: easy, stage 2: normal)");
        process::exit(1);
    }

    let map = get_value(map_hash, stage);
    let person = get_value_info(person_hash, stage);
    let star_arr = get_value_info(star_hash, stage);

    let tmp: Vec<&str> = person.split_ascii_whitespace().collect();

    let mut person = Element::Location(
        tmp[0].parse().expect("person location x error"),
        tmp[1].parse().expect("person location y error"),
    );

    let mut star = make_star_vec(star_arr);
    let star_len = star.len();
    let mut game_map = set_map(&map, &star, &person);

    println!(
        "\n--- Push Push Game Start ---\nw: up, s: down, a: left, d: right\nr: reset, q: quit"
    );

    loop {
        display_map(&game_map);

        if chk_complete(&game_map, star_len) {
            break;
        }

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Invalid input");
        let user_input = user_input.trim();

        match user_input {
            "q" => {
                println!("\n=== Finish Game ===");
                display_map(&game_map);
                break;
            }
            "r" => {
                person = Element::Location(
                    tmp[0].parse().expect("person location x error"),
                    tmp[1].parse().expect("person location y error"),
                );
                star = make_star_vec(star_arr);
                game_map = set_map(&map, &star, &person);
                println!("\n=== Restart Game ===");
            }
            _ => {
                let op = match op_hash.get(user_input) {
                    Some(val) => val,
                    None => {
                        println!("Invalid input");
                        continue;
                    }
                };

                if move_op(&game_map, op, &mut star, &mut person) {
                    game_map = set_map(&map, &star, &person);
                }
            }
        }
    }
}

fn make_star_vec(star_arr: &str) -> Vec<Element> {
    let tmp: Vec<&str> = star_arr.split(',').collect();

    let mut star_vec: Vec<Element> = Vec::new();

    for i in tmp.iter() {
        let temp: Vec<&str> = i.split_ascii_whitespace().collect();
        star_vec.push(Element::Location(
            temp[0].parse().expect("star location x error"),
            temp[1].parse().expect("star location y error"),
        ));
    }

    star_vec
}

fn set_map(map: &Vec<Vec<i32>>, star: &Vec<Element>, person: &Element) -> Vec<Vec<i32>> {
    let mut new_map = map.clone();

    for i in star {
        let Element::Location(x, y) = i;
        if let Some(elem) = new_map
            .get_mut(*y as usize)
            .expect("map y error")
            .get_mut(*x as usize)
        {
            if *elem == 2 {
                *elem = 9;
            } else {
                *elem = 3;
            }
        }
    }

    let Element::Location(x, y) = person;
    if let Some(elem) = new_map
        .get_mut(*y as usize)
        .expect("person error")
        .get_mut(*x as usize)
    {
        *elem = 7; // 사람
    }

    new_map
}

fn match_op(op: &i32, x: &mut i32, y: &mut i32) -> bool {
    match op {
        8 => {
            *y -= 1;
        }
        6 => {
            *x += 1;
        }
        2 => {
            *y += 1;
        }
        4 => {
            *x -= 1;
        }
        _ => return false,
    }
    true
}

fn back_match_op(op: &i32, x: &mut i32, y: &mut i32) {
    match op {
        8 => {
            *y += 1;
        }
        6 => {
            *x -= 1;
        }
        2 => {
            *y -= 1;
        }
        4 => {
            *x += 1;
        }
        _ => {}
    }
}

fn chk_val(
    val: Option<i32>,
    x: &mut i32,
    y: &mut i32,
    game_map: &Vec<Vec<i32>>,
    star: &mut Vec<Element>,
    op: &i32,
) -> bool {
    match val {
        Some(1) => {
            // 아무것도 없을 때 - 이동 가능
            match_op(op, x, y);
            return true;
        }
        Some(2) => {
            // 아무것도 없을 때 - 이동 가능
            match_op(op, x, y);
            return true;
        }
        Some(3) => {
            match_op(op, x, y);
            if move_star(star, x, y, game_map, op) {
                return true;
            }
            back_match_op(op, x, y);
        } // 이동해야할 자리에 star가 있을 때 - star도 이동 가능한지 확인해봐야 함
        Some(9) => {
            match_op(op, x, y);
            if move_star(star, x, y, game_map, op) {
                return true;
            }
            back_match_op(op, x, y);
        }
        Some(_) => {}
        None => {}
    }
    false
}

fn move_op(
    game_map: &Vec<Vec<i32>>,
    op: &i32,
    star: &mut Vec<Element>,
    person: &mut Element,
) -> bool {
    let Element::Location(x, y) = person;
    let map_size = game_map.len() as i32;

    if *y - 1 < 0 || *y + 1 > map_size - 1 || *x - 1 < 0 || *x + 1 > map_size - 1 {
        return false;
    }

    match op {
        8 => {
            let val: Option<&i32> = game_map
                .get((*y - 1) as usize)
                .expect("move op up val error")
                .get(*x as usize);
            if chk_val(val.copied(), x, y, game_map, star, op) {
                *person = Element::Location(*x, *y);
            }
        }
        2 => {
            let val = game_map
                .get((*y + 1) as usize)
                .expect("move op down val error")
                .get(*x as usize);
            if chk_val(val.copied(), x, y, game_map, star, op) {
                *person = Element::Location(*x, *y);
            }
        }
        4 => {
            let val = game_map
                .get(*y as usize)
                .expect("move left down val error")
                .get((*x - 1) as usize);
            if chk_val(val.copied(), x, y, game_map, star, op) {
                *person = Element::Location(*x, *y);
            }
        }
        6 => {
            let val = game_map
                .get(*y as usize)
                .expect("move right right val error")
                .get((*x + 1) as usize);
            if chk_val(val.copied(), x, y, game_map, star, op) {
                *person = Element::Location(*x, *y);
            }
        }
        _ => {
            return false;
        }
    }
    true
}

fn move_star(
    star: &mut Vec<Element>,
    x: &mut i32,
    y: &mut i32,
    game_map: &Vec<Vec<i32>>,
    op: &i32,
) -> bool {
    for i in star {
        let Element::Location(x_i, y_i) = i;

        if *x == *x_i && *y == *y_i {
            // 이동해줘야 함 - op에 따라
            match_op(op, x_i, y_i);

            let val = *game_map
                .get(*y_i as usize)
                .expect("err")
                .get(*x_i as usize)
                .unwrap();
            match val {
                1 => {
                    return true;
                } // 비어있는 길 ok
                2 => {
                    *i = Element::Location(*x_i, *y_i);
                    return true;
                } // 창고
                3 => {
                    back_match_op(op, x_i, y_i);
                }
                0 => {
                    back_match_op(op, x_i, y_i);
                }
                9 => {
                    back_match_op(op, x_i, y_i);
                }
                _ => {}
            };
        }
    }
    false
}

fn chk_complete(map: &Vec<Vec<i32>>, star_len: usize) -> bool {
    let mut cnt = 0;
    for y in map {
        for x in y {
            match x {
                9 => cnt += 1,
                _ => (),
            }
        }
    }

    if cnt == star_len {
        println!("=== complete game ===");
        return true;
    }
    false
}

fn display_map(map: &Vec<Vec<i32>>) {
    for y in map {
        for x in y {
            match x {
                1 => print!("1"),
                2 => print!("O"),
                3 => print!("*"),
                9 => print!("@"),
                7 => print!("8"),
                _ => print!(" "),
            }
            print!(" ");
        }
        println!("");
    }
}

fn set_op() -> HashMap<String, i32> {
    let mut op = HashMap::new();

    op.insert("w".to_string(), 8); // up
    op.insert("s".to_string(), 2); // down
    op.insert("a".to_string(), 4); // left
    op.insert("d".to_string(), 6); // right

    op
}

fn set_map_star() -> HashMap<i32, Vec<Vec<i32>>> {
    let map1: Vec<Vec<i32>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 2, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 2, 0],
        vec![0, 2, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 2, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let map2: Vec<Vec<i32>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 0, 0, 2, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 2, 0],
        vec![0, 0, 0, 1, 1, 1, 1, 2, 0],
        vec![0, 0, 1, 1, 1, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let mut map_star = HashMap::new();

    map_star.insert(1, map1); // 1라운드
    map_star.insert(2, map2); // 2라운드

    map_star
}

fn set_star() -> HashMap<i32, &'static str> {
    let mut star = HashMap::new();

    star.insert(1, "3 3, 3 4, 4 5, 5 3");
    star.insert(2, "2 2, 3 2, 2 3");

    star
}

fn set_person() -> HashMap<i32, &'static str> {
    let mut person = HashMap::new();

    person.insert(1, "4 4");
    person.insert(2, "1 1");

    person
}

fn get_value(hashmap: HashMap<i32, Vec<Vec<i32>>>, key: i32) -> Vec<Vec<i32>> {
    let value = match hashmap.get(&key) {
        Some(map) => map,
        None => panic!("get value error"),
    };

    value.to_vec()
}

fn get_value_info(hashmap: HashMap<i32, &'static str>, key: i32) -> &'static str {
    let value = match hashmap.get(&key) {
        Some(map) => map,
        None => panic!("get value error"),
    };

    value
}

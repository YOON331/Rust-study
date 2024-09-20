use std::{io, usize};
use array2d::Array2D;

const N_SIZE:i32 = 6;

#[derive(Debug)]
struct Car {
    number: usize,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    capa: i32,
    direction: char,
}

fn main() {
    let mut puzzle = Array2D::filled_with(0,N_SIZE.try_into().unwrap(),N_SIZE.try_into().unwrap());

    // 퍼즐 구성 입력값
    let input_arr = ["A4", "horizontal", "2", "\n", "C6", "vertical", "3", "\n", "A3", "horizontal","3","\n", "F3","vertical", "3", "\n"];
    // let input_arr = ["C4","horizontal","2", "\n", "C6", "vertical", "2", "\n",  "D5", "horizontal", "3", "\n", "E4", "vertical", "2", "\n", "E2", "vertical", "2", "\n"];

    let mut cars_info = Vec::<Car>::new();
    let mut puzzle_state = 0;   // 시작 전 상태

    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("failed to read line");
        let user_input = user_input.trim();
        match user_input {
            "start" => {    // 시작 
                cars_info = load_game(&input_arr);
                puzzle = update_puzzle(&cars_info);
                display(&puzzle);
                puzzle_state = 1;
            },  // 종료
            "quit" => {
                println!("Finish Rush Hour Puzzle");
                break;
            },
            _ => {
                let user_input: Vec<&str> = user_input.split(' ').collect();

                if puzzle_state == 0 || user_input.len() != 2 { 
                    println!("len Input Error");
                    continue;
                }


                let car_num: usize = match user_input.get(1).expect("car_num error").parse() {
                    Ok(num) => num,
                    _ => {
                        println!("Input Error");
                        continue;
                    }
                };

                let mut op: &str = *user_input.get(0).expect("op error");

                if car_num -1 > cars_info.len() {   // 유요하지 않은 자동차 번호를 입력했을 때 예외 처리
                    println!("Input Error");
                    continue;
                }

                move_car(&puzzle, &mut cars_info, car_num, op);
                puzzle = update_puzzle(&cars_info);
                display(&puzzle);
                
                if check_destination(&puzzle) {
                    break;
                }

            }
        }
    }


}

// 화면 출력
fn display(puzzle: &Array2D<usize>) {
    for rows in puzzle.rows_iter() {
        for element in rows {
            if *element == 0 {  // &usize로 반환되어 값 비교를 할 수 없어서 *(역참조) 키워드로 값 비교
                print!("+ ");
                continue;
            }
            print!("{element} ");
        }
        println!("");
    }
    println!("");
}

fn load_game(input_arr: &[&str]) -> Vec<Car> {   // input arr의 정보를 읽어와서 Vec<car> 타입으로 반환
    let mut cars = Vec::<Car>::new();
    let mut car_num = 0;
    let mut car = Car {
        number: 0,
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
        capa: 0,
        direction: 'n'
    };

    for (idx, val) in input_arr.iter().enumerate() {
        let remain = idx % 4;
        match remain {
            0 => {
                car_num += 1;   // 자동차 번호 1씩 증가
                car.number = car_num;   // car 구조체의 number에 car_num 값으로 업데이트

                match &val[..1] {   // x1 좌표
                    "A" => car.x1 = 0,
                    "B" => car.x1 = 1,
                    "C" => car.x1 = 2,
                    "D" => car.x1 = 3,
                    "E" => car.x1 = 4,
                    "F" => car.x1 = 5,
                    _ => (),
                }
                car.y1 = match val[1..].parse() {   // y1 좌표
                    Ok(num) => num,
                    Err(_) => {
                        println!("car cell error");
                        break;
                    },
                };
            },
            1 => {  // direction 
                if &val[..1] == "h" {
                    car.direction = char::from('h');
                    continue;
                } 
                car.direction = char::from('v');
                
            },
            2 => {  // car 크기 (capa)
                let val: i32 = match val.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("car capability parse error");
                        break;
                    }
                };
                car.capa = val;
            },
            3 => {  // 개행문자를 만나면 '\n'
                match car.direction {
                    'h' => {
                        car.y2 = car.y1;
                        car.x2 = car.x1 + car.capa;
                    },
                    'v' => {
                        car.y2 = car.y1 - car.capa;
                        car.x2 = car.x1;
                    },
                    _ => (),
                }

                let tmp = Car {
                    direction: car.direction.clone(),
                    ..car
                };
                cars.push(tmp);
            }
            _ => (),
        }
    }
    cars
}

fn update_puzzle (cars_info: &Vec<Car>) -> Array2D<usize>{
    let mut new_puzzle = Array2D::filled_with(0, N_SIZE.try_into().unwrap(), N_SIZE.try_into().unwrap());

    for idx in cars_info.into_iter() {
        match idx.direction {
            'h' => {    // x값만 변동
                for i in idx.x1..idx.x2 {
                    new_puzzle.set((N_SIZE-idx.y1).try_into().unwrap(), i.try_into().unwrap(), idx.number);
                }
            },
            'v' => {    // y값만 변동 
                for i in idx.y2..idx.y1 {
                    new_puzzle.set((N_SIZE-i-1).try_into().unwrap(), idx.x1.try_into().unwrap(), idx.number);
                }
            },
            _ => (),  
        } 
    }
    new_puzzle
}

fn move_car(puzzle: &Array2D<usize>, cars_info :&mut Vec<Car>, car_num: usize, op: &str) {  // cars_info에서 
    let car_idx = car_num - 1;

    let x1 = cars_info[car_idx].x1;
    let x2 = cars_info[car_idx].x2;
    let y1 = cars_info[car_idx].y1;
    let y2 = cars_info[car_idx].y2;

    if cars_info[car_idx].direction == 'h' {    // h 인 경우
        match op {
            "right" => { //right - x좌표만 오른쪽으로 이동 (범위 벗어나거나 이미 다른 차가 있으면 x )
                if  x2 + 1 > 6 || puzzle.get((N_SIZE - y1).try_into().unwrap(), (x2).try_into().unwrap()) != Some(&0) {
                    println!("right invalid data");
                } else {
                    cars_info[car_idx].x1 = x1 + 1;
                    cars_info[car_idx].x2 = x2 + 1;
                }
            },
            "left" => {  // left - x좌표만 왼쪽으로 이동
                if x1 - 1 < 0 || puzzle.get((N_SIZE-y1).try_into().unwrap(), (x1-1).try_into().unwrap()) != Some(&0) {
                    println!("left invalid data");
                } else {
                    cars_info[car_idx].x1 = x1 - 1;
                    cars_info[car_idx].x2 = x2 - 1;
                }
            },
            _ => {
                println!("invalid data");
            }
        }
    } else {    // v인 경우
        match op {
            "up" => {  // up
                if y1 + 1 > 6 || puzzle.get((N_SIZE-y1-1).try_into().unwrap(), x1.try_into().unwrap()) != Some(&0){
                    println!("up invalid data");
                } else {
                    cars_info[car_idx].y1 = y1 + 1;
                    cars_info[car_idx].y2 = y2 + 1;
                }
            },
            "down" => {  // down
                if N_SIZE - y2 - 1 < 0 || puzzle.get((N_SIZE-y2).try_into().unwrap(), x1.try_into().unwrap()) != Some(&0) {
                    println!("down invalid data");
                } else {
                    cars_info[car_idx].y1 = y1 - 1;
                    cars_info[car_idx].y2 = y2 - 1;
                }
            },
            _ => {
                println!("invalid data");
            }
        }
    }
}

fn check_destination (puzzle: &Array2D<usize>) -> bool {
    if puzzle.get(2,5) == Some(&1) {
        println!("complete!");
        true
    } else {
        false
    }
}
use std::env;
use rppal::gpio::{Gpio, Level};
use mysql::conn::pool::{Pool};
use mysql::conn::pool::Opts;

fn main() {
    let mut gpio = Gpio::new().unwrap();
    let mut ports = [29, 31, 32, 33, 35, 36, 37, 38, 40];

    let pool = Pool::new(Opts::from_url(&env::var("DATABASE_URL").unwrap()).unwrap()).unwrap();
    let mut conn = pool.get_conn().unwrap();

    conn.query("UPDATE areaStat SET area1 = area1 + 1").unwrap();

    fn increment(conn: &mut mysql::conn::pool::PooledConn, area_num: i32) {
        let query = format!("UPDATE areaStat SET area{} = area{} + 1", area_num, area_num);
        conn.query(query).unwrap();
    }

    for port in ports.iter_mut() {
        gpio.set_mode(*port, gpio::Input);
        gpio.set_async_interrupt(*port, gpio::Edge::Both, move |level: Level| {
            println!("a");

            let mut board = [0,0,0,0,0,0,0,0,0];
            let mut player = 1;
            let mut winner = 0;
            let mut moves = 0;
            let mut gameover = false;

            println!("Channel {} value is now {}", port, level);
            if level == Level::High {
                match port {
                    29 => increment(&mut conn, 1),
                    31 => increment(&mut conn, 2),
                    32 => increment(&mut conn, 3),
                    33 => increment(&mut conn, 4),
                    35 => increment(&mut conn, 5),
                    36 => increment(&mut conn, 6),
                    37 => increment(&mut conn, 7),
                    38 => increment(&mut conn, 8),
                    40 => increment(&mut conn, 9),
                    _ => (),
                }

                // Remove the code that checks for the winner
            }
            if moves == 9 {
                moves = 0;
                gameover = true;
            }
            if player == 1 {
                player = 2;
            }
            else {
                player = 1;
            }
            moves += 1;
            if gameover == true {
                println!("win");
            }
        });
    }
}

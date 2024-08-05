use std::{sync::{Arc, Mutex}, thread::sleep, time::{Duration, Instant}};
use console::{Style, Term};
use rand::Rng;

const WIDTH: i32 = 100;
const HEIGHT: i32 = 30;
const PETAL_COUNT: usize = 25;
const PETAL_GENERATION_INTERVAL_MS: u64 = 350;

fn main() {
    let term = Arc::new(Mutex::new(Term::stdout()));
    let pink = Style::new().color256(225);
    let brown = Style::new().color256(136);

    let petal = "*";

    let foliage = r"
        （
    （、、             ））
        \_           __ゞ )
    ゞ ,(、、|| ;| / /  ノ
        (、、  ,)  || ﾉ  ))
        ゞ  ,r || |,,ノ~";

    let trunk = r"
            〉 / ヾ |
            | ゝﾉ/  |
            |  || 〉|
            |,|   〉|
            ﾉ |  /\ |
            〉ﾉ  ||  ヾﾞ
            ノし、し ヽ ﾞ
";

    let x = 5;
    let foliage_height = foliage.lines().count() as i32;
    let trunk_height = trunk.lines().count() as i32;

    let y = HEIGHT - trunk_height - foliage_height;

    let mut rng = rand::thread_rng();

    let petal_start_x = x + 23;
    let petal_start_y = y + 3;

    let mut petals: Vec<(i32, i32)> = Vec::with_capacity(PETAL_COUNT);

    term.lock().unwrap().hide_cursor().unwrap();

    term.lock().unwrap().clear_screen().unwrap();
    let foliage_pos_y = y;
    let trunk_pos_y = y + foliage_height - 1;
    term.lock().unwrap().move_cursor_to(x as usize, foliage_pos_y as usize).unwrap();
    print!("{}", pink.apply_to(foliage));
    term.lock().unwrap().move_cursor_to(x as usize, trunk_pos_y as usize).unwrap();
    print!("{}", brown.apply_to(trunk));

    let mut last_generation_time = Instant::now();

    let term_clone = Arc::clone(&term);
    ctrlc::set_handler(move || {
        term_clone.lock().unwrap().clear_screen().unwrap();
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    loop {
        for &(px, py) in &petals {
            if (0..WIDTH).contains(&px) && (0..HEIGHT).contains(&py) {
                term.lock().unwrap().move_cursor_to(px as usize, py as usize).unwrap();
                print!(" ");
            }
        }

        petals.retain(|&(_px, py)| py < HEIGHT);

        for petal in petals.iter_mut() {
            petal.1 += 1;
            petal.0 += rng.gen_range(1..=10);

            if petal.1 >= HEIGHT {
                petal.0 = petal_start_x;
                petal.1 = petal_start_y;
            }
        }

        if last_generation_time.elapsed() >= Duration::from_millis(PETAL_GENERATION_INTERVAL_MS) {
            if petals.len() < PETAL_COUNT {
                let new_petal = (
                    rng.gen_range(petal_start_x..petal_start_x + 7),
                    petal_start_y,
                );
                petals.push(new_petal);
            }
            last_generation_time = Instant::now();
        }

        for &(px, py) in &petals {
            if (0..WIDTH).contains(&px) && (0..HEIGHT).contains(&py) {
                term.lock().unwrap().move_cursor_to(px as usize, py as usize).unwrap();
                print!("{}", pink.apply_to(petal));
            }
        }

        term.lock().unwrap().flush().unwrap();

        sleep(Duration::from_millis(200));
    }
}

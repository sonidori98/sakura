use console::{Style, Term};
use crossterm::terminal::size;
use rand::Rng;
use core::panic;
use std::{
    sync::{Arc, Mutex, OnceLock},
    thread::sleep,
    time::{Duration, Instant},
};

// const WIDTH: i32 = 100;
// const HEIGHT: i32 = 30;
static WIDTH: OnceLock<i32> = OnceLock::new();
static HEIGHT: OnceLock<i32> = OnceLock::new();
const PETAL_COUNT: usize = 25;
const PETAL_GENERATION_INTERVAL_MS: u64 = 350;
pub const ONCE_ERR: &str = "Value should be set";

fn main() {
    let term = Arc::new(Mutex::new(Term::stdout()));
    let pink = Style::new().color256(225);
    let brown = Style::new().color256(136);
    get_size();

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

    let y = *HEIGHT.get().expect(ONCE_ERR) - trunk_height - foliage_height;

    let mut rng = rand::thread_rng();

    let petal_start_x = x + 23;
    let petal_start_y = y + 3;

    let mut petals: Vec<(i32, i32)> = Vec::with_capacity(PETAL_COUNT);

    term.lock().unwrap().hide_cursor().unwrap();

    term.lock().unwrap().clear_screen().unwrap();
    let foliage_pos_y = y;
    let trunk_pos_y = y + foliage_height - 1;
    term.lock()
        .unwrap()
        .move_cursor_to(x as usize, foliage_pos_y as usize)
        .unwrap();
    print!("{}", pink.apply_to(foliage));
    term.lock()
        .unwrap()
        .move_cursor_to(x as usize, trunk_pos_y as usize)
        .unwrap();
    print!("{}", brown.apply_to(trunk));

    let mut last_generation_time = Instant::now();

    let term_clone = Arc::clone(&term);
    ctrlc::set_handler(move || {
        term_clone.lock().unwrap().clear_screen().unwrap();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        for &(px, py) in &petals {
            if (0..*WIDTH.get().expect(ONCE_ERR)).contains(&px) && (0..*HEIGHT.get().expect(ONCE_ERR)).contains(&py) {
                term.lock()
                    .unwrap()
                    .move_cursor_to(px as usize, py as usize)
                    .unwrap();
                print!(" ");
            }
        }

        petals.retain(|&(_px, py)| py < *HEIGHT.get().expect(ONCE_ERR));

        for petal in petals.iter_mut() {
            petal.1 += 1;
            petal.0 += rng.gen_range(1..=10);

            if petal.1 >= *HEIGHT.get().expect(ONCE_ERR) {
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
            if (0..*WIDTH.get().expect(ONCE_ERR)).contains(&px) && (0..*HEIGHT.get().expect(ONCE_ERR)).contains(&py) {
                term.lock()
                    .unwrap()
                    .move_cursor_to(px as usize, py as usize)
                    .unwrap();
                print!("{}", pink.apply_to(petal));
            }
        }

        term.lock().unwrap().flush().unwrap();

        sleep(Duration::from_millis(200));
    }
}

fn get_size() {
    match size() {
        Ok((col, row)) => {
            HEIGHT.set(col.into()).unwrap();
            WIDTH.set(row.into()).unwrap();
        }
        Err(e) => {
            eprintln!("Failed to get terminal size: {}", e);
            panic!();
        }
    }
}

use std::{thread::sleep, time::Duration};

use console::{Style, Term};

fn main() {
    let term = Term::stdout();
    let pink = Style::new().color256(225);
    let blown = Style::new().color256(136);

    let foliage = r"
        （
    （、、             ））
        \_           __ゞ )
    ゞ ,(、、|| ;| / /  ノ
        (、、  ,)  || ﾉ  ))
        ゞ  ,r || |,,ノ~
    ";
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
    let y = 10;

    let mut buffer = String::new();
    term.clear_screen().unwrap();

    loop {
        buffer.clear();
        buffer.push_str(&format!("{}{}", pink.apply_to(foliage), blown.apply_to(trunk)));
        term.move_cursor_to(x, y).unwrap();
        print!("{}", buffer);

        sleep(Duration::from_millis(500));
    }
}

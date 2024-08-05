use std::{thread::sleep, time::Duration};
use console::{Style, Term};
use rand::Rng;

const WIDTH: i32 = 80;  // ターミナルの幅
const HEIGHT: i32 = 24;  // ターミナルの高さ
const PETAL_COUNT: usize = 20;

fn main() {
    let term = Term::stdout();
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
    
    // 幹の位置を基準に設定
    let y = HEIGHT - trunk_height - foliage_height;  // 隙間をなくすために調整

    let mut rng = rand::thread_rng();
    let mut petals: Vec<(i32, i32)> = (0..PETAL_COUNT)
        .map(|_| (rng.gen_range(x + 20..x + 30), rng.gen_range(y..y + foliage_height))) // 花びらの初期位置を葉の上端に設定
        .collect();

    term.hide_cursor().unwrap();

    // 最初に木を描画
    term.clear_screen().unwrap();
    let foliage_pos_y = y;
    let trunk_pos_y = y + foliage_height; // 葉っぱのすぐ下に幹を配置
    term.move_cursor_to(x as usize, foliage_pos_y as usize).unwrap();
    print!("{}", pink.apply_to(foliage));
    term.move_cursor_to(x as usize, trunk_pos_y as usize).unwrap();
    print!("{}", brown.apply_to(trunk));

    loop {
        // 花びらのみをクリア
        for &(px, py) in &petals {
            if px >= x + 20 && px < x + 30 && (0..HEIGHT).contains(&py) {
                term.move_cursor_to(px as usize, py as usize).unwrap();
                print!(" ");  // 空白で花びらを消す
            }
        }

        // 花びらの位置を更新
        for petal in petals.iter_mut() {
            petal.1 += 1; // 花びらを下に移動
            if petal.1 >= y + 20 { // 花びらが幹の下端に到達したら、画面の上部に再配置
                petal.1 = y; // 初期位置に戻す
                petal.0 = rng.gen_range(x + 20..x + 30);
            }
        }

        // 花びらを再描画
        for &(px, py) in &petals {
            if px >= x + 20 && px < x + 30 && (0..HEIGHT).contains(&py) {
                term.move_cursor_to(px as usize, py as usize).unwrap();
                print!("{}", pink.apply_to(petal));
            }
        }

        // 画面全体をバッファから一度に描画
        term.flush().unwrap();

        sleep(Duration::from_millis(200)); // アニメーション速度調整
    }
}

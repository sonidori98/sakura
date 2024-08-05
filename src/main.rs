use std::{thread::sleep, time::Duration};
use console::{Style, Term};
use rand::Rng;

const WIDTH: i32 = 100;  // ターミナルの幅
const HEIGHT: i32 = 24;  // ターミナルの高さ
const PETAL_COUNT: usize = 50;

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

    let petal_start_x = x + 23;
    let petal_start_y = y + 3; // 花びらの生成位置を調整

    let mut petals: Vec<(i32, i32)> = (0..PETAL_COUNT)
        .map(|_| {
            let px = rng.gen_range(petal_start_x..petal_start_x + 7); // 木の上部から横方向に設定
            let py = rng.gen_range(petal_start_y..petal_start_y + 7); // 木の上部から設定
            (px, py)
        })
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
            if (0..WIDTH).contains(&px) && (0..HEIGHT).contains(&py) {
                term.move_cursor_to(px as usize, py as usize).unwrap();
                print!(" ");  // 空白で花びらを消す
            }
        }

        // 花びらの位置を更新
        for petal in petals.iter_mut() {
            petal.1 += 1; // 花びらを下に移動
            petal.0 += rng.gen_range(0..=5); // 横方向に移動

            // 縦方向の移動制限
            if petal.1 >= HEIGHT { // 画面の下端に到達したら上端に戻す
                petal.0 = petal_start_x;
                petal.1 = petal_start_y; // 木の上部よりさらに下に戻す
            }
        }

        // 花びらを再描画
        for &(px, py) in &petals {
            if (0..WIDTH).contains(&px) && (0..HEIGHT).contains(&py) {
                term.move_cursor_to(px as usize, py as usize).unwrap();
                print!("{}", pink.apply_to(petal));
            }
        }

        // 画面全体をバッファから一度に描画
        term.flush().unwrap();

        sleep(Duration::from_millis(200)); // アニメーション速度調整
    }
}

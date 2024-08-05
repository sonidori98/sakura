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

    // let tree = r"
    //        （
    //         （、、             ））
    //           \_           __ゞ )
    //         ゞ ,(、、|| ;| / /  ノ
    //          (、、  ,)  || ﾉ  ))
    //             ゞ  ,r || |,,ノ~
    //                〉 / ヾ |
    //                | ゝﾉ/  |
    //                |  || 〉|
    //                |,|   〉|
    //                ﾉ |  /\ |
    //                〉ﾉ  ||  ヾﾞ
    //                ノし、し ヽ ﾞ
    // ";

    println!("{}{}", pink.apply_to(foliage), blown.apply_to(trunk));
    term.clear_line().unwrap();
}

/// このライフゲームは周辺の状態を利用して自身の状態を書き換えるタイプのライフゲーム
/// となります。そのためエージェントが移動するタイプのライフゲームには使えません。
///

extern crate life_game;

use life_game::game::stage;
use life_game::game::stage::Stage;

// 生存時間定数の定義
//（できればサイズなどの初期化パラメータや時間などのパラメータはオプションで指定できるようにしたい）


fn main() {
    let TIME = 4;
    // 初期状態を生成して保持状態に（例：newで呼び出したあとにrandomに渡すことでランダム化）
    let mut stage = stage::Stage::make_stage(false, 4, 4);
    stage.set_data(&1, &1, &true);
    stage.set_data(&1, &2, &true);
    stage.set_data(&2, &1, &true);
    stage.set_data(&2, &2, &true);

    // 初期状態を画像として書き出す（例：lifegame_0.png）
    println!("{}", stage.to_string());

    // 生存時間の間だけwhileループ
    for t in 1 .. TIME+1 {
        // 前回の状態をコピーして用意

        // 次のステージを計算して結果を更新
        stage = stage.calc_stage(nbh_rule, calc_rule);

        // 保持状態を画像として書き出す
        println!("{}", stage.to_string());
    }

    // 画像をGifに変換
}

fn nbh_rule(s: &Stage<bool>, i: usize, j: usize) -> Vec<(usize, usize)>{
    let (h, w) = (*s).get_sizes();
    return vec![
        ((i+h-1)%h, (j+w-1)%w), ((i+h-1)%h, j), ((i+h-1)%h, (j+1)%w),
        (i, (j+w-1)%w), (i, (j+1)%w),
        ((i+1)%h, (j+w-1)%w), ((i+1)%h, j), ((i+1)%h, (j+1)%w)
    ];
}

fn calc_rule(s: &Stage<bool>, i: usize, j: usize, data: bool, nbh: Vec<(usize, usize)>) -> bool{
    let v: Vec<_> = nbh.iter().map(|index| {*(*s).get_data(&index.0, &index.1)}).filter(|t| *t).collect();
    let c = v.len();
    // 生きているセルをtrueとして扱うことにする
    return if data{
        if c==2 || c==3 {true}else {false}
    }else{
        if c==3 {true}else{false}
    };
}

/// このライフゲームは周辺の状態を利用して自身の状態を書き換えるタイプのライフゲーム
/// となります。そのためエージェントが移動するタイプのライフゲームには使えません。
///

extern crate life_game;

use life_game::game::stage;
use life_game::game::stage::Stage;

// 生存時間定数の定義
//（できればサイズなどの初期化パラメータや時間などのパラメータはオプションで指定できるようにしたい）


fn main() {
    let TIME = 8;
    // 初期状態を生成して保持状態に（例：newで呼び出したあとにrandomに渡すことでランダム化）
    let mut stage = stage::Stage::make_stage(0 as u8, 8, 10);
    // 0000000000
    // 0000000000
    // 0000100000
    // 0000011000
    // 0001100000
    // 0000010000
    // 0000000000
    // 0000000000
    stage.set_data(&2, &4, &1);
    stage.set_data(&3, &5, &1);
    stage.set_data(&3, &6, &1);
    stage.set_data(&4, &3, &1);
    stage.set_data(&4, &4, &1);
    stage.set_data(&5, &5, &1);

    // 初期状態を画像として書き出す（例：lifegame_0.png）
    println!("{}", stage.to_string());

    // 生存時間の間だけwhileループ
    for t in 1 .. TIME+1 {
        // 前回の状態をコピーして用意

        // 次のステージを計算して結果を更新
        stage = stage.calc_stage(nbh_rule, calc_rule);

        // 保持状態を画像（今は文字列）として書き出す
        println!("{}", stage.to_string());
    }

    // 画像をGifに変換
}

fn nbh_rule(s: &Stage<u8>, i: usize, j: usize) -> Vec<(usize, usize)>{
    let (h, w) = (*s).get_sizes();
    // トーラス型のライフゲーム
    let v = vec![
        ((i+h-1)%h, (j+w-1)%w), ((i+h-1)%h, j), ((i+h-1)%h, (j+1)%w),
        (i, (j+w-1)%w), (i, (j+1)%w),
        ((i+1)%h, (j+w-1)%w), ((i+1)%h, j), ((i+1)%h, (j+1)%w)
    ];
    // println!("{:?}", v);
    return v;
}

fn calc_rule(s: &Stage<u8>, i: usize, j: usize, data: u8, nbh: Vec<(usize, usize)>) -> u8{
    let v: Vec<_> = nbh.iter().map(|index| {*(*s).get_data(&index.0, &index.1)}).filter(|t| *t==1).collect();
    let c = v.len();
    // 生きているセルを1として扱うことにする
    return if data==1{
        if c==2 || c==3 {1}else {0}
    }else{
        if c==3 {1}else{0}
    };
}

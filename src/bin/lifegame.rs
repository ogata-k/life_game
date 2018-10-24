extern crate life_game;

use life_game::game::status;
use life_game::game::stage;
use life_game::game::next;

// 生存時間定数の定義
//（できればサイズなどの初期化パラメータや時間などのパラメータはオプションで指定できるようにしたい）


fn main() {
    let TIME = 100;
    // 初期状態を生成して保持状態に（例：newで呼び出したあとにrandomに渡すことでランダム化）
    let mut state = stage::Stage::make_stage(false, 300, 400);

    // 初期状態を画像として書き出す（例：lifegame_0.png）
    println!("{:?}", state);

    // 生存時間の間だけwhileループ
    for t in 1 .. TIME+1 {
        // 前回の状態をコピーして用意
        let mut new_state = state.copy();

        /*
        // イテレータからデータ情報と番号を取得
        for (ref i, ref j, ref s) in state{
            // スライスの各データ毎に
            // データと番号から周辺のデータ情報を取得
            let nbh: [u16] = &state.get_nhd(&i);
            // データ情報とそのデータ情報から次の状態を計算
            let data = calc_status(&state, nbh, &i);
            // コピーして用意しておいた情報に結果を保存
            new_state.set_data(&i, data);
        }
        */
        // 結果を保持状態に保持
        state = new_state.copy();

        // 保持状態を画像として書き出す
        println!("{:?}", state);
    }

    // 画像をGifに変換
}

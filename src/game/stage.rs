use std::clone::Clone;
/// ライフゲームのステージの状態を保持する構造体。
/// 今は生存データも保持しているがゆくゆくはステージ状態、生態状態を別で保持する
#[derive(Clone, Debug, PartialEq)]
pub struct Stage<T: Clone + Copy> {
    stage: Vec<T>,
    sizes: (usize, usize),  // height, width
    d_state: T
}

impl<T: Clone + Copy + PartialEq>Stage<T> {
    /// ステージ全体のイテレーターの作成
    pub fn iterate(&self) -> impl Iterator<Item=(usize, usize, &T)> {
       return  (0 .. (*self).sizes.0 * (*self).sizes.1).map(move |l|
               (l / (*self).sizes.1, 
                l % (*self).sizes.1, 
                (*self).get_data(&(l / (*self).sizes.1), &(l % (*self).sizes.1))));
    }

    /// 初期ステージ作成関数
    pub fn make_stage(d_state: T, height: usize, width: usize) -> Self {
        return Stage {
            stage: vec![d_state; height * width],
            sizes: (height, width),
            d_state: d_state
        };
    }

    /// ステージ複製関数。後に非公開にするかも。
    fn copy(&self) -> Self {
        let (w, h) = ((*self).sizes.0, (*self).sizes.1);
        let mut v = vec![(*self).d_state; w * h];
        for i in 0 .. w * h {
            v[i] = (*self).stage[i];
        }
        return Stage{
            stage: v,
            sizes: (w, h),
            d_state: (*self).d_state
        };
    }

    /// サイズを取得する関数
    pub fn get_sizes(&self) -> &(usize, usize) {
        return &(*self).sizes;
    }

    /// 高さを取得する関数
    pub fn get_height(&self) -> &usize {
        return &(*self).sizes.0;
    }

    /// 横幅を取得する関数
    pub fn get_width(&self) -> &usize {
        return &(*self).sizes.1;
    }

    /// 座標からステージの位置を取得
    fn get_index(&self, i: &usize, j: &usize) -> usize {
         return ((*self).sizes.1 * *i + *j).clone();
    }

    /// 指定座標のステージの状態を更新
    pub fn set_data(&mut self, i: &usize, j: &usize, data: &T) {
        let l = (*self).get_index(i, j);
        (*self).stage[l] = *data;
    }

    /// 指定した座標からデータを取得
    pub fn get_data(&self, i: &usize, j: &usize) -> &T {
        let l = (*self).get_index(i, j);
        return &(*self).stage[l];
    }

    /// 計算規則を与える関数を引数にして新しいステージを計算する関数
    /// rule :: target_row -> target_col -> target_data -> [(nbh_row, nbh_col)] -> result
    pub fn calc_stage(&self,
            nbh_rule: fn(&Stage<T>, usize, usize) -> Vec<(usize, usize)>,
            calc_rule: fn(&Stage<T>, usize, usize, T, Vec<(usize, usize)>) -> T) -> Self{
        let mut new_stage = (*self).copy();
        for (i, j, t) in (*self).iterate() {
            let indeces = nbh_rule(self, i, j);
            let new_t = calc_rule(self, i, j, *t, indeces);
            new_stage.set_data(&i, &j, &new_t);
        }
        return new_stage;
    }
}


#[test]
fn test_iterater(){
    let mut s = Stage::make_stage(false, 2, 3);
    s.set_data(&0, &2, &true);
    s.set_data(&1, &2, &true);
    for (i, j, d) in s.iterate(){
        assert_eq!(s.get_data(&i, &j), d);
    }
}

#[test]
fn test_new_make(){
    assert_eq!(Stage::make_stage(false, 2, 3),
            Stage{
                stage: vec![false, false, false, false, false, false],
                sizes: (2, 3),
                d_state: false
            });
}

#[test]
fn test_copy(){
    let s = Stage::make_stage(true, 2, 3);
    assert_eq!(s, s.copy());
}

#[test]
fn test_set_data(){
    let mut s = Stage::make_stage(false, 2, 3);
    s.set_data(&1, &1, &true);
    assert_eq!(s.stage[1], false);
    assert_eq!(s.stage[4], true);
}

#[test]
fn test_get_data(){
    let mut s = Stage::make_stage(false, 2, 3);
    s.set_data(&1, &1, &true);
    assert_eq!(s.get_data(&1, &1), &true);
}

#[test]
fn test_calc_new_stage() {
    let mut s = Stage::make_stage(true, 3, 3);
    s.set_data(&0, &1, &false);

    fn nbh_rule(s: &Stage<bool>, i: usize, j: usize) -> Vec<(usize, usize)>{
        return vec![((i + 2) % 3, j)];
    }

    fn calc_rule(s: &Stage<bool>, i:usize, j: usize, t: bool, nbh: Vec<(usize, usize)>) -> bool{
        let index = nbh[0];
        let over = (*s).get_data(&index.0, &index.1);
        return t && *over;
    }

    s = s.calc_stage(nbh_rule, calc_rule);
    let mut c = Stage::make_stage(true, 3, 3);
    c.set_data(&0, &1, &false);
    c.set_data(&1, &1, &false);
    assert_eq!(s, c);
}

pub struct ToyVec<T> {
    elements: Box<[T]>, // T型の要素を格納する領域。各要素はヒープ領域に置かれる
    len: usize,         // ベクタの長さ（現在の要素数）
}

impl<T: Default> ToyVec<T> {
    // new はキャパシティ（容量）が0のToyVecを作る
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    // with_capacityは指定されたキャパシティを持つToyVecを作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    // T型の値がsize個格納できるBox<[T]>を返す
    pub fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size) // T型のデフォルト値をsize個作り
            .collect::<Vec<_>>() // Vec<T>に収集してから
            .into_boxed_slice() // Box<[T]>に変換する
    }

    // ベクタの長さを返す
    pub fn len(&self) -> usize {
        self.len
    }

    // ベクタの現在のキャパシティを返す
    pub fn capacity(&self) -> usize {
        self.elements.len() // elementsの要素数(len)がToyVecのキャパシティになる
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            // 要素を追加するスペースがないなら
            self.grow(); // もっと大きいelementsを確保して既存の要素を引っ越す
        }
        self.elements[self.len] = element; // 要素を格納する（所有権がムーブする）
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // インデックスが範囲内なら
            Some(&self.elements[index]) // Some(不変の参照)を返す
        } else {
            None // 範囲外ならNoneを返す
        }
    }

    fn grow(&mut self) {
        /* 本体は省略 */
        // self.capacityが0の時は長さ1のBox<[T]>を作成しself.elementsにセットする
        // self.capacityが1以上の時は現在の2倍の長さのBox<[T]>を作成しself.elementsにセットする。
        //   既存の全要素を新しいBox<[T]>にムーブしたあと、古いBox<[T]>を破棄する

        if self.capacity() == 0 {
            // 1要素分の領域を確保する
            self.elements = Self::allocate_in_heap(1);
        } else {
            // 現在の2倍の領域を確保する
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // self.elementsを置き換える
            let old_elements = std::mem::replace(&mut self.elements, new_elements);

            // ダメな例
            // let old_elements = self.elements
            // self.elements = Self::allocate_in_heap(self.capacity() * 2);
            // これだと self が &mut なので elements のBox<[T]>の所有権が奪えない

            // 既存の全要素を新しい領域へムーブする
            // Vec<T>のinto_iter(self)なら要素の所有権が得られる
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
            // Box<[T]>::into_vecとVec<T>::into_iterはデータをコピーせずにその場で変換してくれる
            // 効率の良い実装になっている
        }
    }

    // 説明のためにライフタイムを明示しているが省略可
    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements, // Iter構造体の定義より、ライフタイムは'vecになる
            len: self.len,
            pos: 0,
        }
    }
}
pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>, // ToyVec構造体のelementsを指す不変の参照
    len: usize,               // ToyVecの長さ
    pos: usize,               // 次に返す要素のインデックス
}
impl<'vec, T> Iterator for Iter<'vec, T> {
    // 関連型（トレイトに関連付いた型）で、このイテレータがイテレートする要素の型を指定する
    // 関連型は8章で説明
    type Item = &'vec T;

    // nextメソッドは次の要素を返す
    // 要素があるなら不変の参照（&T）をSomeで包んで返し、無いときはNoneを返す
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

// IntoIteratorトレイトを実装するとfor式での繰り返しができるようになる
impl<'vec, T: Default> IntoIterator for &'vec ToyVec<T> {
    type Item = &'vec T; // イテレータがイテレートする値の型
    type IntoIter = Iter<'vec, T>; // into_iterメソッドの戻り値の型

    // &ToyVec<T>に対するトレイト実装なので、selfの型はToyVec<T>ではなく&ToyVec<T>
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
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
        std::iter::repeat_with(Default::default())
            .take(size) // T型のデフォルト値をsize個作り
            .collect::<Vec<_>>() // Vec<T>に収集してから
            .into_boxed_slice() // Box<[T]>に変換する
    }

    // ベクタの長さを返す
    pub fn len(&self) -> usize {
        self.len()
    }

    // ベクタの現在のキャパシティを返す
    pub fn capacity(&self) -> usize {
        self.elements.len() // elementsの要素数(len)がToyVecのキャパシティになる
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() { // 要素を追加するスペースがないなら
            self.grow(); // もっと大きいelementsを確保して既存の要素を引っ越す
        }
        self.elements[self.len] = element; // 要素を格納する（所有権がムーブする）
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len { // インデックスが範囲内なら
            Some(&self.elements[index]) // Some(不変の参照)を返す
        } else {
            None // 範囲外ならNoneを返す
        }
    }

    fn grow(&mut self) {
        /* 本体は省略 */
    }
}
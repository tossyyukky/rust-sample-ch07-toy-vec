pub struct ToyVec<T> {
    elements: Box<[T]>, // T型の要素を格納する領域。各要素はヒープ領域に置かれる
    len: usize,         // ベクタの長さ（現在の要素数）
}
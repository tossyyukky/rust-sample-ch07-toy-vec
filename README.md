# toy-vec

https://gihyo.jp/book/2019/978-4-297-10559-4
ch07-9 簡単なベクタの実装

# summary

- ベクタの要素は連続したメモリ領域に格納する。この領域（elementsと呼ぶことにします）はヒープ領域に確保する。
- pushメソッドで要素を追加できる。elementsの容量（キャパシティ）を超えたら現在の2倍のキャパシティをもつ領域を確保し直す。
- getメソッドによる要素の借用に加えて、popメソッドによる要素の取り出し（所有権のムーブアウト）もサポートする。
- イテレータをサポートする。
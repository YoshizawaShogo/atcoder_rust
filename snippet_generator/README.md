# 準備

``` bash
    $ cargo install cargo-snippet --features="binaries"
```

# 使用方法

``` bash
    $ cargo snippet -t vscode > ../template/.vscode/generated.code-snippets
```

# 次の目標
* 関数の単体testを作成
* 尺取り法

# 注意点
* テストしていません、動くかどうか知りません。
* 関数化していない部分があるため、このリポジトリでcargo buildが通りません。
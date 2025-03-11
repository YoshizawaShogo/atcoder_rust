# 使用方法
``` sh
$ source env.fish
$ atcoder_new abc123 # Function atcoder is defined at env.fish.
$ cargo atcoder test a
$ cargo atcoder submit a [-f]
```

# 初回準備
``` sh
$ cargo install cargo-atcoder
$ rustup target add x86_64-unknown-linux-musl
$ cargo atcoder login
```
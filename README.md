# lava

## Description
Javaのコンパイルと実行を一度にしたいから作った(;o;)

## Build
以下を実行すると，成果物が`target/release/lava`に生成される．
```
cargo build --release
```

## Examples

- ClassAをコンパイル，実行する（依存関係のコンパイルはjavacにより行われる）
```
lava ClassA.java
```

- `*.java`に一致するファイルをコンパイルし，ClassAを実行する
```
lava *.java ClassA.java
```

- 実行とコンパイルにオプションを指定する
```
lava -cp class/path/ ClassA.java
```

- 実行とコンパイルにオプションを指定し，`*.java`に一致するファイルをコンパイルし，ClassAを実行する
```
lava -cp class/path/ */*.java ClassA.java
```
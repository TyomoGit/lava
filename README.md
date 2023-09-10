# lava

## Description
Javaのコンパイルと実行を一度にしたいから作った(;o;)

## Install
`cargo`が必要．
```sh
cargo install --git https://github.com/TyomoGit/lava.git
```

## Usage

```sh
lava [options] [source files] <main file> [args...]
```
|||
|:---|:---|
|`options`|javacとjavaに指定するオプション<br>（--classpathなど）|
|`source files`|コンパイルするファイル|
|`main file`|Main Classが記述されているファイル<br>（.javaも含めて書く）|
|`args...`|javaアプリケーションに指定する引数|


## Examples
zshで実行

- ClassAをコンパイル，実行する
    - 依存関係のコンパイルはjavacにより行われる
    - ClassA.main(String[])が実行される
```sh
lava ClassA.java
```

- 実行とコンパイルにオプションを指定する
```sh
lava -cp class/path/ ClassA.java
```

- `*/*.java`に一致するファイルをコンパイルし，ClassAを実行する
```sh
lava */*.java ClassA.java
```

- 実行とコンパイルにオプションを指定し，`My*.java`に一致するファイルをコンパイルし，ClassAを実行する
```sh
lava -cp class/path/ My*.java MyMainClass.java
```
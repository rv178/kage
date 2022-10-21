# Kage

Chess engine written in Rust.

### Features

-	FEN string parser
- 	BitBoard representation
-   Attack generation for pawns, knights and the king
-   Sliding piece attack generation with hyperbola quintessence
-	Nothing else (as of now)

### Aim

- 	Make a decent (preferrably strong) UCI compatible chess engine

I'm currently reading the [chess programming wiki](https://www.chessprogramming.org/) to learn more about engine development.

### Compiling

You can compile it using `cargo` or install [baker](https://github.com/rv178/baker) and compile it like this:

```
bake setup
bake
```

A binary will be copied to `./bin/kage`

### Usage

#### FEN string parsing

```
kage --fen "<FEN_STRING>"
```

#### Note: this project is a WIP

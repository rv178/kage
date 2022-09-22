# Cranium

Chess engine written in Rust.

### Features

-	FEN string parser
- 	bitboard representation
-	nothing else

### Aim

- 	Make a decent (preferrably strong) UCI compatible chess engine
- 	Negamax search with alpha-beta pruning
- 	Search with depth

I'm currently reading the [chess programming wiki](https://www.chessprogramming.org/) to learn more about engine development.

### Compiling

You can compile it using `cargo` or install [baker](https://github.com/rv178/baker) and compile it like this:

```
bake setup
bake
```

A binary will be copied to `./bin/cranium`

### Usage

#### FEN string parsing

```
cranium --fen "<FEN_STRING>"
```

#### Note: this project is a WIP

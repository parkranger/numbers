# NumbeRS

![Rust](https://github.com/parkranger/numbers/workflows/Rust/badge.svg?branch=master)

Playground to do some number processing with Rust.

## Prime Factorization

Primfaktorzerlegung

```
$ numbers.exe pfz 7676
Primfaktorzerlegung
 pfz(7676) = [2, 2, 19, 101]
```

## Greatest Common Divisor (GCD)

größter gemeinsamer Teiler (ggT)

```
$ numbers.exe ggt 1111 143
 ggT(1111, 143) = 11
$ numbers.exe ggt 1111 143
 ggT(111, 143) = teilerfremd
```

## Least Common Multiple (LCM)

kleinstes gemeinsames Vielfaches (kgV)

```
$ numbers.exe kgv 6 40
 kgV(6, 40) = 120
```

---

Run with logging enabled:

```
$ env RUST_LOG=numbers=info cargo run -- pfz 12
$ env RUST_LOG=info cargo run -- pfz 12
```

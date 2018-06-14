# Rust

Quick Rust overview

---

# History

* 2006 personal project 
    * probably named after [fungus](https://www.reddit.com/r/rust/comments/27jvdt/internet_archaeology_the_definitive_endall_source/)
* Mozilla sponsored in 2009 
* 1.0 in 2012

---

# Overview

* Curly-bracket languages

features:
  * zero-cost abstractions
  * move semantics
  * guaranteed memory safety
  * threads without data races
  * trait-based generics
  * pattern matching
  * type inference
  * minimal runtime
  * efficient C bindings
 

---

# Overview

---?code=code/overview.rs?lang=rs

[link to playground](https://play.rust-lang.org/?gist=d5ef143fc2207389686c92aa49e96a65&version=stable&mode=debug)

---

"fast, reliable, productive — pick three"

---

# Fast

* based on [llvm](https://llvm.org/)
* performance on par with c/c++ 
* no [GC](https://en.wikipedia.org/wiki/Garbage_collection_%28computer_science%29):
    * tight memory consumption
    * predictive performance

---

# Reliable

---

# Reliable

 * @fa[ban red] uninitialized variable @fa[ban red]

It's impossible not to initialize a variable.

---

# Reliable

 * @fa[ban red] NullPointerException @fa[ban red]

Enums to the rescue!

```rust
enum Option<T> {
    Some(T),
    None,
}
```

---

# Reliable

## Enums

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Adamantium(usize),
}

fn value_in_cents(coin: Coin) -> usize {
    match coin {
        Coin::Penny => 1,
        Coin::Adamantium(size) => 25 * size,
        _ => 3,
    }
}
```

[link to playground](https://play.rust-lang.org/?gist=3dcb04eab58dff65cd4760aee0388f01&version=stable&mode=debug)

---

# Reliable

## No Exceptions

```
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

---?code=code/error_handling.rs?lang=rs

[link to playground](https://play.rust-lang.org/?gist=170beaeafa5f7c0cf8e009a2c821c167&version=stable&mode=debug)

---

# Reliable

 * @fa[ban red] race condition @fa[ban red]
 * @fa[ban red] use after free @fa[ban red]
 * @fa[ban red] double free @fa[ban red]

Here come's the borrow checker!

Statically analyse the code to ensure the memory safety and ownership model.

---

# Reliable

## Ownership 

[move by default](https://play.rust-lang.org/?gist=d9aa3dda0a69370d405deefa366ee974&version=stable&mode=debug)

[scoped variables](https://doc.rust-lang.org/book/second-edition/ch04-01-what-is-ownership.html)

---

# Reliable

## Borrow Checker
"1, 2, 3, soleil !"

[Only at most one mutable reference](https://doc.rust-lang.org/book/second-edition/ch04-02-references-and-borrowing.html)

---

# Productive

* nice build errors
* cargo
* iters
* macros

---

# Conclusion

* @fa[frown faa-failing animated red] steep lurning curve
* @fa[frown faa-pulse animated red] borrow checker 
* @fa[heart faa-failing animated red] borrow checker 


# Rust

Quick Rust overview

@snap[west splitscreen]
<br>
![Image-Absolute](https://www.rust-lang.org/logos/rust-logo-128x128.png)
<br>
@snapend

---?image=assets/PPrust.png&size=auto 70%

@snap[east splitscreen]
From [David Marino](http://leftoversalad.com/c/015_programmingpeople/)
@snapend

---

# History

* 2006 personal project 
* Mozilla sponsored in 2009 
* 1.0 in 2015

---

  * curly-bracket languages
  * statically typed 

---

### features 

  * zero-cost abstractions
  * guaranteed memory safety
  * threads without data races

---

### features

  * move semantics
  * trait-based generics
  * pattern matching
  * type inference
  * sane default (interior immutability, move, ...)
  * efficient C bindings

---

### What it has not

 * exceptions
 * inheritance
 * function overloading
 * templates as powerful as in c++

---

# Overview

---?gist=https://gist.github.com/rust-play/0a67014cca5764727f8ae0d54befdd84

[link to playground](https://play.rust-lang.org/?gist=0a67014cca5764727f8ae0d54befdd84&version=stable&mode=debug&edition=2015)

---

"fast, reliable, productive — pick three"

---

# Fast

 * based on [llvm](https://llvm.org/)
 * performance on par with c/c++ 
 * no [GC](https://en.wikipedia.org/wiki/Garbage_collection_%28computer_science%29):
     * tight memory consumption
     * predictive performance
 * no runtime (other language integration and low level programming)

---

# Reliable

---

 * @fa[ban red] uninitialized variable @fa[ban red]

It's impossible not to initialize a variable.

---

 * @fa[ban red] NullPointerException @fa[ban red]

Enums to the rescue!

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Force to think ahead whether something can be null or not.

```rust
fn contains(&self, val: u32) -> bool

fn get(&self, val: u32) -> Option<String>
```

---

## No Exceptions

```
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```
---?gist=https://gist.github.com/rust-play/7df643b8c306b6225d6f48f42b26f07c


[link to playground](https://play.rust-lang.org/?gist=170beaeafa5f7c0cf8e009a2c821c167&version=stable&mode=debug)

---

 * @fa[ban red] data race @fa[ban red]
 * @fa[ban red] use after free @fa[ban red]
 * @fa[ban red] double free @fa[ban red]

Here come's the borrow checker!

Statically analyse the code to ensure the memory safety and ownership model.

---

## Scope 

```
fn main() {
    {
        let x = 1;   
    }
    // println!("value: {}", x); // <--- fail because x does not exists anymore
}
```

---

## Ownership 

Move by default

---?gist=https://gist.github.com/rust-play/705ca5f380634e13b7f6bccf20676858

[link to playground](https://play.rust-lang.org/?gist=705ca5f380634e13b7f6bccf20676858&version=stable&mode=debug)


---

## Borrow 

You can give references too

---?gist=https://gist.github.com/61bbe6d58b2c7f7d48b6cda8eeb8495b

[link to playground](https://play.rust-lang.org/?gist=61bbe6d58b2c7f7d48b6cda8eeb8495b&version=stable&mode=debug)

---

## Borrow checker

---?gist=https://gist.github.com/antoine-de/c2bf26706d1c76afc5267bf4dd5f6943

---

![Image-GIF](https://media.giphy.com/media/oe33xf3B50fsc/giphy.gif)
 
---

---?gist=https://gist.github.com/74f6e91ef9ef1bbc16a839b54606b29c

[link to playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=74f6e91ef9ef1bbc16a839b54606b29c)

## Borrow & mutability
"1, 2, 3, soleil !"

[Only at most one mutable reference](https://doc.rust-lang.org/book/second-edition/ch04-02-references-and-borrowing.html)

---?gist=https://gist.github.com/fc14539b8f1261cf899d1a640a31b65e

[link to playground](https://play.rust-lang.org/?gist=fc14539b8f1261cf899d1a640a31b65e&version=stable&mode=debug)

---

# Productive

* nice build errors
* nice tooling: cargo, rustfmt, doc, tests, ...
* iters
* macros

---

# Conclusion

- 😞 steep lurning curve
- 😞 borrow checker
- 💖 borrow checker
- 💖 refactoring is a brease
- 💖 great tooling out of the box 
- 💖 fast evolving language (Rust 2018!) 
- 😞 young ecosystem 

---

# Links

 * [rust doc](https://doc.rust-lang.org/)
 * [nice slides](http://manishearth.github.io/Presentations/Rust/)
 * [nice slides too](http://pnkfx.org/cyot/tutorial/slides/whistler_rust_intro.html)


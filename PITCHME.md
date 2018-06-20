# Rust

Quick Rust overview

@snap[west splitscreen]
<br>
![Image-Absolute](https://www.rust-lang.org/logos/rust-logo-128x128.png)
<br>
@snapend

---?image=assets/PPrust.png&size=auto 70%

From [David Marino](http://leftoversalad.com/c/015_programmingpeople/)

---

# History

* 2006 personal project 
    * probably named after [fungus](https://www.reddit.com/r/rust/comments/27jvdt/internet_archaeology_the_definitive_endall_source/)
* Mozilla sponsored in 2009 
* 1.0 in 2012

---


@snap[north]
@size[1.5em](Curly-bracket languages)
@snapend

@snap[west]
@ul[](false)
  * zero-cost abstractions
  * guaranteed memory safety
  * threads without data races
@ulend
@snapend

@snap[east]
@ul[](false)
  * move semantics
  * trait-based generics
  * pattern matching
  * type inference
  * efficient C bindings
@ulend
@snapend

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

Force to think ahead whether something can be null or not.

```rust
fn contains(&self, val: u32) -> bool

fn get(&self, val: u32) -> Option<String>
```

---

# Reliable

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

# Reliable

 * @fa[ban red] data race @fa[ban red]
 * @fa[ban red] use after free @fa[ban red]
 * @fa[ban red] double free @fa[ban red]

Here come's the borrow checker!

Statically analyse the code to ensure the memory safety and ownership model.

---

# Reliable

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

# Reliable

## Ownership 

Move by default

---?gist=https://gist.github.com/rust-play/705ca5f380634e13b7f6bccf20676858

[link to playground](https://play.rust-lang.org/?gist=705ca5f380634e13b7f6bccf20676858&version=stable&mode=debug)


---

# Reliable

## Borrow 

---?gist=https://gist.github.com/61bbe6d58b2c7f7d48b6cda8eeb8495b

[link to playground](https://play.rust-lang.org/?gist=61bbe6d58b2c7f7d48b6cda8eeb8495b&version=stable&mode=debug)

---

# Reliable

## Borrow checker

---?gist=https://gist.github.com/antoine-de/c2bf26706d1c76afc5267bf4dd5f6943

![Image-GIF](https://media.giphy.com/media/oe33xf3B50fsc/giphy.gif)
 
---

# Reliable

## Borrow & mutability
"1, 2, 3, soleil !"

[Only at most one mutable reference](https://doc.rust-lang.org/book/second-edition/ch04-02-references-and-borrowing.html)

---?gist=https://gist.github.com/fc14539b8f1261cf899d1a640a31b65e

[link to playground](https://play.rust-lang.org/?gist=fc14539b8f1261cf899d1a640a31b65e&version=stable&mode=debug)

---

# Productive

* nice build errors
* cargo
* iters
* macros

---

# Conclusion

- @fa[frown fa-failing animated red](steep lurning curve)
- @fa[frown faa-pulse animated red](borrow checker)
- @fa[heart faa-falling animated ] borrow checker


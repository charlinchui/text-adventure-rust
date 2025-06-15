# DAY 1:

.unwrap() ==> [unwrap](https://burntsushi.net/unwrap/)

## Inportant info:
Parameter data types in Rust are specified with `:` similar to TypeScript. Return types are specified with `->`. You can return by using the `return` keyword or by simply not adding `;` at the end of the value you want to return.

Vectors in rust are resizable arrays. Arrays have a fixed size.

Paths in rust are relative to the folder you are running the project from. Keep that in mind.

```rust
//This might be wrong due to the borrow checker (fun :) )
fn amazing_function(number: i32) -> i32 {
    number
}
```

_**Note to self:** Learn more about borrow checker, and ownership in rust. Otherwise you will suffer._

_**Note to self 2:** Learn more about unwrap, when to use it and why, also about what the f*ck it is and how it works._

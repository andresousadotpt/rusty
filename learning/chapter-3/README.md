# Variables and Mutability

### Immutable and mutable variables
```rust
let not_mutable = 1; // immutable by default
let mut mutable = 3;
```

### Constants <br>
Can be any scope.
```rust
const CONSTANT_VARIABLE = 123; // immutable by default
```

### Shadowing <br>
We can have the create a variable with the same name. The compiler will only see the latest shadow. So we say that the first variable is shadowed by the second, in this case. <br>
Is like having a mut variable, but in this case if we want to keep the variable immutable after doing some transformations we can use shadowing. <br>
```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```
#### Result
```shell
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

Is also possible to change the data type when shadowing but not on mut.
```rust

// WORKS
let spaces = "   ";
let spaces = spaces.len()

// Gives compiler-time error
let mut spaces = "   ";
spaces = spaces.len();

// $ cargo run
//    Compiling variables v0.1.0 (file:///projects/variables)
// error[E0308]: mismatched types
//  --> src/main.rs:3:14
//   |
// 2 |     let mut spaces = "   ";
//   |                      ----- expected due to this value
// 3 |     spaces = spaces.len();
//   |              ^^^^^^^^^^^^ expected `&str`, found `usize`

// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `variables` (bin "variables") due to 1 previous error
```


# Data Types

### Scalar Types
integers, floating-point numbers, Booleans, and characters.

### Integer Types
We have signed and unsigned
| Length | Signed | Unsigned |
| --- | --- | --- |
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |


#### Integer literals
| Number literals | Example |
| --- | --- |
| Decimal | 98_222 |
| Hex | 0xff |
| Octal | 0o77 |
| Binary | 0b1111_0000 |
| Byte (u8 only) | b'A' |

#### Floating-Point types
We have either `f32` or `f64` 32-bit or 64-bit respectively.

#### Numeric Operations
addition, subtraction, multiplication, division, and remainder. <br>
Integer division truncates toward zero to the nearest integer.


#### Boolean Type
bool = true/false

#### Character Type
char - 'A' or 'Z' or '😀'


### Compound Types
Compound types can group multiple values into one type. Tuples and arrays.

#### Tuple Type
We can group together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
// A tuple is considered a single compound element. TO get individual values out of a tuple,
// we can use patern matching to destructure a tuple value like this:
// This is called destructuring
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

// We can also access it by its index
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

#### Array Type
Unlike `tuple`, every item on an `array` must be of the same type. <br>
Arrays are useful when you want your data allocated onm the stack rather than the heap or when you want to ensure you always have a fixed number of elements. <br>
An `array` isn't flexible as the `vector` type. A `vector` is a similar collection type that is allowed to grow or shrink in size. If you don't know wether you want to use an `array` or a `vector`, chances are you should use a `vector`. <br>
However, arrays are more useful when you know the number of elements will not need to change. <br>
```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```



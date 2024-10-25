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

You can specify the size of the array and the type by:
```rust
let a: [i32, 5] = [1, 2, 3, 4, 5];
```
<br>

To access is like all languages `a[0]`<br>
In case you try to access an index that does not exists, it will most likely cause a runtime error.

# Functions
Rust uses `fn` keyword to declare new functions. <br>
Rust uses `snake_case` as the conventional style for functions and variable names, with all leter lowercase. <br>

## Parameters
We can define functions to have parameters. <br>
We *must* specify the type of the parameters via this way:
```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```
<br>

## Statements and Expressions
Function bodies are made up of a series of statements optionally ending in an expression. <br>
*Statements* are instructions that perform some action and do not return a value. <br>
*Expressions* evaluate to a resultant value.<br>

When we create a variable and assign a value to it with the `let` keyword is a statement. <br>
Function definitions are also statements (calling a function is not a statement).

Since statements does not return value we cannot do like this:
```rust
fn main() {
    let x = (let y = 6);
}
```
<br>

The statement `let y = 6;` is an expression that evaluates to the value `6`. <br>
Calling a function is an expression. Calling a macro is an expression. A new scope block created with `{}` is an expression, for example:
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```
<br>

This block is an expression:
```rust
{
    let x = 3;
    x + 1
}
```
<br>
In this case evaluates to 4.


## Functions with return values
```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

If we add a `;` to the end of the expression inside the `five()` function, it will give an error. <br>
It will gives us an error, because the signature of the method `five()` says that it will return a i32, but it returned a statement.

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```
<br>

This will give a code like:
```shell
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` (bin "functions") due to 1 previous error
```



# Control Flow
`if` is basically the same (rust just don't use `()`).
`if` is an expression and because of that we can do:
```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```
<br>
The `if` expression in the above example we cannot have mismatch of types. They should be the same type.


## Repetition with Loops
There is 3 kinds of loops: `loop`, `while`, `for`

### `loop`
Infinite loop until you explicity tell it to stop. (With for example a `break`)
```rust
fn main() {
    loop {
        println!("again!");
    }
}
```


#### Returning Values from Loops
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```


#### Loop labels to Disambiguate Between Multiple Loops
Loops within loops, `break` and `continue` apply to the innermost loop at that point.<br>
You can optionally specify a `loop label` that you can then use with `break` or `continue`.<br>
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // this will break the inner loop
            }
            if count == 2 {
                break 'counting_up; // this will break the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

### `while`

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```


### `for`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

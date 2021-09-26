% Geospatial programming with Rust
% Pirmin Kalberer @implgeo
% FOSS4G, 1. October 2021

::: notes

Abstract: Rust is one of the emerging new programming languages which is well suited for geospatial applications and escpially libraries. It was originally developed by Mozilla as a long-term replacement for C++. It's a modern language with great tooling and Stack Overflow's most loved language for four years in a row. This talk gives an introduction into Rust and an overview of the current state of geospatial libraries and applications.

Description: FOSS4G has always been a conference, where geospatial develeopers meet for discussions. So in a year without the possibility of having a beer, a talk dedicated to developers could be a motivation for attending anyway.

More:
- for iteration
- Borrow checker
:::

# About me

## Language history

::: incremental
- Basic
- Assembler (6502)
- (Turbo) Pascal
- Modula
- C
- C++
- Java
:::

---

::: incremental
- Eiffel
- Perl
- Ruby
- Javascript
- Python
- Rust (2016)
:::

## Sourcepole

* Geospatial software
  (C++, Python, React, Ruby on Rails, ...)
* Creating maps with Rust!


# Rust

## Goals

* Performance
* Reliability
* Productivity

::: notes

Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.
Productivity

Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more.

:::

## History

* Originally developed at Mozilla
* Servo browser engine
* 2015: Version 1.0 
* 2021: The Rust Foundation

::: notes
founding companies: AWS, Huawei, Google, Microsoft, and Mozilla
:::

# Rust Language

## Hello World

```Rust
fn main() {
    println!("Hello world!");
}
```

## Variables

```Rust
let x = 5;
println!("The value of x is: {}", x);
x = 6; // <-- Error: error[E0384]: cannot assign twice to immutable variable `x`
println!("The value of x is: {}", x);
```
. . .

* Type inference
* Immutable by default 

## Mutable variable

```Rust
let mut x = 5;
println!("The value of x is: {}", x);
x = 6;
println!("The value of x is: {}", x);
```

## Data types

::: incremental
* u8, u16, u32, u64
* i8, i16, i32, i64
* f32, f64
* usize, isize
:::

## Primitive Types:

- Tuples: (i32, u64)
- Unit type: ()
- Booleans: bool


## Functions

```Rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    println!("area={}", area(4, 3));
}
```

## Control Flow

## `if` Expressions

```Rust
if number % 2 == 0 {
    println!("even");
} else {
    println!("odd");
}
```

## `while` Expressions

```Rust
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

## `for` loop

```Rust
for number in 1..4 {
    println!("{}!", number);
}
```

## Structs

```Rust
struct Rectangle {
    width: u32,
    height: u32,
}
```
. . .

Instantiating
```Rust
let rect = Rectangle {
    width: 3,
    height: 4,
};
```

## Methods

```Rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
. . .

Method call
```Rust
let area = rect.area();
```

## Enums

```Rust
enum ImageType {
    Png,
    Jpeg,
    Tiff,
}
```
. . .

```Rust
let image_type = ImageType::Png;
```

## Enums with data

```Rust
enum Color {
    Rgb(u8, u8, u8),
    None,
}
```
. . .

```Rust
let color = Color::Rgb(255, 0, 0);
```

## `match` control flow operator

```Rust
match color {
    Color::Rgb(r, g, b) => println!("{}/{}/{}", r, g, b),
    Color::None => println!("No Color"),
}
```

## `if let`

```Rust
if let Color::Rgb(r, g, b) = color {
    println!("{}/{}/{}", r, g, b);
}
```

## Generic data types

```Rust
struct Point<T> {
    x: T,
    y: T,
}
```
. . .

```Rust
let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
```

## Option

```Rust
enum Option<T> {
    None,
    Some(T),
}
```
. . .

```Rust
if let Some(v) = vec.pop() {
    println!("{}", v);
}
```

## Traits

```Rust
trait Shape {
    fn area(&self) -> u32;
}
```
. . .

```Rust
impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

## Closures

```Rust
let filtered = shapes
    .iter()
    .filter(|r| r.area() > 100);
```

## Multi-threading

```Rust
thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});
```

## More

* Modules
* Multi-threading
* `async`/`await`
* Error handling with `Result` type
* `unsafe`
* Macros

## Derive-Macros

```Rust
#[derive(Serialize)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

## Standard library

* Collections Vec, HashMap, ...

# Tooling

## Testing & Documentation

* Built-in Testing
* Benchmark tests
* Built-in Documentation generator

https://docs.rs/

## cargo

* ...

https://crates.io


# Geospatial Rust

## Awesome Geo Rust

https://github.com/pka/awesome-georust

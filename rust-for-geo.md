% Geospatial programming with Rust
% Pirmin Kalberer @implgeo
% FOSS4G, 1. October 2021

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

* Geospatial software:
  QGIS, QGIS Web Client and more
* Developing in Python, C++, Javascript, Ruby
  and also Rust:
  * t-rex vector tile server


# Rust

## Why Rust?

::: incremental
* Performance
  - no runtime or garbage collector
  - low-level control
* Reliability
  - Rich type system
  - Ownership model guarantees memory-safety and thread-safety
* Productivity
  - Great documentation and tooling
  - Friendly compiler
:::

::: notes

Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.

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

* u8, u16, u32, u64
* i8, i16, i32, i64
* f32, f64
* usize, isize
* bool
* Characters, Tuples, Arrays

::: notes
- primary scalar types: integers, floating-point numbers, Booleans, and characters
- primitive compound types: tuples and arrays
:::

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

::: notes
example uses value range, could also be an iterator,...
:::

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
    Transparent,
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
    Color::Transparent => println!("No Color"),
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
for i in (1..10).filter(|&x| x % 2 == 0) {
    println!("{}", i);
}
```

::: notes
Functional Language Features: Iterators and Closures
Closures: Anonymous Functions that Can Capture Their Environment
:::

## More

* Modules
* Multi-threading
* `async`/`await`
* Error handling with `Result` type
* `unsafe`
* Efficient C bindings
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

* Primitive types
* String, Arc, Mutex, ...
* Container and collections
  - (Vec, HashMap, Iterator, ...)
* I/O: Files, UDP/TCP
* Multithreading
* Macros: println, assert, ...

# Tooling

## Testing & Documentation

* Built-in Testing
* Benchmark tests
* Built-in documentation generator

<https://docs.rs/>

## Cargo package manager

* Manage dependencies
* Format code
* Compile code
* Run application
* Run tests
* Create documentation
* Create packages for publishing
* Extensible

<https://crates.io>


# Geospatial Rust

## Geospatial primitives and algorithms

* [Geo](https://github.com/georust/geo) - Geospatial primitives such as Point & LineString, and algorithms such as distance, convex hull, centroidcalculations.
* [spade](https://github.com/Stoeoef/spade) - Spatial datastructures like r-trees and delaunay triangulations for Rust.
* [geographiclib-rs](https://github.com/georust/geographiclib-rs) - A port of geographiclib (geodesic calculations).

## Geospatial primitives and algorithms (bindings)

* [GEOS](https://github.com/georust/geos) - Bindings for the Geometry Engine - Open Source (GEOS) library.
* [PROJ](https://github.com/georust/proj) - Bindings for the PROJ library for coordinate transformation and projections.

## Handling GIS data formats

* [GeoZero](https://github.com/georust/geozero) - Zero-Copy reading and writing of geospatial data.
* [GeoJSON](https://github.com/georust/geojson) - Work with GeoJSON files.
* [GeoTIFF](https://github.com/georust/geotiff) - Work with GeoTIFF raster files.
* [image-tiff](https://github.com/image-rs/image-tiff) - TIFF decoding and encoding library in pure Rust.
* [FlatGeobuf](https://github.com/flatgeobuf/flatgeobuf) - A performant binary encoding for geographic data based on flatbuffers.
* [las-rs](https://github.com/gadomski/las-rs) - Read and write ASPRS las files.

## Handling GIS data formats (bindings)

* [GDAL](https://github.com/georust/gdal) - Bindings for the Geographic Data Abstraction Library (GDAL) for reading and writing raster and vector GIS files.

## More crates

* [osmpbfreader-rs](https://github.com/TeXitoi/osmpbfreader-rs) - Read OpenStreetMap PBF files.
* [rstar](https://github.com/georust/rstar) - R\*-tree library for the rust ecosystem.

More topics:

* Raster and image processing
* Routing
* 3D (Meshes, TINs)
* Geocoding
* Map rendering

## Applications

* [t-rex](https://t-rex.tileserver.ch/) - Vector tile server specialized on publishing MVT tiles from your own data
* [Martin](https://github.com/urbica/martin) - Blazing fast and lightweight PostGIS vector tiles server
* [WhiteboxTools](https://jblindsay.github.io/ghrg/WhiteboxTools/) - Geospatial data analysis platform with more than 445 tools for processing various types of geospatial data. 
* [A/B Street](https://github.com/dabreegster/abstreet) - A traffic simulation game exploring how small changes to roads affect cyclists, transit users, pedestrians, and drivers.

## Awesome Geo Rust

<https://github.com/pka/awesome-georust>

## Community

* <https://github.com/georust/>
* [Discord chat](https://discord.gg/Fp2aape)


# Learning Rust

## Where to start

* <https://www.rust-lang.org/learn>
* Books, e.g. "Programming Rust" (O’Reilly)


## Thank you!

@[implgeo](https://twitter.com/implgeo)

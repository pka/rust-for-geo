fn area(width: u32, height: u32) -> u32 {
    width * height
}

#[allow(unused_variables, dead_code)]
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("area={}", area(4, 3));

    let number = 3;
    if number % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    for number in 1..4 {
        println!("{}!", number);
    }

    // #[derive(Serialize)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle {
        width: 3,
        height: 4,
    };

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let area = rect.area();

    enum ImageType {
        Png,
        Jpeg,
        Tiff,
    }

    let image_type = ImageType::Png;

    enum Color {
        Rgb(u8, u8, u8),
        Transparent,
    }

    let color = Color::Rgb(255, 0, 0);

    match color {
        Color::Rgb(r, g, b) => println!("{}/{}/{}", r, g, b),
        Color::Transparent => println!("No Color"),
    }

    let color = Color::Rgb(255, 0, 0);
    if let Color::Rgb(r, g, b) = color {
        println!("{}/{}/{}", r, g, b);
    }

    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    trait Shape {
        fn area(&self) -> u32;
    }

    impl Shape for Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let shapes = vec![Rectangle {
        width: 3,
        height: 4,
    }];
    let filtered = shapes.iter().filter(|x| x.area() > 100);

    let wh = vec![(3, 4)];
    let filtered: Vec<Rectangle> = wh
        .iter()
        .map(|&(w, h)| Rectangle {
            width: w,
            height: h,
        })
        .filter(|r| r.area() > 100)
        .collect();

    for i in (1..10).filter(|&x| x % 2 == 0) {
        println!("{}", i);
    }

    use std::thread;
    use std::time::Duration;

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
}

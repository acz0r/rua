// Transpiler playground, not a passing golden fixture yet.
// Sections are independent so unsupported constructs can be commented out.

// Literals and constants.
const X: i32 = 5;
const Y: i32 = 5i32;
const Z: i32 = -5i32;
const HEX: u32 = 0xff;
const OCTAL: u32 = 0o755;
const BINARY: u32 = 0b1010_0101;
const LARGE: i64 = 1_000_000;
const S: &str = "Hello, world!";
const EMPTY: &str = "";
const ESCAPED: &str = "slash=\\ double=\" single=' newline=\n tab=\t cr=\r nul=\0 digit_after_nul=\07 bell=\x07 backspace=\x08 vertical_tab=\x0b form_feed=\x0c escape=\x1b delete=\x7f unicode=\u{1f980}";
const RAW: &str = r#"quotes: "hello", backslash: \"#;
const UNICODE: &str = "\u{1f980}";
const C: char = 'A';
const QUOTE: char = '\'';
const NEWLINE: char = '\n';
const F: f32 = 3.14;
const D: f64 = -3.14159f64;
const SCIENTIFIC: f64 = 1.25e3;
const B: bool = true;
const FALSE: bool = false;
fn foo() {}
fn bar(arg: i32) -> bool {
    arg > 0
}
fn baz(a: i32, b: i32, c: char, d: f64, e: bool) -> bool {
    a < b && c == 'A' && d >= 0.0 && e
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn early_return(value: i32) -> i32 {
    if value < 0 {
        return 0;
    }
    add(value, 1)
}

// // Bindings, shadowing, mutation, and nested scopes.
// fn bindings(seed: i32) -> i32 {
//     let inferred = 5;
//     let explicit: i32 = 10;
//     let mut total = seed + inferred;
//     total = total + explicit;
//     total += 2;
//     total -= 1;
//     total *= 3;
//     total /= 2;
//     total %= 17;

//     let total = total + 1;
//     let nested = {
//         let total = 2;
//         total * 3
//     };
//     total + nested
// }

// // Arithmetic, comparisons, logical operators, bitwise operators, and casts.
// fn operators(a: i32, b: i32) -> (i32, bool, u32) {
//     let arithmetic = (a + b) * 2 - a / 2 + a % 3;
//     let comparisons = a == b || a != b && a <= b && b >= a;
//     let logical = !comparisons && (a < b || a > b);
//     let bits = ((a & b) | (a ^ b)) << 1;
//     let inverted = !bits >> 1;
//     (arithmetic + inverted, logical, a as u32)
// }

// // Conditional expressions and recursion.
// fn classify(value: i32) -> &'static str {
//     if value < 0 {
//         "negative"
//     } else if value == 0 {
//         "zero"
//     } else {
//         "positive"
//     }
// }

// fn factorial(n: u32) -> u32 {
//     if n <= 1 { 1 } else { n * factorial(n - 1) }
// }

// // Loops, ranges, break values, continue, and labels.
// fn loops(limit: i32) -> i32 {
//     let mut total = 0;
//     let mut index = 0;
//     while index < limit {
//         index += 1;
//         if index % 2 == 0 {
//             continue;
//         }
//         total += index;
//     }

//     for value in 0..limit {
//         total += value;
//     }
//     for value in 0..=3 {
//         total += value;
//     }

//     let found = loop {
//         total += 1;
//         if total > limit {
//             break total;
//         }
//     };

//     'outer: for row in 0..3 {
//         for column in 0..3 {
//             if row + column == 2 {
//                 break 'outer;
//             }
//             total += 1;
//         }
//     }
//     found + total
// }

// // Tuples, arrays, indexing, destructuring, and slices.
// fn collections() -> i32 {
//     let pair = (3, 4);
//     let (left, right) = pair;
//     let first = pair.0;
//     let mut values = [left, right, first];
//     values[1] = 10;
//     let repeated = [2; 4];
//     let slice = &values[1..];
//     let [head, ..] = values;
//     head + slice[0] + repeated[3]
// }

// // Structs, field access, update syntax, methods, and references.
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Point {
//     fn new(x: i32, y: i32) -> Self {
//         Self { x, y }
//     }

//     fn sum(&self) -> i32 {
//         self.x + self.y
//     }

//     fn shift(&mut self, amount: i32) {
//         self.x += amount;
//         self.y += amount;
//     }
// }

// fn structs() -> i32 {
//     let mut point = Point::new(1, 2);
//     point.shift(3);
//     let other = Point { x: 10, ..point };
//     let Point { x, y } = other;
//     point.sum() + x + y
// }

// // Enums and match patterns, including guards and wildcards.
// enum Shape {
//     Empty,
//     Circle(i32),
//     Rectangle { width: i32, height: i32 },
// }

// fn area(shape: Shape) -> i32 {
//     match shape {
//         Shape::Empty => 0,
//         Shape::Circle(radius) if radius > 0 => radius * radius * 3,
//         Shape::Circle(_) => 0,
//         Shape::Rectangle { width, height } => width * height,
//     }
// }

// fn patterns(value: i32) -> bool {
//     match value {
//         0 => false,
//         1 | 2 => true,
//         3..=9 => true,
//         _ => false,
//     }
// }

// // Standard-library types and control flow over optional/fallible values.
// fn optional(value: Option<i32>) -> i32 {
//     if let Some(number) = value { number } else { 0 }
// }

// fn pop_sum() -> i32 {
//     let mut values = vec![1, 2, 3];
//     values.push(4);
//     let mut total = 0;
//     while let Some(value) = values.pop() {
//         total += value;
//     }
//     total
// }

// fn checked(value: i32) -> Result<i32, &'static str> {
//     if value < 0 {
//         Err("negative input")
//     } else {
//         Ok(value)
//     }
// }

// fn propagate(value: i32) -> Result<i32, &'static str> {
//     let number = checked(value)?;
//     Ok(number + 1)
// }

// // Closures, function values, generics, and nested modules.
// fn closures(seed: i32) -> i32 {
//     let double = |value: i32| value * 2;
//     let captured = |value| value + seed;
//     let function: fn(i32, i32) -> i32 = add;
//     function(double(3), captured(4))
// }

// fn identity<T>(value: T) -> T {
//     value
// }

// mod helpers {
//     pub fn triple(value: i32) -> i32 {
//         value * 3
//     }
// }

// fn qualified_call() -> i32 {
//     helpers::triple(identity(4))
// }

// // Deferred: these need more than straightforward Lua syntax lowering.
// /*
// trait Render {
//     fn render(&self) -> String;
// }

// impl Render for Point {
//     fn render(&self) -> String {
//         format!("({}, {})", self.x, self.y)
//     }
// }

// async fn asynchronous(value: i32) -> i32 {
//     async { value + 1 }.await
// }

// macro_rules! twice {
//     ($value:expr) => {
//         $value + $value
//     };
// }
// */

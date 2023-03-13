#![allow(dead_code)]

use crate::garden::vegetables::Asparagus;
use std::collections::HashMap;

pub mod garden;

fn main() {
    // using_structs();
    // using_struct_methods();
    // using_enums();
    // project_structure();
    vectors();
    // strings();
    // hashmaps();
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn c_to_f(c: f64) -> f64 {
    c * (9. / 5.) + 32.
}

fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn string_ownership(s: &String) {
    println!("{}", s);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn fun_with_iter() {
    let foo: Vec<u8> = vec![1, 2, 3].iter().map(|x| x * 5).collect();
    println!("{:?}", foo);

    vec![1, 2, 5, 9, 4, 10]
        .iter()
        .skip(2)
        .take_while(|&&x| x > 4)
        .for_each(|x| println!("{}", x));

    let cnt: usize = vec![1, 2, 3].iter().filter(|&&x| x % 2 == 0).count();

    println!("{}", cnt);
}

fn fun_with_ownership() {
    let s1 = String::from("hello");
    println!("{}", s1.to_uppercase());
    let mut s2 = s1;
    println!("{}", s2.to_uppercase());
    string_ownership(&s2);
    s2.push_str(" dude");
    println!("{}", s2);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: usize,
}

fn using_structs() {
    let _user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let username = String::from("yoyoy");
    let email = String::from("yoyoy@example.com");
    let mut user2 = User {
        active: true,
        username,
        email,
        sign_in_count: 2,
    };
    user2.active = false;

    let mut _user3 = User {
        email: String::from("another@example.com"),
        .._user1 // this must come last
                 // also _user1 is now unusable since `_user1.username` was moved into `_user3`
    };

    dbg!(&_user3);
    _user3.active = false;
}

struct Color(i32, i32, i32);
fn using_tuple_structs() {
    let black = Color(0, 0, 0);
    println!("Black hex values are {} {} {}", black.0, black.1, black.2);
}

struct AlwaysEqual; // "unit-like struct"
fn using_unit_like_struct() {
    let _subject = AlwaysEqual;
}

#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        return self.width * self.height;
    }

    fn flatten(&mut self) {
        self.height = self.height / 4;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn from_square(width: usize) -> Self {
        return Self {
            width,
            height: width,
        };
    }
}

fn using_struct_methods() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    println!(
        "The area of the rectangle {:?} is {} square pixels",
        rect1,
        rect1.area()
    );

    rect1.width = 100;
    rect1.flatten();
    println!(
        "The area of the rectangle {:?} is now {} square pixels",
        rect1,
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::from_square(200);
    println!("Can rect3 hold rect1? {}", rect3.can_hold(&rect1));
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

impl std::ops::Add for Coin {
    type Output = u8;
    fn add(self, other: Coin) -> Self::Output {
        self.value_in_cents() + other.value_in_cents()
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn plus_one2(x: Option<i32>) -> Option<i32> {
    Some(x? + 1)
}

fn using_enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}, {:?}", four, six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    println!(
        "{:?} + {:?} = {} cents",
        Coin::Penny,
        Coin::Dime,
        Coin::Penny + Coin::Dime
    );

    println!("plus_one(Some(5)): {:?}", plus_one(Some(5)));
    println!("plus_one(None): {:?}", plus_one(None));
    println!("plus_one2(Some(5)): {:?}", plus_one2(Some(5)));
    println!("plus_one2(None): {:?}", plus_one2(None));

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn project_structure() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
}

fn vectors() {
    let mut v = vec![1, 2, 3, 4, 5];
    let _first = &v[0];
    v.push(6);

    // v cant do this since v might exist on a entirely new block of mem when enlarged
    // println!("the first element is {first}");

    for i in &v {
        println!("{i}");
    }
    for i in &mut v {
        *i += 50;
    }

    let list_of_nums = vec![
        vec![1, 2, 3, 4, 5],
        vec![3, 3, 3, 5, 2, 1],
        vec![3, 3, 1, 1],
        vec![3, 3, 2, 1],
    ];
    for nums in &list_of_nums {
        println!("mode of {:?} is {}", nums, mode(nums));
    }
}

fn mode(nums: &Vec<usize>) -> usize {
    let mut map = HashMap::new();
    for &num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut mode = None;
    let mut max_count = 0;

    for (number, count) in map {
        if count > max_count {
            max_count = count;
            mode = Some(number);
        }
    }

    mode.expect("List cannot be empty")
}

fn strings() {
    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    println!("s2 is {s2}");

    let s3 = s1 + &s2; // s1 is moved
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // like println! uses references so values not moved
    println!("{}, {} {} {}", s, s1, s2, s3);
    let s = String::from("नमस्ते");
    for c in s.chars() {
        println!("{c}");
    }
}

fn hashmaps() {
    let mut scores = HashMap::new();
    // primitives will be copied into the map but non-prims will be moved
    scores.insert("Blue", 10);
    scores.insert("Yellow", 30);
    scores.insert("Blue", 20); // overwrites prev values

    // insert only if new key
    scores.entry("Black").or_insert(50);
    scores.entry("Yellow").or_insert(50);
    scores.entry("Blue").or_insert(50);

    println!("{:?}", scores);
    let _score = scores.get("Blue").copied().unwrap_or(0);

    for (key, val) in &scores {
        println!("{key}: {val}")
    }
    count_words();
}

fn count_words() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

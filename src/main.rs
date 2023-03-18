#![allow(dead_code)]

use crate::garden::vegetables::Asparagus;
use std::{collections::HashMap, io::Read};

pub mod garden;

fn main() {
    // using_structs();
    // using_struct_methods();
    // using_enums();
    // project_structure();
    // vectors();
    // strings();
    // hashmaps();
    // read_lines2();
    // using_generics();
    // playing_with_traits();
    life_times();
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

fn erroring() {
    use std::fs::File;
    use std::io::ErrorKind;

    let filename = "hello.txt";

    let greeting_file_result = File::open(filename);
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(file) => file,
                Err(err) => panic!("Problem creating the file: {:?}", err),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let _greeting_file = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename)
                .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn get_username(filename: &str) -> Result<String, std::io::Error> {
    let mut username_file = std::fs::File::open(filename)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn get_username_shorter(filename: &str) -> Result<String, std::io::Error> {
    let mut username = String::new();
    std::fs::File::open(filename)?.read_to_string(&mut username)?;
    Ok(username)
}

fn get_username_even_shorter(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename)
}

fn read_lines() {
    let mut file = std::fs::File::open("lines").expect("couldn't open file");
    let mut lines = String::new();
    file.read_to_string(&mut lines).unwrap();
    lines.lines().for_each(|line| println!("{}", line));
}

fn read_lines2() {
    std::fs::read_to_string("lines")
        .unwrap()
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));
}

enum Colors {
    Red,
    Green,
    Blue,
}

fn print_color(color: Colors) {
    match color {
        Colors::Red => println!("red",),
        Colors::Green => println!("green",),
        Colors::Blue => println!("blue",),
    };
}

fn using_generics() {
    let nums = vec![34, 50, 25, 100, 65];
    println!("the largest of {:?} is {}", nums, largest(&nums));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("the largest of {:?} is {}", chars, largest(&chars));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if largest < item {
            largest = item;
        }
    }
    return largest;
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn some_default_method(&self) -> String {
        String::from("doing some default stuff")
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn some_default_method(&self) -> String {
        String::from("Tweet overrides trait default behavior")
    }
}

fn notify(item: &(impl Summary + std::fmt::Debug)) {
    println!("Breaking news! {}", item.summarize());
}

// can only ever return one type tho
fn get_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn playing_with_traits() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!(
        "article:\n\t{}\n\t{}\n",
        article.summarize(),
        article.some_default_method()
    );
    println!(
        "tweet:\n\t{}\n\t{}\n",
        tweet.summarize(),
        tweet.some_default_method()
    );

    notify(&article);
    notify(&tweet);

    let pair = Pair { x: 10, y: 2 };
    pair.cmp_display();
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn life_times() {
    // let foo = vec![1,3,4,5].iter();
    // foo.for_each(|x| println!("{}", x));

    let r;
    let _s;
    {
        let x = "xyz";
        let y = 12;
        r = x;
        _s = &y;
    }
    println!("r: {}", r);
    // println!("s: {}", _s);
    // ^ can't do cuz the reference to `&y` is gone

    // basic example - lifetime of `string1`, `string2` & `result` is the same
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_string(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // noice example
    let string1 = String::from("long string is long");
    let _result;
    {
        let string2 = String::from("xyz");
        _result = longest_string(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result);
    // ^ can't do cuz `longest_string` `result` only lives as long as the shorter lifetime, in this
    // case the `string2` even though WE know that `_result` would really be `string1`, the
    // compiler doesn't know that

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = "xyz";
        result = longest_string(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);
    // ^ this is actually ok cuz string literals are stored on the binary as read-only data so they
    // have infinite lifetime (known is `'static`)

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest_string_dumb(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    // ^ now this is allowed cuz `longest_string_dumb` is ALWAYS returnings `string1` which has a
    // longer lifetime than `string2` and the `result`'s lifetime is tied to that of `string1`

    // struct that borrow variables need to be aware of lifetimes
    lifetimes_with_structs();
    println!("string literals are special: {}", static_lifetimes());

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        _ = generic_type_lifetime_shenanigans(string1.as_str(), string2.as_str(), 12);
        result = generic_type_lifetime_shenanigans(
            string1.as_str(),
            string2.as_str(),
            "generics are dank",
        );
        println!("The longest string is {}", result);
    }
}

fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    }
    return s2;
}

fn longest_string_dumb<'a>(s1: &'a str, s2: &str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    }
    return s1;
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        return 3;
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        return self.part;
    }
}

fn lifetimes_with_structs() {
    let novel = String::from("Call me Ishmael, Some years ago...");
    let first_sentence = novel
        .split(".")
        .next()
        .expect("Could not find a complete sentence");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}

fn static_lifetimes() -> &'static str {
    return "I live forever!!!";
}

fn generic_type_lifetime_shenanigans<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        return x;
    }
    return y;
}

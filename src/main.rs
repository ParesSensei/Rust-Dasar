mod first;
mod second;
mod third;
mod model;

// use std::ops::Add;
use crate::first::say_hello;
use second::say_hello as say_hello_second;
#[test]
fn test_use() {
    say_hello();
    say_hello_second();
    crate::first::second::third::say_hello();
}

#[test]
fn test_module() {
    let user: model::User = model::User{
        first_name: String::from("Eko"),
        last_name: String::from("Khannedy"),
        username: String::from("Khannedy"),
        email: String::from("khunsan@example.com"),
        age: 20,
    };

    user.say_hello("Pares");
}


fn main() {
    println!("Hello, world!");

    println!("Hello, jink!");

    println!("Hello, yo!");
}


#[test]
fn unit_test() {
    println!("Hello, world!");
}

#[test]
fn test_variable() {
    let nama = "ParesSensei";
    println!("hello {nama}");
}

#[test]
fn test_mutable() {
    let mut nama = "ParesSensei";
    println!("hello {nama}");

    nama = "budi ampun roti";
    println!("hello {nama}");
}

#[test]
fn static_typing() {
    let nama = "ParesSensei";
    println!("hello {nama}");

    // nama = 10;
    println!("hello {}", nama);
}

#[test]
fn shadowing() {
    let nama = "ParesSensei";
    println!("hello {}", nama);

    let nama = 10; // secara default akan pakai i32 untuk integer
    println!("hello {}", nama);
}


#[test]
fn explicit() {
    let age : i32 = 20;
    println!("{}", age);
}

#[test]
fn number() {
    let a: i8 = 10; // i8 hampir mirip di solidity pakai uint8
    println!("{}", a);

    let b: f32 = 10.5; // f32 juga sama mirip
    println!("{}", b);
}


#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);
}

#[test]
fn numeric_operator() {
    let a = 10; // let biasa secara default adalah immutable jadi tidak bisa diubah
    let b = 10; // nilai a akan selalu tetap 10
    let c = a * b;
    println!("{}", c);
    let d = a / b;
    println!("{}", d);
    let e = a + b;
    println!("{}", e);
}


#[test]
fn augmented_assigment() {
    let mut a = 10; // mut adalah mjutable jadi bisa diubah
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 5;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true; // implicit
    let b: bool = false; // explicit

    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let a = 10;
    let b = 10;

    let result : bool = a > b;
    println!("{}", result);
}


#[test]
fn boolean_operator() {
    let absen = 75;
    let nilai_akhir = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai: bool = nilai_akhir >= 75;

    let lulus: bool = lulus_absen && lulus_nilai;
    println!("{}", lulus);
}

#[test]
fn char_type() {
    let char1: char = 'a'; // tipe data char wajib petik 1
    let char2: char = 'b';

    println!("{} {}", char1, char2);
}


#[test]
fn tuple () {
    let mut data:  (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a, b, c) = data;

    println!("{} {} {}", a, b, c);


    data.0 = 20;
    data.1 =  20.5;
    data.2 = false;

    println!("{:?}", data);
}


#[test]
fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result: () = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}


#[test]
fn array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);


    array[0] = 10;
    array[1] = 20;

    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);
}


#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [
        [1,2,3],
        [4,5,6],
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}


#[test]
fn variable_scope() {
    let eko = 1;
    {
        println!("{}", eko);
        let kurniawan = 2;
        println!("{}", kurniawan);
    }
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Kurniato");
    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10; // dismpan di stack
    let b = String::from("eko"); // disimpan di heap
    println!("{}, {}", a, b);
}

// shortcut tfn
#[test]
fn string() {
    // string slice disimpan di stack
    let name: &str = "   Pares Sensei   "; // fixsize
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);


    let mut username: &str = "Pares";
    println!("{}", username);
    username = "Budi";
    println!("{}", username);

    // string slice diaats  sama sperti ini, cuma mengganti isi bukan mengganti variable nya, jadi sebenarnya
    let mut _value = 10; // data lama tetap ada
    _value = 11; // dan ini data baru
}

// shortcut tfn
#[test]
fn string_type() {
    let mut name = String::from("Pares");
    println!("{}", name);

    name.push_str(" Sensei");
    println!("{}", name);

    let budi = name.replace("Pares", "Budi"); // lupa bbjir intinya gabisa print name, harus disimpan di variable baru
    println!("{}", name);
    println!("{}", budi);
}

#[test]
fn ownership_rules() {
    // a tidak bisa diakses disini, belum di deklarasikan
    let a = 10; // a bisa diakses nulai disini

    { // b tidak bisa diakses disini, belum dideklarasikan
        let b = 20; // b bisa diakses mulai dari sini
        println!("{}", b);
    } // scope b selesai b dihapus, b tidak bisa diakses lagi

    println!("{}", a);
} // scope a selesai, a dihapus, a tidak bisa diakses lagi

#[test]
fn ownership_movement() {
    let name1 = String::from("Pares"); // tidak bisa copy karena hanya khusus stak bukan tipe data heap
    println!("{}", name1);
    // ownership name1 dipindah ke name2

    let name2 = name1;
    // name1 tidak bisa lagi diakses karna ownership sudah dipindah
    println!("{}", name2);
    // println!("{}", name1);
}

#[test]
fn clone() {
    let name1 = String::from("Pares");
    let name2 = name1.clone();

    println!("{}", name2);
}

#[test]
fn if_expression() {
    let value = 4;

    if value >= 8 {
        println!("Good pointer");
    } else if value >=6 {
        println!("Not bad pointer");
    } else if value >=3 {
        println!("Bad pointer");
    } else {
        println!("Very Bad Pointer")
    }
}

#[test]
fn if_expression2() {
    let value = 10;
    let result: &str = if value >= 8 {  // bisa digabung bro canggih
        "Good pointer"
    } else if value >=6 {
        "Not bad pointer"
    } else if value >=3 {
        "Bad pointer"
    } else {
        "Very Bad Pointer"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        } else if counter % 2 == 0{
            continue;
        }

        println!("{}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter < 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("value: {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array{
        println!("Value : {}", value);
    }
}

#[test]
fn range() {
    let range = 1..5;
    println!("start: {}", range.start);
    println!("end: {}", range.end);

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range{
        println!("{}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let range = 0..=4;
    println!("start: {}", range.start());
    println!("end: {}", range.end());

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range {
        println!("{}", array[i]);
    }
}


// #[test]
// fn say_hello() {
//     println!("Hello")
// }

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
}


fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}" , first_name, last_name );
}

#[test]
fn test_say_goodbye() {
    say_goodbye("eko", "khanedy");
    say_goodbye("pares", "sensei");
    say_goodbye("joko", "santosi");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..n+1 {
        result *= i;
    }
    result
}

#[test]
fn test_factorial_loop() {
    let result: i32 = factorial_loop(5);
    println!("{}", result);

    let result: i32 = factorial_loop(10);
    println!("{}", result);
}

fn prin_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    prin_text(value, times - 1);
}

#[test]
fn test_printed_text() {
    prin_text(String::from("Pares"), 10);
}

fn factorial_recursive(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result: i32 = factorial_recursive(5);
    println!("{}", result);
}


fn print_number(number: i32) {
    println!("Number {}", number);
}

fn hi(name: String) {
    println!("Hello, {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Eko");
    hi(name);
    // println!("{}", name);
}

fn full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Pares");
    let last_name = String::from("Sensei");

    let full_name= full_name(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(value: &mut String) {
    value.push_str("Test");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Eko");

    let value_borrow = &mut value;

    change_value(value_borrow);
    change_value(value_borrow);
    change_value(value_borrow);

    println!("{}", value);
}

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Pares");
    let last_name = String::from("Sensei");

    let full_name= get_full_name(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];

    let slice1 = &array[..];
    println!("{:?}", slice1);

    let slice2 = &array[0..5];
    println!("{:?}", slice2);

    let slice3 = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name = String::from("Eko Kurniawan Khannedy");

    let first_name: &str = &name[0..3];
    println!("{}", first_name);

    let last_name: &str = &name[14..];
    println!("{}", last_name);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn say_hello(self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn test_struct_person() {
    let person: Person = Person{
        age: 20,
        first_name: String::from("Pares"),
        middle_name: String::from("Khannedy"),
        last_name: String::from("Sensei"),
    };

    print_person(&person);

    let person2 = Person{
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };

    print_person(&person2);

    println!("{}", person.first_name);
}

struct GeoPoint (f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point: GeoPoint = GeoPoint::new(60.0, 20.0);
    println!("{}",geo_point.0);
    println!("{}",geo_point.1);
}

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.57843, 10.57842);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing{};
}

#[test]
fn test_method() {
    let person: Person = Person{
        age: 20,
        first_name: String::from("Pares"),
        middle_name: String::from("Khannedy"),
        last_name: String::from("Sensei"),
    };

    person.say_hello("Budi")
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_level() {
    let level: Level = Level::Premium;

    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

enum Payment{
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment{
    fn pay(&self, amount: u32){
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with bank transfer {} {} amount {}", bank, number, amount);
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with EWALlet {} {} amount {}", wallet, number, amount);
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1: Payment = Payment::CreditCard(String::from("423423423"));
    _payment1.pay(1000000);

    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("324324324"));
    _payment2.pay(2000000);

    let _payment3: Payment = Payment::EWallet(String::from("Gopay"), String::from("324324324"));
    _payment3.pay(7600000);
}

#[test]
fn test_match_value() {
    let name = "Budi";

    match name {
        "Eko" => {
            println!("Hello Eko");
        }
        "Budi" => {
            println!("Hello Budi");
        }
        other => {
            println!("Hello {}", other);
        }
    }

    match name {
        "Eko" | "Budi" | "Joko" => {
            println!("Hello Bos");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

#[test]
fn tes_range_pattern() {
    let value = 1900;
    match value {
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not Bad")
        }
        0..=24 => {
            println!("Bad")
        }
        other => {
            println!("You Are Out of Order {}", other);
        }
    }
}

#[test]
fn test_struct_pattern() {
    let point = GeoPoint::new(0.0, 20.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("Long: {}", long);
        }
        GeoPoint( 0.0, lat) => {
            println!("lat: {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long: {}, lat: {}", long, lat);
        }
    }
    
    let person = Person{
        first_name: String::from("eko"),
        middle_name: String::from("kurniawan"),
        last_name: String::from("khunnedy"),
        age: 20,
    };
    
    match person {
        Person { first_name, last_name, ..} => {
            println!("first_name: {}, last_name: {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignoring() {
    let point = GeoPoint::new(0.0, 20.0);

    match point {
        GeoPoint(long,_) => {
            println!("Long: {}", long);
        }
    }
}

#[test]
fn tes_ignoring_range() {
    let value = 1900;
    match value {
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not Bad")
        }
        0..=24 => {
            println!("Bad")
        }
        _ => {
            println!("You Are Out of Order");
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 2;

    let result = match value {
        0 => "nol",
        1 => {
            "satu"
        },
        2 => {
            "dua"
        },
        _ => "Invalid"
    };

    println!("result: {}", result);
}

type Age = u8;
type IdentityNumber = String;


struct Customer{
    id: IdentityNumber,
    name: String,
    age: Age,
}

#[test]
fn test_identity_number() {
    let customer = Customer{
        id: String::from("8676454787"),
        name: String::from("EKo"),
        age: 20,
    };

    println!("{} {} {}", customer.id, customer.name, customer.age);
}

trait CanSayHello {
    fn hello(&self) -> String {
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodBye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello, {}, my name is {}", name, self.first_name)
    }
}

impl CanSayGoodBye for Person {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.first_name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye, {}, my name is {}", name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello());
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)) {
    println!("{}", value.hello());
    println!("{}", value.good_bye());
}

#[test]
fn test_trait() {
    let person = Person{
        first_name: String::from("eko"),
        middle_name: String::from("khunedy"),
        last_name: String::from("khun san"),
        age: 20,
    };

    say_hello_trait(&person);
    hello_and_goodbye(&person);

    let result = person.say_hello_to("amira");
    println!("{}", result);

    let result = person.hello();
    println!("{}", result);

    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Budi"));
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodBye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye, {}, my name is {}", name, self.name)
    }
}

fn create_person(name: String) -> SimplePerson {
    SimplePerson {name}
}

#[test]
fn test_return_trait() {
    let person =  create_person(String::from("eko"));
    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Budi"));
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {

    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_struck() {
    let integer:Point<i32> = Point::<i32>{
        x: 1, y: 2,
    };
    let float:Point<f64> = Point::<f64> {
        x: 1.0, y: 2.0,
    };
    println!("{} {}", integer.x, integer.y);
    println!("{} {}", float.x, float.y);
}

enum Value<T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_generic_enum() {
    let value = Value::<i32>::VALUE(10);

    match value {
        Value::NONE => {
            println!("NONE");
        }
        Value::VALUE(value) => {
            println!("Value {}", value);
        }
    }
}

struct Hi<T: CanSayGoodBye> {
    value: T,
}

#[test]
fn test_generic_bound() {
    let hi = Hi::<SimplePerson>{
        value: SimplePerson {
            name: String::from("eko"),
        }
    };
    println!("{}", hi.value.name);
}

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        return value1;
    } else {
        return value2;
    }
}

#[test]
fn generic_in_function() {
    let result = min::<i32>(10, 20);
    println!("{}", result);

    let result = min(10.6, 5.9);
    println!("{}", result);
}

#[test]
fn test_generic_method() {
    let point = Point::<i32>{x: 10, y: 20};
    println!("{}", point.get_x());
    println!("{}", point.get_y());
    println!("{}", point.get_value());
}

trait GetValue<T> where T: PartialOrd {
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T>  where T: PartialOrd {
    fn get_value(&self) -> &T {
        &self.x
    }
}

use::core::ops::Add;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 10};

    let apple3 = apple1 + apple2;
    println!("{}", apple3.quantity);
}

fn double(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i * 2)
    }
}

#[test]
fn test_option() {
    let result = double(Some(10));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result);
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_comparing() {
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 20};

    println!("Apple1 == Apple2: {}", apple1 == apple2);
    println!("Apple1 < Apple2: {}", apple1 < apple2);
    println!("Apple1 > Apple2: {}", apple1 > apple2);
}

#[test]
fn string_manipulation() {
    let s = String::from("Eko Kurniawan Khannedy");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("Eko", "Pares"));
    println!("{}", s.contains("Kurniawan"));
    println!("{}", s.starts_with("Eko"));
    println!("{}", s.ends_with("Khannedy"));
    println!("{}", s.trim());
    println!("{}", &s[0..3]);
    println!("{:?}", s.get(0..3));
}

struct Category{
    id: String,
    name: String,
}

use std::fmt::{Debug};

impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
        .field("id", &self.id)
        .field("name", &self.name)
        .finish()
    }
}

#[test]
fn test_formet() {
    let category = Category{
        name: String::from("Gadget"),
        id: String::from("GADGET"),
    };

    println!("{:?}", category);
}

#[test]
fn test_closure() {
    let sum: fn(i32, i32) -> i32 = |value1: i32, value2: i32| -> i32 {
        value1 + value2
    };

    let result = sum(10, 20);
    println!("{}", result);
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("{}", result);
}

#[test]
fn test_closure_as_parameter() {
    let filter = |value: String| -> String {
        value.to_uppercase()
    };

    print_with_filter(String::from("Eko"), filter);
}

fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_function_as_closure() {
    print_with_filter(String::from("Eko"), to_uppercase);
}

#[test]
fn test_closure_scope() {
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        println!("Increment")
    };

    increment();
    increment();
    increment();

    println!("Coounter {}", counter);
}

struct Counter {
    coounter: i32
}

impl Counter {
    fn increment(&mut self) {
        self.coounter += 1;
        println!("Increment");
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter{coounter: 0};
    counter.increment();
    counter.increment();
    counter.increment();

    println!("Counter {}", counter.coounter);
}

#[test]
fn test_vector() {
    let mut names: Vec<String> = Vec::<String>::new();
    names.push(String::from("Eko"));
    names.push(String::from("Pares"));
    names.push(String::from("khannedy"));

    for name in &names {
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn test_vector_deque() {
    let mut names: VecDeque<String> = VecDeque::<String>::new();
    names.push_back(String::from("Eko"));
    names.push_back(String::from("Kurniawan"));
    names.push_front(String::from("khannedy"));

    for name in &names {
        println!("{}", name);
    }

    println!("{}", names[1]);
}

#[test]
fn test_linked_list() {
    let mut names: LinkedList<String> = LinkedList::<String>::new();
    names.push_back(String::from("Eko"));
    names.push_back(String::from("Kurniawan"));
    names.push_front(String::from("khannedy"));

    for name in &names {
        println!("{}", name);
    }
}

#[test]
fn test_hash_map() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("name"), String::from("eko"));
    map.insert(String::from("age"), String::from("25"));
    map.insert(String::from("Country"), String::from("Indonesia"));

    let name = map.get("name");
    let age = map.get("age");

    println!("name: {}", name.unwrap());
    println!("age: {}", age.unwrap());

    for entry in &map {
        println!("{} : {}", entry.0, entry.1);
    }
}

#[test]
fn test_btree_map() {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert(String::from("name"), String::from("eko"));
    map.insert(String::from("age"), String::from("25"));
    map.insert(String::from("Country"), String::from("Indonesia"));

    let name = map.get("name");
    let age = map.get("age");

    println!("name: {}", name.unwrap());
    println!("age: {}", age.unwrap());

    for entry in &map {
        println!("{} : {}", entry.0, entry.1);
    }
}

#[test]
fn test_hash_set() {
    let mut set: HashSet<String> = HashSet::new();
    set.insert(String::from("Eko"));
    set.insert(String::from("Eko"));
    set.insert(String::from("Kurniawan"));
    set.insert(String::from("Kurniawan"));
    set.insert(String::from("Khannedy"));
    set.insert(String::from("Khannedy"));

    for value in &set {
        println!("{}", value);
    }
}

#[test]
fn test_btree_set() {
    let mut set: BTreeSet<String> = BTreeSet::new();
    set.insert(String::from("Eko"));
    set.insert(String::from("Eko"));
    set.insert(String::from("Kurniawan"));
    set.insert(String::from("Kurniawan"));
    set.insert(String::from("Khannedy"));
    set.insert(String::from("Khannedy"));

    for value in &set {
        println!("{}", value);
    }
}

#[test]
fn test_iterator() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next() {
        println!("{}", value);
    }

    for value in iterator {
        println!("{}", value);
    }
}

#[test]
fn test_iterator_method() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", vector);

    let sum: i32 = vector.iter().sum();
    println!("{:?}", sum);

    let count: usize = vector.iter().count();
    println!("{:?}", count);

    let doubles: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("{:?}", doubles);

    let odd: Vec<i32> = vector.iter().filter(|&&x| x % 2 != 0).copied().collect();
    println!("{:?}", odd);
}

fn connect_database(host: Option<String>) {
    match host {
        Some(host) => {
            println!("connected to {}", host);
        }
        None => {
            panic!("No Database host provided!");
        }
    }
}

#[test]
fn test_panic() {
    connect_database(Some(String::from("localhost")));
    // connect_database(None)
}

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        None => {
            Err(String::from("No cache host provided!"))
        }
        Some(host) => {
            Ok(host)
        }
    }
}

fn connect_email(host: Option<String>) -> Result<String, String> {
    match host {
        None => {
            Err(String::from("No Email host provided!"))
        }
        Some(host) => {
            Ok(host)
        }
    }
}

fn connect_application(host: Option<String>) -> Result<String, String> {
    // let connect_cache = connect_cache(host.clone());
    // match connect_cache {
    //     Ok(_) => {}
    //     Err(err) => {
    //         return Err(err);
    //     }
    // }
    //
    // let connect_email = connect_email(host.clone());
    // match connect_email {
    //     Ok(_) => {}
    //     Err(err) => {
    //         return Err(err);
    //     }
    // }

    connect_cache(host.clone())?;
    connect_email(host.clone())?;
    Ok("Connected to application".to_string())
}

#[test]
fn test_application_error() {
    // let result = connect_application(Some("localhost".to_string()));
    let result = connect_application(None);
    match result {
        Ok(host) => { println!("success connected with message: {}", host); }
        Err(err) => { println!("error with message: {}", err); }
    }
}

#[test]
fn test_recoverable_error() {
    // let cache = connect_cache(Some("localhost".to_string()));
    let cache = connect_cache(None);

    match cache {
        Ok(host) => {
            println!("succes connect to host: {}", host);
        }
        Err(error) => {
            println!("error with message: {}", error);
        }
    }
}

#[test]
fn test_dangling_reference() {
    let r: &i32;
    {
        let _x = 5;
        // r = &x; error
    }
    r = &40;
    println!("r: {}", r);
}

fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

#[test]
fn test_lifetime_annotation() {
    let value1 = "eko";
    let value2 = "Kurniawan";
    let result = longest(value1, value2);
    println!("result: {}", result);
}

#[test]
fn test_lifetime_annotation_dangling_reference() {
    let string1 = String::from("Khannedy");
    let string2 = String::from("Eko");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("result: {}", result);
}

struct Student<'a, 'b> {
    name: &'a str,
    last_name: &'b str,
}

impl <'a, 'b> Student<'a, 'b> {
    fn longest_name(&self, student: &Student<'a, 'b>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
    }
}

#[test]
fn test_student() {
    let student: Student = Student{
        name: "Eko",
        last_name: "Khannedy",
    };
    println!("{}", student.name);

    let student2 = Student{
        name: "Budi",
        last_name: "Nugraha",
    };
    let result = longest_student_name(&student, &student2);
    println!("{}", result);

    let result = student.longest_name(&student2);
    println!("{}", result);
}

struct Teacher<'a, ID> where ID: Ord {
    id: ID,
    name: &'a str,
}

#[test]
fn test_lifetime_annotation_generic() {
    let teacher: Teacher<i32> = Teacher{
        id: 10,
        name: "Eko",
    };
    println!("{}", teacher.id);
    println!("{}", teacher.name);
}
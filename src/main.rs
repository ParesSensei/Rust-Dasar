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


#[test]
fn say_hello() {
    println!("Hello")
}

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


use std::io::stdin;
use std::io::stdout;
use std::io::Write;


fn sum_numbers() {
    print!("Enter numbers to add seperated by spaces: ");
    let _= stdout().flush();

    let mut s = String::new();
    stdin().read_line(&mut s).expect("");

    let mut sum: i64 = 0;

    for word in s.split_whitespace() {
        sum += word.parse().unwrap();
    }

    println!("{}", sum);
}

fn calc_int_arithmetic(){
    print!("Enter first number: ");
    let _= stdout().flush();

    let mut s = String::new();
    stdin().read_line(&mut s).expect("");
    let num1: i64 = s.trim().parse().unwrap();

    print!("Enter second number: ");
    let _= stdout().flush();

    let mut s = String::new();

    stdin().read_line(&mut s).expect("");
    let num2: i64 = s.trim().parse().unwrap();

    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;
    let quotient = num1 / num2;
    let remainder = num1 % num2;

    println!("Sum: {}\nDifference {}\nProduct: {}\nRemainder: {}\nQuotient: {}",
        sum, difference, product, remainder, quotient);
}

fn multiplication_table() {
    println!("X |   1   2   3   4   5   6   7   8   9  10  11  12");
    println!("---------------------------------------------------");
    for i in 1..13 {
        print!("{:<2}|", i);
        for x in 1..13 {
                print!("{:4}", i*x);
        }
        println!();
    }
}

fn fizzbuzz() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{} - FizzBuzz", i);
        }
        else if i % 3 == 0 {
            println!("{} - Fizz", i);
        }
        else if i % 5 == 0 {
            println!("{} - Buzz", i);
        }
        else {
        println!("{}", i);
        }
    }
}

fn multiply(num1: i64, num2: i64) -> i64 {
    num1 * num2
}

fn greater_less_equal() {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("");
    let num1: i64 = s.trim().parse().unwrap();

    let mut s = String::new();
    stdin().read_line(&mut s).expect("");
    let num2: i64 = s.trim().parse().unwrap();

    if num1 > num2 {
        println!("{} is greater than {}", num1, num2);
    }
    else if num1 < num2 {
        println!("{} is less than {}", num1, num2);
    }
    else if num1 == num2{
        println!("{} is equal to {}", num1, num2);
    }
}

fn append_strings() {
    let mut s1 = "test".to_owned();
    let s2 = "hello";
    s1.push_str(s2);
    println!("{}", s1);
}

fn concat_strings() {
    let s1 = "test".to_owned();
    let s2 = "hello";
    let s3 = s1 + s2;
    println!("{}", s3);
}

use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::str;
use std::thread;


fn handle_client(mut stream: TcpStream) {

    let _= stream.write("\nEcho server: enter text and have it echoed back\n".as_bytes());
    let _= stream.write("-----------------------------------------------\nYou: ".as_bytes());

    loop
    {
        let mut buf = [0; 512];
        let bytecount = stream.read(&mut buf).unwrap();

        if bytecount != 0
        {
            let s = &str::from_utf8(&buf).unwrap().trim()[..bytecount-2];

            if s == "exit"
            {
                println!("client quit");
                break;
            }

            let _= stream.write("Server: ".as_bytes());
            let _= stream.write(&buf);
            let _= stream.write("You: ".as_bytes());

            println!("user: '{}'", s);
        }
    }
}

fn echoserver() {
    let addr = "127.0.0.1:8888";
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening: {}", addr);

    for stream in listener.incoming() {
        println!("New client");
        thread::spawn(move || {
            handle_client(stream.unwrap());
        });
    }
}

fn main() {
    if false {
        sum_numbers();
        calc_int_arithmetic();
        multiplication_table();
        fizzbuzz();
        multiply(2,4);
        greater_less_equal();
        append_strings();
        concat_strings();
        echoserver();
    }
}

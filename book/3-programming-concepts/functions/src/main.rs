// functions are usually in snake_case
fn main() {
    println!("Hello, world!");
    response(); // no params
    another_function(69); // func with params
    format_question_number(50, 'a'); // multiple params

    // this funky let thing which is a statement and a function in disguise
    let x = {
        let y = 3;
        y + 1 // don't use compound assignment: x += 1, doesn't work because the x+1 is directly assigned to y
    };
    println!("statement-function: {}", x);

    // return type (no params)
    let z = five();
    println!("five | {}", z);

    let a = inc(1000);
    println!("1000 + 1 = {}", a);
}

fn response() {
    println!("hi");
}

// take params
fn another_function(x: i32) {
    println!("testing the number {}", x);
}

// multiple params
fn format_question_number(question_number: i64, part: char) {
    println!("do question {}{} right now", question_number, part);
}

// return types
fn five() -> i32 {
    5
}

// return types (with params)
fn inc(x: i64) -> i64 {
    x + 1
}

use std::io;

fn main() {
    // 1. IF/ELSE

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("failed."); // get the input
    let num: u64 = match num.trim().parse() {
        // make sure number is an int
        Ok(a) => a,
        Err(_) => 0,
    };

    if num < 5 {
        println!("the number is indeed, less than 5.");
    } else if num > 5 {
        println!("you failed the first test, but passed the second. how?");
    } else {
        println!("you fool. you absolute moron. you thought this number was greater than 5 or less than 5. the number is indeed, 5.")
    }

    // rust doesn't allow such syntax to check for a value:
    // if num {
    //     println!("this number has a value!");
    // }

    // 2. TERNARY STATEMENTS
    let cond: bool = true;
    // make sure the if  braces and the else braces have the same type
    let n = if cond { 5 } else { 6 };
    println!("value of number is: {}", n);

    // 3. LOOPS
    // forever loop
    // loop {
    //     println!("this will repeat forever! press ctrl+c to cancel this annoying thing")
    // }

    // break & continue - an example
    let mut count = 0;
    'counting_up: loop {
        // give a label so that we know later which loop to break out of
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count: {}", count);

    // returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break and return counter * 2
        }
    };

    println!("The result is {}", result);

    // CONDITIONAL LOOPS - WHILE
    // count up from 3 - 10
    let mut cnt = 3;
    while cnt <= 10 {
        println!("eta: {}", cnt);
        cnt += 1;
    }
    println!("sorry guys i was late because of traffic (yea, totally)");

    // RANGE-BASED FOR LOOPS
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // COUNTING LOOPS
        for number in (3..11).rev() {
        println!("eta: {}", number);
    }
    println!("how did i travel backwards");
}

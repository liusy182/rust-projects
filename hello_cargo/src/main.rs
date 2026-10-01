const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    mutate();
    constant();
    shadow();
    for_loop();
}

fn for_loop() {
    for number in (1..5).into_iter() {
        println!("{number}");
    }
}

fn constant() {
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}

fn mutate() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    let y = x;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

fn shadow() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
fn main() {
    println!("example Mutable");

    let mut x = 10;
    println!("current value = {}", x);

    // Pass 'x' as a mutable reference:
    change_value(&mut x);

    // Print new value of 'x':
    println!("New value of x = {}", x);

    change_value_with(&mut x,11);
    println!("New value of x = {}", x);
}

fn change_value(s: &mut i8) {
    // Dereferencing 's' and changing its value:
    *s = 5;
}

fn change_value_with(s: &mut i8, change_target: i8) {
    // Dereferencing 's' and changing its value:
    *s = change_target;
}

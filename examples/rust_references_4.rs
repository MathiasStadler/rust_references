fn main() {
    // works
    // However, the lifetime of an immutable reference only lasts
    // till its last usage. So, the following code ** won’t **  generate
    // an error because the lifetime of ref1 ends at line 5,
    // i.e., before the mutable ref2 was created.

    //two line
    let mut x = 10;
    let ref1 = &x;

    // one line DOSN'T work
    // let mut y: &i8 = &11;

    //change_value(&ref1);

    read_value(ref1); // Lifetime of ref1 ends here

    let ref2 = &mut x;
    change_value(ref2);

    // Print new value of 'x':
    println!("New value of x = {}", *ref2);

    change_value_with(ref2, 2);
    println!("change_value_with:: New value of x = {}", *ref2);
}

fn read_value(i: &i8) {
    // Use immutable reference 'i':
    println!("x = {}", *i);
}

fn change_value(i: &mut i8) {
    // Use mutable reference 'i':
    *i = 5;
}

fn change_value_with(i: &mut i8, _change: i8) {
    // Use mutable reference 'i':
    *i = _change;
}

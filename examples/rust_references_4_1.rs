fn main() {
    let mut x = 10;

    // works
    // However, the lifetime of an immutable reference only lasts 
    // till its last usage. So, the following code ** won’t **  generate 
    // an error because the lifetime of ref1 ends at line 5,
    // i.e., before the mutable ref2 was created.

    let ref1 = &x;
    //move to 
    // read_value(ref1); // Lifetime of ref1 ends here

    let ref2 = &mut x;

    //here
    read_value(ref1); // Lifetime of ref1 ends here
    change_value(ref2);

    // Print new value of 'x':
    println!("New value of x = {}", *ref2);
}

// don't work
// error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
//   --> examples/rust_references_4_1.rs:14:16
//    |
// 10 |     let ref1 = &x;
//    |                -- immutable borrow occurs here
// ...
// 14 |     let ref2 = &mut x;
//    |                ^^^^^^ mutable borrow occurs here
// ...
// 17 |     read_value(ref1); // Lifetime of ref1 ends here
//    |                ---- immutable borrow later used here


fn read_value(i: &i8) {
    // Use immutable reference 'i':
    println!("x = {}", *i);
}

fn change_value(i: &mut i8) {
    // Use mutable reference 'i':
    *i = 5;
}

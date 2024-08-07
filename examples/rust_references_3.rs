fn main() {
    println!(
        "An immutable reference and a mutable reference to the same variable 
    cannot exist at the same time"
    );


    let mut x= 33;
    let _ref1 = &mut x;
    let _ref2 = &mut x;

    let _ref1 = &x;
    // An immutable reference and a mutable 
    // reference to the same variable 
    // cannot exist at the same time
    // let ref2 = &mut x;

    // Pass immutable ref1 to read():
    read(_ref1);

    fn read(_i: &i8) {
        // Use value of 'x' using immutable reference 'i'
    }
}

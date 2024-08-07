fn main() {
    println!("example Immutable by default");

    let x = 4711;
    // Passing a reference of 'x':
    foo(&x);

    // foo borrows the value of 'x':
    fn foo(a: &i32) {
        // Do something
        println!("{}",a);

        // ERROR: cannot assign to immutable argument
        //a=&815;
    }
}

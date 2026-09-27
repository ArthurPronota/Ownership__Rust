fn take(s: String) {
    println!("s: {}", s) ;
}
fn main() {
    let my_string = "Hello, world!".to_string() ;

    take(my_string);    // Out: s: Hello, world!

    /* use of moved value: `my_string`
    my_string ;
    */
}

// This generates a vector and transfers/moves ownership to the caller
fn listing<'a>() -> Vec<&'a str> {
    "hey,there".split(',').collect()
}
// The string a is "moved" from this function because nothing else attempts to use it after the
// return (last line)
fn handover() -> String {
    let a = String::from("Hello");
    a
}
fn main() {
    // Get mutable ownership of a Vec<&str>
    let mut a = listing();
    // Get ownership of a String
    let b = handover();

    println!("Listing: {:?}", a);
    // We can modify this since we have mutable ownership
    a.push("another");
    println!("Listing: {:?}", a);

    println!("Handed over: {}", b);
}

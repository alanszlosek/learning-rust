fn listing<'a>() -> Vec<&'a str> {
    "hey,there".split(',').collect()
}
fn handover() -> String {
    let a = String::from("Hello");
    a
}
fn main() {
    let mut a = listing();
    let b = handover();

    println!("Listing: {:?}", a);
    a.push("another");
    println!("Listing: {:?}", a);

    println!("Handed over: {}", b);
}

use std::fmt;

// This definition doesn't work:
//    string_reference(&mut a: String)
//    Though not exactly sure why, or what the compiler thinks it means.
fn string_reference(a: &mut String) {
    a.push_str(" are");
    println!("String reference: {}", a);
}

fn tuples() {
    let t1: (u8, u8,  u8) = (1,2,3);
    let t2: (u8, &str) = (1, "a");

    let (a, b, c) = t1;
    println!("{}, {}, {}", a, b, c);

    println!("{:?}", t2);
}


fn strings() {
    let str_immutable = "str types are immutable";
    println!("{}", str_immutable);
    // this won't work: str_immutable = "hey";
    //let mut a: &str = "Hey";
    let mut a = String::from("How");
    
    string_reference(&mut a);
    a.push_str(" you?");
    //a = "Hey There";
    println!("After: {}", a);
}

fn arrays() {
    let hi: [u8; 3] = [7,8,9];
    println!("First element from u8 array: {}", hi[0]);

    let hi = "Whoah";
    println!("Redeclare var of different type (str) and print value: {}", hi);

    
    /*
    let mut input = String::new();
    std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    println!("Typed: {}", input);
    */
}


struct Coordinates {
    x: u8,
    y: u8,
    z: u8
}
impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}
fn structs() {

    let a = Coordinates 
    {
        x: 1,
        y: 2,
        z: 3
    };
    let b = Coordinates {
        x: 100,
        ..a
    };

    println!("Coordinates: {}", a);
    println!("Coordinates: {}", b);
}


fn main() {
    //strings();

    //tuples();

    //arrays();

    //structs();


    println!("1 - 2 = {}", 1i32 - 2);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

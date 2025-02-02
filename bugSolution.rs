fn main() {
    let mut x = 5;
    { // Create a new scope
        let y = &mut x;
        *y += 1;
    }
    { // Create another scope 
        let z = &mut x;
        *z += 1;
    }
    println!("{}", x);
}
//Alternatively, you could clone the data:
//fn main() {
//    let mut x = 5;
//    let mut y = x.clone();
//    let mut z = x.clone();
//    y += 1;
//    z += 1;
//    println!("{}", x + y + z);
//}
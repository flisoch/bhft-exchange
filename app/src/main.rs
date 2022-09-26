
fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);

    for num in &mut v {
        *num += 1;
        println!("{num}");
    }
    
    let six = &v[1];
    
    println!("{six}");
    
    
}

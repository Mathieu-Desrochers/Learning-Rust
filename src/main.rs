fn ownership_move(_x: Vec<i32>) {
}

fn ownership() {

    // each value has a single owner

    // assignation moves ownership
    // x1 is left uninitialized
    // let z1 = x1 would not compile
    let x1: Vec<i32> = Vec::new();
    let _y1 = x1;

    // parameter passing also moves ownership
    // x2 is left uninitialized
    // let z2 = x2 would not compile
    let x2: Vec<i32> = Vec::new();
    ownership_move(x2);

    // copiable types are an exception
    let x3 = 25;
    let _y3 = x3;
    let _z3 = x3;

    // cannot move value out of container
    // let y4 = x4[0] would not compile
    let mut x4 = Vec::new();
    x4.push("a".to_string());
    x4.push("b".to_string());

    // can move value out of Option though
    // x5[0] is left as None
    let mut x5 = Vec::new();
    x5.push(Some("a".to_string()));
    x5.push(Some("b".to_string()));
    let _y5 = x5[0].take();

}

fn main() {
    ownership();
}

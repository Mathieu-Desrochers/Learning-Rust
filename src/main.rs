use std::collections::HashMap;

fn ownership_move(_x: Vec<i32>) {}

fn ownership() {
    // each value has a single owner

    // assignation moves ownership
    // x1 is left uninitialized
    // let z1 = x1 will not compile
    let x1: Vec<i32> = Vec::new();
    let _y1 = x1;

    // passing a parameter also moves ownership
    // x2 is left uninitialized
    // let z2 = x2 will not compile
    let x2: Vec<i32> = Vec::new();
    ownership_move(x2);

    // copiable types are an exception
    let x3 = 1;
    let _y3 = x3;
    let _z3 = x3;

    // cannot move value out of its owner
    // let y4 = x4[0] will not compile
    let mut x4 = Vec::new();
    x4.push("a".to_string());
    x4.push("b".to_string());

    // can move value out of Option though
    // x5[0] is left as None
    let mut x5 = Vec::new();
    x5.push(Some("a".to_string()));
    x5.push(Some("b".to_string()));
    let _y5 = x5[0].take();

    // loops move ownership of a
    // container and their elements
    // let _z6 = x6 will not compile
    // let _z6 = x6[0] will not compile
    let mut x6 = Vec::new();
    x6.push(Some("a".to_string()));
    x6.push(Some("b".to_string()));
    for _y6 in x6 {}
}

fn references_shared(x: &HashMap<String, String>) {
    // loops on references do not move ownership
    // let _y = x will compile
    for (_code, _name) in x {}
}

fn references_mutable(_x: &mut HashMap<String, String>) {}

fn references() {
    // values can be shared with references
    // without moving ownership

    // passing a reference parameter does not move ownership
    // let _y1 = x1 will compile
    let mut x1: HashMap<String, String> = HashMap::new();
    x1.insert("LAX".to_string(), "Los Angeles".to_string());
    x1.insert("YUL".to_string(), "Montréal".to_string());
    references_shared(&x1);
    references_mutable(&mut x1);

    // the dot operator implicitly deferences
    // so do arithmetic and comparaison operators
    struct Airplane {
        callcode: String,
        _color: String,
    };
    let x2 = Airplane {
        callcode: "Bandit".to_string(),
        _color: "Black".to_string(),
    };
    let x2_r = &x2;

    // these are the same
    // this is great for the same code
    // works on values and on references
    let _y2_a = &((*x2_r).callcode);
    let _y2_b = &(x2.callcode);
    let _y2_c = &(x2_r.callcode);

    // the dot operator implicitly gets a reference
    // to the left operand if needed
    let mut x3 = vec!["B".to_string(), "A".to_string()];

    // these are the same
    (&mut x3).sort();
    x3.sort();

    // can get reference to any expression
    // it gets an anonymous variable for as long as needed
    let x4 = &(1 + 2);
    let _y4 = x4 + &3;
}

fn main() {
    ownership();
    references();
}

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
    // this is great since the same code
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
    // it gets an anonymous variable
    let x4 = &(1 + 2);
    let _y4 = x4 + &3;
}

fn lifetimes_basics() {
    // references must have shorter lifetimes
    // than the values they point to
    let _reference: &i32;
    {
        let _value = 1;
        // this will not compile
        //_reference = &_value;
    }
}

// lifetimes can be declared on functions
// here is what is implied when nothing is spelled out
fn _lifetimes_explicit<'a, 'b, 'c>(_x: &'a Vec<i32>, _y: &'b i32) -> &'c i32 {
    return &0;
}

// when a function returns a reference
// and there is a single reference argument
// rust assumes they have the same lifetimes
fn _lifetimes_explicit_simple_case<'a>(x: &'a Vec<i32>) -> &'a i32 {
    return &x[0];
}

// this can be used to bind the lifetime of a function's result
// to the lifetimes of its parameters
fn lifetimes_bound<'a, 'b>(x: &'a Vec<i32>, y: &'b usize) -> &'a i32 {
    return &x[*y];
}

fn lifetimes() {
    lifetimes_basics();

    // vector must live as long as result
    // since result points to one of its elements
    // we expressed this with 'a
    let vector = vec![1, 2, 3];
    let _result;
    {
        // index must live only for the function call
        // we do not care otherwise
        let index: usize = 0;
        _result = lifetimes_bound(&vector, &index);
    }
}

fn main() {
    ownership();
    references();
    lifetimes();
}

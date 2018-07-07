use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

fn ownership_move(_x: Vec<i32>) {}

fn ownership() {
    // each value has a single owner

    // assignation moves ownership
    // x1 is left uninitialized
    let x1: Vec<i32> = Vec::new();
    let _y1 = x1;
    // COMPILE ERROR: let z1 = x1;

    // passing a parameter also moves ownership
    // x2 is left uninitialized
    let x2: Vec<i32> = Vec::new();
    ownership_move(x2);
    // COMPILE ERROR: let z2 = x2;

    // copiable types are an exception
    let x3 = 1;
    let _y3 = x3;
    let _z3 = x3;

    // cannot move value out of its owner
    let mut x4 = Vec::new();
    x4.push("a".to_string());
    x4.push("b".to_string());
    // COMPILE ERROR: let y4 = x4[0];

    // can move value out of Option though
    // x5[0] is left as None
    let mut x5 = Vec::new();
    x5.push(Some("a".to_string()));
    x5.push(Some("b".to_string()));
    let _y5 = x5[0].take();

    // loops move ownership of a
    // container and their elements
    let mut x6 = Vec::new();
    x6.push(Some("a".to_string()));
    x6.push(Some("b".to_string()));
    for _y6 in x6 {}
    // COMPILE ERROR: let _z6 = x6;
    // COMPILE ERROR: let _z6 = x6[0];
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
    let mut x1: HashMap<String, String> = HashMap::new();
    x1.insert("LAX".to_string(), "Los Angeles".to_string());
    x1.insert("YUL".to_string(), "Montréal".to_string());
    references_shared(&x1);
    references_mutable(&mut x1);

    // all good
    let _y1 = x1;

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
    // this is great because the same code
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
        // COMPILE ERROR: _reference = &_value;
    }
}

// lifetimes can be declared on functions
// here is what is implied when nothing is spelled out
fn _lifetimes_explicit<'a, 'b, 'c>(_x: &'a Vec<i32>, _y: &'b i32) -> &'c i32 {
    return &0;
}

// when a function returns a reference
// and takes only one reference parameter
// rust assumes they have the same lifetimes
fn _lifetimes_explicit_simple_case<'a>(x: &'a Vec<i32>) -> &'a i32 {
    return &x[0];
}

// this declaration limits the lifetime of the function's result
// to the lifetime of its first parameter
fn lifetimes_bound<'a, 'b>(x: &'a Vec<i32>, y: &'b usize) -> &'a i32 {
    return &x[*y];
}

fn lifetimes() {
    lifetimes_basics();

    // result cannot live longer than vector
    // since it points to one of vector's elements
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

fn lifetimes_structs() {
    // lifetimes must be declared inside structs
    // this limits the lifetime of the struct
    // to the lifetimes of its references
    struct Piano<'a> {
        _keys: &'a i32,
    };

    // piano cannot live longer than keys
    // since it has a reference to keys
    // we expressed this with 'a
    let keys = 64;
    let _piano = Piano { _keys: &keys };
}

fn mutability() {
    // values are immutable by default
    let _x1 = 25;
    // COMPILE ERROR: _x1 = 26;

    // immutability extends inside the variable
    let _x2 = vec![1, 2, 3];
    // COMPILE ERROR: _x2.push(4);

    // tree structure to
    // illustrate next points
    struct Leaf {
        _value: i32,
    };
    struct Branch {
        left: Leaf,
        right: Leaf,
    };
    struct Root {
        left: Branch,
        right: Branch,
    };

    let mut root = Root {
        left: Branch {
            left: Leaf { _value: 1 },
            right: Leaf { _value: 2 },
        },
        right: Branch {
            left: Leaf { _value: 3 },
            right: Leaf { _value: 4 },
        },
    };

    // multiple read references are allowed
    // inside the same ownership tree
    {
        let _root_left = &root.left;
        let _root_left_left = &root.left.left;
    }

    // a read reference in the tree makes the children
    // and the ancestors immutable
    // we looking; no touchy
    {
        let _root_left = &root.left;

        // COMPILE ERRORS:
        // let _root_left_left = &mut root.left.left;
        // let _root = &mut root;

        // other parts of the tree are free game
        let _root_right = &mut root.right;
    }

    // a mutable reference in the tree makes the children
    // accessible only through that reference
    // and the ancestors inaccessible
    // we touchy, no looking
    {
        let root_left = &mut root.left;

        // COMPILE ERRORS:
        // let _root_left_left = &root.left.left;
        // let _root = &root;

        // other parts of the tree are free game
        let _root_right = &root.right;

        // children are accessible through
        // the mutable reference
        let _root_left_left = &root_left.left;
        let _root_left_right = &mut root_left.right;
    }
}

fn errors_success() -> Result<i32, String> {
    Ok(1)
}

fn _errors_failure() -> Result<i32, String> {
    Err("Uh oh".to_string())
}

fn errors() {
    // results must be consumed
    // errors_success() will generate a warning
    let _x1 = match errors_success() {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // this often is reduced to one
    // character using the ? operator
    fn propagation() -> Result<i32, String> {
        // either gets the success value
        // or propagates the error to the caller
        let y = errors_success()?;
        Ok(y + 1)
    };

    // ignore a result
    let _ = propagation();

    // cause a panic on error
    // better be real sure
    let _x2 = errors_success().unwrap();
    let _x3 = errors_success().expect("Ugh");
}

fn structs() {
    // named-field
    #[allow(dead_code)]
    struct Pizza {
        size: u8,
        with_bacon: bool,
    }

    let small = Pizza {
        size: 8,
        with_bacon: true,
    };

    // populate fields from local variables
    // having the same name
    fn _medium(with_bacon: bool) -> Pizza {
        Pizza {
            size: 12,
            with_bacon,
        }
    }

    // populate fields from another instance
    let _large = Pizza { size: 16, ..small };

    // tuple-like
    struct RgbColor(u8, u8, u8);
    let black = RgbColor(0, 0, 0);
    let _red = black.0;
    let _green = black.1;
    let _blue = black.2;

    // unit-like
    struct Square;
    let _square = Square;
}

fn associated_functions() {
    struct _Dog {
        is_good_boy: bool,
    }

    impl _Dog {
        fn _bark(&self) -> String {
            "ruf".to_string()
        }
    }
}

fn enums() {
    #[allow(dead_code)]
    enum Speed {
        Slow,
        TooFast,
    }

    // with tuple data
    #[allow(dead_code)]
    enum Shape {
        Circle(u32),
        Rectangle(u32, u32),
    }

    // with struct data
    #[allow(dead_code)]
    enum Monster {
        Rat { rabid: bool },
        Vampire { blood_thristy: bool, bat_form: bool },
    }

    // matching patterns
    let x1 = Speed::Slow;
    let _y1 = match x1 {
        Speed::Slow => "All good",
        Speed::TooFast => "Slow down you crazy",
    };

    // with multiple values
    let x2 = 100;
    let _y2 = match x2 {
        0...100 => "Small",
        101 | 102 | 103 => "Medium",
        _ => "Big",
    };

    // with guards
    let x3 = Shape::Rectangle(10, 20);
    let _y3 = match x3 {
        Shape::Rectangle(h, _) if h > 100 => "You a tall rectangle",
        Shape::Rectangle(_, _) => "You an ok rectangle",
        _ => "You no rectangle at all",
    };

    // with fields
    let x4 = Monster::Vampire {
        blood_thristy: true,
        bat_form: false,
    };
    let _y4 = match x4 {
        Monster::Rat { .. } => "Squishy",
        Monster::Vampire {
            blood_thristy: true,
            ..
        } => "We are sooo done...",
        Monster::Vampire { .. } => "Keep calm",
    };

    // with borrowed references
    let _z4 = match x4 {
        Monster::Rat { ref rabid } => *rabid,
        _ => false,
    };

    // with no unpacking
    let x5 = Shape::Rectangle(10, 20);
    let _y5 = match x5 {
        circle @ Shape::Circle(_) => circle,
        rectangle @ Shape::Rectangle(_, _) => rectangle,
    };

    // testing a single pattern
    if let Shape::Circle(r) = x3 {
        let _area = 3.14 * (r as f32) * (r as f32);
    }
}

fn traits() {
    trait Eatable {
        fn eat_it(&self);

        // provide default implementation
        fn chump_it(&self, _speed: u8) {
            self.eat_it();
        }
    }

    // adding traits to types
    impl Eatable for i32 {
        fn eat_it(&self) {}
    }

    impl Eatable for f64 {
        fn eat_it(&self) {}
    }

    // trait objects
    let x1: &Eatable = &12;
    fn have_quick_snack(_eatable: &Eatable) {}
    have_quick_snack(x1);

    // trait objects add runtime overhead
    // the type of eatable could be i32 or f64
    // it is unknown a compile time for public functions

    // but traits can be mixed and matched inside boxes
    let mut x2: Vec<Box<Eatable>> = Vec::new();
    x2.push(Box::new(12));
    x2.push(Box::new(3.456));

    // fully qualified trait method
    Eatable::eat_it(&9.999);

    // generics can be bound to traits
    fn have_lunch<E: Eatable>(_eatables: Vec<E>) {}
    have_lunch(vec![1, 6, 2]);

    // generics make the compiled code bigger
    // different versions of have_lunch are compiled
    // one for each E used by the program

    // bounds can be a combinaison of traits
    fn _eat_debugable<E: Eatable + Debug>(_eatables: Vec<E>) {}
    fn _eat_hashable<E>(_eatables: Vec<E>)
    where
        E: Eatable + Hash,
    {
    }
}

#[test]
fn unit_test() {
    assert_eq!(1, 1);
}

/// Actually run by cargo test for library crates.
///
///     assert_eq!(documentation(), "You bet");
pub fn documentation() -> String {
    "You bet".to_string()
}

fn main() {
    ownership();
    references();
    lifetimes();
    lifetimes_structs();
    mutability();
    errors();
    structs();
    associated_functions();
    enums();
    traits();
}

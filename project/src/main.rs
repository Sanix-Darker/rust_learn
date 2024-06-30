// common types
type INT = i32;

fn print_format_name(name: String){
    println!("Hello '{}'!", name);
}

fn type_of<T>(_: &T) -> String {
    let type_name = std::any::type_name::<T>();
    return format!("{}", type_name);
}

fn ignore_warning_from_unused_vars(){
    // 1- by adding the underscore
    // 2- by adding a 'decorator' for that
    let _x = 12;
}

#[allow(dead_code)]
fn no_void() -> () {
    println!("ok");
}

fn match_(val: i8) -> i8 {
    match val {
        3 => {
            println!("is 3 !");
            10_i8
        }
        _ => {
            println!("Something else.");
            22_i8
        }
    }
}

fn time_goes(age: u16) -> u16{
    let newage = age + 1;

    // this will return
    // we could also do 'return newage;'
    newage
}

fn destructured_vars(){
    let (mut a, b) = (String::from("acid"), 100);
    a = format!("{}{}", a ,"eee");
    println!("a={} | b={}", a, b);

    let (x, ..) = (12, 11);
    let [.., y] = ["a", "b"];

    assert_eq!(x, 12);
    assert_eq!(y, "b");
    println!("Hello {} {} !", y, x);
}

fn separator(rait: INT){
    for _ in 0..rait{
        print!("-");
    }
    println!("\n");
}

fn multiple_type_in_array(){

    // let x:u16 = 12_u8 as u16;

    let mut arr: Vec<Box<dyn std::any::Any>> = Vec::new();

    arr.push(Box::new(42));
    arr.push(Box::new("hello"));

    separator(12);

    for item in &arr {
        if let Some(i) = item.downcast_ref::<i32>() {
            println!("i32: {}", i);
        } else if let Some(s) = item.downcast_ref::<&str>() {
            println!("&str: {}", s);
        }
    }
}

fn main() {
    //  Variable that is going to be used
    //  should be initialised with a value
    let name: &str = "sanix";
    // For assertions :
    // assert_eq!(name, "dk");
    print_format_name(
        name.to_string()
    );

    let mut vector = vec![];
    vector.push("a");

    let hobbies: [&str; 2] = ["draw", "code"];
    println!("as hobbies: {:?}", hobbies);
    // we can also shadowing variable by redeclared them
    // We can shadowing with a value from another data type.
    //
    // let hobbies: [&str; 1] = ["draw"];
    // println!("as hobbies: {:?}", hobbies);

    // custom isolated scope.
    {
        // A variable that will be changed, need
        // to be set with the 'mut' attr.
        let mut age: u16 = 30;
        // note ux and iy for unsigned and signed
        age = time_goes(age);

        println!("You have {} years old.", age);
    }

    // How to ignore unsuded vars:
    ignore_warning_from_unused_vars();

    // how to destructure vars from tuple
    destructured_vars();

    // a multiple type in array
    multiple_type_in_array(  );

    // to get the type of a value
    println!("> type of hobbies is '{}'", type_of(&hobbies));

    // ranges
    // ..= (means inclusive)
    // .. (is not inclusive)
    let range = 5..10;
    for i in range {
        println!("Hello {}", i);
    }
    // loop on a -> Z
    for c in 'a'..='d' {
        println!("> {}", c);
    }

    // defined type integers
    let _xyz: u8 = 10_u8; // means it is unsigned
    // println!("{}", (-12 + xyz)); // will raise since -12 is a signed (belongs to negative values).

    // {} <- value
    // {:?} <- value with type

    // chars
    let xx: char = 'a';
    println!("{:?}", xx);

    let uni: (u8, &str) = (12, "ok");
    println!("{:?}", uni);

    let mut arr = vec![];
    arr.push("x");
    arr.push("13");
    // difference between push and append
    // push add a new item;
    // append add a vector to the other one;
    println!("{:?}", arr);

    // Mutiline expressions
    let zeta: () = {
        let x = 12_u16;
        let y = 30_u16;

        // I can either add a return
        // statement or just leave as it
        // (WITHOUT the semicolomn ;)
        // it will assing the computed value to
        //
        // zeta.
        let result = x * y - (x^y);
        println!("{:?}", result);
    };

    println!("zeta={:?}", zeta);
    println!("{:?}", match_(3));

    ownership();
}

fn ownership(){

    let a = String::from("1");
    // i have to use .clone() because i will use
    // a later and the new owner of it is owner method.
    println!("{:?}", owner(a.clone()));

    println!("{}", a);
}

fn owner(z: String) -> (String, String, String) {
    let x = String::from("this");
    let y = x.clone();
    // x = String::from("doum");

    (y, x, z)
}

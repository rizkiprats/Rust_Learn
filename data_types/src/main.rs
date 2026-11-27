#[path="scalar_types.rs"]
mod scalar_types_module;

fn main() {
    println!("==== Data Types: Scalar ====");
    let ints = scalar_types_module::IntegerTypes::default();
    let floats = scalar_types_module::FloatingPointTypes::default();
    let nums = scalar_types_module::NumberTypes::default();
    let boolean = scalar_types_module::BooleanTypes::default();

    println!("{}\n", ints);
    println!("{}\n", floats);
    println!("{}\n", nums);
    println!("{}", boolean)
}

pub fn export_main(){
    main();
}

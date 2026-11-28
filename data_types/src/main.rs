#[path="scalar_types.rs"]
mod scalar_types_module;

fn main() {
    println!("==== Data Types: Scalar ====");
    let ints = scalar_types_module::IntegerType::default();
    let floats = scalar_types_module::FloatingPointType::default();
    let nums = scalar_types_module::NumberType::default();
    let boolean = scalar_types_module::BooleanType::default();
    let character = scalar_types_module::CharacterType::default();

    println!("{}\n", ints);
    println!("{}\n", floats);
    println!("{}\n", nums);
    println!("{}\n", boolean);
    println!("{}", character)
}

pub fn export_main(){
    main();
}

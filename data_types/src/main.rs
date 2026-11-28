#[path="scalar_types.rs"]
mod scalar_types_module;

#[path="compound_types.rs"]
mod compound_types_module;

fn main() {
    println!("==== Data Types: Scalar Types ====");
    let ints = scalar_types_module::IntegerType::default();
    let floats = scalar_types_module::FloatingPointType::default();
    let nums = scalar_types_module::NumberType::default();
    let boolean = scalar_types_module::BooleanType::default();
    let character = scalar_types_module::CharacterType::default();

    println!("{}\n", ints);
    println!("{}\n", floats);
    println!("{}\n", nums);
    println!("{}\n", boolean);
    println!("{}\n", character);

    println!("==== Data Types: Compound Types ====");
    let tuple = compound_types_module::TupleType::default();
    let array=compound_types_module::ArrayType::default();
    
    println!("{}\n", tuple);
    println!("{}\n", array);

}

pub fn export_main(){
    main();
}

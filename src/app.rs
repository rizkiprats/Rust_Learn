// call hello world script as a module
#[path = "hello_world/hello_world.rs"]
mod hello_world_module;

// call hello cargo lib as a module
use hello_cargo;

fn main(){
    // call hello world function from hello world module
    hello_world_module::hello_world();

    // call hello cargo test function from hello cargo lib
    hello_cargo::test();
}
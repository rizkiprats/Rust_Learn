// call hello world script as a module
#[path = "../hello_world/hello_world.rs"]
mod hello_world_module;

fn main(){
    // call hello world function from hello world module
    hello_world_module::hello_world();
}
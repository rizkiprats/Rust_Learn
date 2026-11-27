// call inquire lib to help create interactive selection on terminals with error handler
use inquire::{InquireError, Select};
// call strum, fmt lib for help displaying and formatting the enum
use strum::{Display, EnumIter, IntoEnumIterator};
use std::fmt;

// call hello world script as a module
#[path = "hello_world/hello_world.rs"]
mod hello_world_module;
// call hello cargo lib
use hello_cargo;
//call guessing game lib
use guessing_game;
//call variables declaration lib
use variables;
//call data types lib
use data_types;

// create enum for selection menu
#[derive(Debug, Clone, Copy, Display, EnumIter)]
enum Selection {
    #[strum(serialize = "Hello World Scipt")]
    HelloWorldScript,

    #[strum(serialize = "Hello Cargo Lib")]
    HelloCargoLib,

    #[strum(serialize = "Guessing Game")]
    GuessingGame,

    #[strum(serialize = "Variables Declaration")]
    VariablesDeclaration,

    #[strum(serialize = "Data Types")]
    DataTypes,

    #[strum(serialize = "Exit")]
    Exit,
}

//create enum for after selection menu
#[derive(Debug, Clone, Copy, Display, EnumIter)]
enum AfterSelection {
    #[strum(serialize = "Back To Menu")]
    Back,

    #[strum(serialize = "Exit")]
    Exit,
}

// create a struct for helping numbering the enum
struct OptionNumber<T> {
    number: usize,
    item: T,
}

// a struct implementation for format displaying enum and it's number
impl<T: fmt::Display> fmt::Display for OptionNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}. {}", self.number, self.item)
    }
}

// Flexible function for show selection with validation input from enum and it's number
fn show_select_validation<T>(title: &str) -> Result<T, InquireError>
where
    T: IntoEnumIterator + fmt::Display + Copy,
{
    //create a number iteration for selection list
    let list: Vec<OptionNumber<T>> = T::iter()
        .enumerate()
        .map(|(i, item)| OptionNumber { number: i + 1, item })
        .collect();

    //create interactive selection list with number in terminal and validating the input 
    let selection = Select::new(title, list)
        .with_vim_mode(false)
        .with_help_message("Type a number only!")
        .with_scorer(&|input, _option, string_value, _idx| {
            let input_typing = input.trim();
            if input_typing.is_empty() {
                return Some(0);
            }
    
            if !input_typing.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }
            
            if string_value.starts_with(&format!("{}.", input_typing)) {
                Some(100)
            } else {
                None
            }
        })
        .prompt()?;

    Ok(selection.item)
}

// Error handler
fn error_handler(err: InquireError) {
    match err {
        InquireError::OperationCanceled => {
            eprintln!("\nMenu canceled (Esc).");
        }
        InquireError::OperationInterrupted => {
            eprintln!("\nMenu interrupted (Ctrl+C).");
        }
        InquireError::IO(io_err) => {
            eprintln!("\nFailed to read input: {}", io_err);
        }
        other => {
            eprintln!("\nSystem failed: {}", other);
        }
    }
}

// Helper function to ask Back or Exit after running a feature
fn offer_after_selection() -> bool {
    //create a interactive after selection menu
    match show_select_validation::<AfterSelection>("Back to menu ? (move up & down or type a number):") {
        Ok(AfterSelection::Back) => true,   // Continue main loop
        Ok(AfterSelection::Exit) => false, // Break main loop
        Err(err) => {
            error_handler(err);
            false
        }
    }
}

fn main(){
    loop {
        println!(); // Give a blank line for cleaner spacing

        //create a interactive selection menu
        match show_select_validation::<Selection>("Select you want to run (move up & down or type a number):") {
            Ok(Selection::HelloWorldScript) => {
                println!("Running Hello World...");
                hello_world_module::hello_world();

                // call after selection helper
                if !offer_after_selection() {
                    println!("See you next time!");
                    break;
                };
            }
            Ok(Selection::HelloCargoLib) => {
                println!("Running Hello Cargo Lib Test...");
                hello_cargo::test();

                // call after selection helper
                if !offer_after_selection() {
                    println!("See you next time!");
                    break;
                };
            }
            Ok(Selection::GuessingGame) => {
                println!("Running Guessing Game...");
                guessing_game::test();

                // call after selection helper
                if !offer_after_selection() {
                    println!("See you next time!");
                    break;
                };
            }
            Ok(Selection::VariablesDeclaration) => {
                println!("Running Variables Declaration Test...");
                variables::test();

                // call after selection helper
                if !offer_after_selection() {
                    println!("See you next time!");
                    break;
                };
            }
            Ok(Selection::DataTypes) => {
                println!("Data Type Test...");
                data_types::test();

                // call after selection helper
                if !offer_after_selection() {
                    println!("See you next time!");
                    break;
                };
            }
            Ok(Selection::Exit) => {
                println!("See you next time!");
                break;
            }
            Err(err) => {
                error_handler(err);
                break;
            }
        }
    }
}
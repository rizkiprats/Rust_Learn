use std::{fmt, io};

pub struct TupleType {
    pub tuple: (i32, f64, u8)
}

impl Default for TupleType {
    fn default() -> Self {
        Self {
            tuple: (500, 6.4, 1)
        }
    }
}

impl fmt::Display for TupleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Tuple Type ===")?;
        let (x,y,z) = self.tuple;
        writeln!(f,   "(x, y, z) : {:?}", (x, y, z))?;
        writeln!(f,   "(y) : {:?}", (y))?;
        writeln!(f,   "(i32, f64, u8) : {:?}", self.tuple)?;
        writeln!(f,   "(i32) : {}", self.tuple.0)?;
        writeln!(f,   "(f64) : {}", self.tuple.1)?;
        write!(f,   "(u8) : {}", self.tuple.2)
    }
}

pub struct ArrayType {
    pub array_i32_5: [i32; 5],
    pub array_string_12: [&'static str; 12],
    pub array_i32_flex: Vec<i32>,
    pub array_string_flex: Vec<String>
}

impl Default for ArrayType {
    fn default() -> Self {
        Self { 
            array_i32_5: [1, 2, 3, 4, 5],
            array_string_12: ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"],
            array_i32_flex: [1, 2, 3, 4, 5].map(i32::from).to_vec(),
            array_string_flex: ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"].map(String::from).to_vec()
        }
    }
}

impl fmt::Display for ArrayType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Array Type ===")?;
        writeln!(f, "[i32; 5] : {:?}", self.array_i32_5)?;
        let a: [i32; 5] = self.array_i32_5;
        writeln!(f, "a [i32; 5] : {:?}", a)?;
        writeln!(f, "a[first] : {:?}", a[0])?;
        writeln!(f, "a[second] : {:?}", a[1])?;
        let b = [3; 5];
        writeln!(f, "b [3; 5] : {:?}", b)?;
        writeln!(f, "[&static str; 12] : {:?}", self.array_string_12)?;
        writeln!(f, "Vec<i32> : {:?}", self.array_i32_flex)?;
        writeln!(f, "Vec<String> : {:?}\n", self.array_string_flex)?;
        
        writeln!(f, "=== Array Element Access  ===")?;
        writeln!(f, "array: {:?}", a)?;
        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        writeln!(f,"The value of the element at index {:?} is {:?}:", index, element)
    }
}


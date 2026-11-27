use std::fmt;

pub struct IntegerTypes {
    //integer types
    pub i_8: i8,   //i8 is a signed 8-bit integer, range: -128 to 127
    pub u_8: u8,   //u8 is an unsigned 8-bit integer, range: 0 to 255

    pub i_16: i16, //i16 is a signed 16-bit integer, range: -32768 to 32767
    pub u_16: u16, //u16 is an unsigned 16-bit integer, range: 0 to 65535

    pub i_32: i32, //i32 is a signed 32-bit integer, range: -2147483648 to 2147483647
    pub u_32: u32, //u32 is an unsigned 32-bit integer, range: 0 to 4294967295

    pub i_64: i64, //i64 is a signed 64-bit integer, range: -9223372036854775808 to 9223372036854775807
    pub u_64: u64, //u64 is an unsigned 64-bit integer, range: 0 to 18446744073709551615

    pub i_128: i128, //i128 is a signed 128-bit integer, range: -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
    pub u_128: u128, //u128 is an unsigned 128-bit integer, range: 0 to 340282366920938463463374607431768211455

    pub integer_lateral_decimal: i32, //decimal
    pub integer_lateral_hex: i32,     //hexadecimal
    pub integer_lateral_octal: i32,   //octal
    pub integer_lateral_binary: i32,  //binary
    pub integer_lateral_byte: u8,     //byte (u8 only)
}

impl Default for IntegerTypes {
    fn default() -> Self {
        Self {
            //integer types
            i_8: -128, //i8 is a signed 8-bit integer, range: -128 to 127
            u_8: 255,  //u8 is an unsigned 8-bit integer, range: 0 to 255

            i_16: -32768, //i16 is a signed 16-bit integer, range: -32768 to 32767
            u_16: 65535,  //u16 is an unsigned 16-bit integer, range: 0 to 65535

            i_32: -2147483648, //i32 is a signed 32-bit integer, range: -2147483648 to 2147483647
            u_32: 4294967295,  //u32 is an unsigned 32-bit integer, range: 0 to 4294967295

            i_64: -9223372036854775808,  //i64 is a signed 64-bit integer, range: -9223372036854775808 to 9223372036854775807
            u_64: 18446744073709551615,  //u64 is an unsigned 64-bit integer, range: 0 to 18446744073709551615

            i_128: -170141183460469231731687303715884105728, //i128 is a signed 128-bit integer, range: -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
            u_128: 340282366920938463463374607431768211455,  //u128 is an unsigned 128-bit integer, range: 0 to 340282366920938463463374607431768211455

            integer_lateral_decimal: 98_222,    //decimal
            integer_lateral_hex: 0xff,          //hexadecimal
            integer_lateral_octal: 0o77,        //octal
            integer_lateral_binary: 0b1111_0000,//binary
            integer_lateral_byte: b'A',         //byte (u8 only)
        }
    }
}

impl fmt::Display for IntegerTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Integer Types ===")?;
        writeln!(f, "i8   : {} | u8   : {}", self.i_8, self.u_8)?;
        writeln!(f, "i16  : {} | u16  : {}", self.i_16, self.u_16)?;
        writeln!(f, "i32  : {} | u32  : {}", self.i_32, self.u_32)?;
        writeln!(f, "i64  : {} | u64  : {}", self.i_64, self.u_64)?;
        writeln!(f, "i128 : {} | u128 : {}", self.i_128, self.u_128)?;
        writeln!(f, "98_222 (Decimal_ : {}", self.integer_lateral_decimal)?;
        writeln!(f, "Hex     : {} (0x{:x})", self.integer_lateral_hex, self.integer_lateral_hex)?;
        writeln!(f, "Octal   : {} (0o{:o})", self.integer_lateral_octal, self.integer_lateral_octal)?;
        writeln!(f, "Binary  : {} (0b{:b})", self.integer_lateral_binary, self.integer_lateral_binary)?;
        write!(f,   "Byte    : {} ('{}')", self.integer_lateral_byte, self.integer_lateral_byte as char)
    }
}

pub struct FloatingPointTypes {
    //floating point types
    pub f_32: f32, //f32 is a 32-bit floating point number
    pub f_64: f64, //f64 is a 64-bit floating point number
}

impl Default for FloatingPointTypes {
    fn default() -> Self {
        Self {
            //floating point types
            f_32: 3.14,              //f32 is a 32-bit floating point number
            f_64: 2.718281828459045, //f64 is a 64-bit floating point number
        }
    }
}

impl fmt::Display for FloatingPointTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Floating Point Types ===")?;
        writeln!(f, "f32 : {}", self.f_32)?;
        write!(f,   "f64 : {}", self.f_64)
    }
}

pub struct NumberTypes {
    // addition
    pub sum: i32,
    // subtraction
    pub difference: f64,
    // multiplication
    pub product: i32,
    // division
    pub quotient: f64,
    pub truncated: i32, // Results in -1
    // remainder
    pub remainder: i32,
}

impl Default for NumberTypes {
    fn default() -> Self {
        Self {
            // addition
            sum: 5 + 10,
            // subtraction
            difference: 95.5 - 4.3,
            // multiplication
            product: 4 * 30,
            // division
            quotient: 56.7 / 32.2,
            truncated: -5 / 3, // Results in -1
            // remainder
            remainder: 43 % 5,
        }
    }
}

impl fmt::Display for NumberTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Number Types Operations ===")?;
        writeln!(f, "Sum (5 + 10)          : {}", self.sum)?;
        writeln!(f, "Difference (95.5-4.3) : {:.2}", self.difference)?;
        writeln!(f, "Product (4 * 30)      : {}", self.product)?;
        writeln!(f, "Quotient (56.7 / 32.2): {:.4}", self.quotient)?;
        writeln!(f, "Truncated (-5 / 3)    : {}", self.truncated)?;
        write!(f,   "Remainder (43 % 5)    : {}", self.remainder)
    }
}

pub struct BooleanTypes {
    pub boolean: bool // bool values is true or false
}

impl Default for BooleanTypes {
    fn default() -> Self {
        Self{
            boolean: true
        }
    }
}

impl fmt::Display for BooleanTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Boolean Types ===")?;
        write!(f, "Boolean : {} | {}", self.boolean, !self.boolean)
    }
}
/*
"Result is a version of the Option type that describes a possible Err instead of None".

Create a structure called Flag which has the following elements:

short_hand: String
long_hand: String
desc: String
This structure must have an associated function called opt_flag which initializes the structure.
This function receives two string references and returns a structure Flag. Here is an example of its usage:

let d = Flag::opt_flag("diff", "gives the difference between two numbers");

println!("short hand: {}, long hand: {}, description: {}", d.short_hand, d.long_hand, d.desc);
// output: "short hand: -d, long hand: --diff, description: gives the difference between two numbers"
An associated type called Callback will also be provided.
It should represent a function pointer which is going to be used in the structure and functions below.
This function will represent the callback for the flag associated to it.

A second structure named FlagsHandler will be given which just has one element: flags: HashMap<(String, String), Callback>.
You'll also need to implement the following associated functions:

add_flag, which adds the flag and callback function to the HashMap.
exec_func, which executes the function using the flag provided and returns the result.
The callback should be executed with the first two arguments of the supplied argv argument. Return either the successful result from the callback or the error stringified.
You will have to create the following callback functions:

div: which converts the reference strings to f64s and returns the Result, as the division of these floats or the error ParseFloatError.
rem: which converts the reference strings to f64s and returns the Result, as the remainder of the division of these floats or the error ParseFloatError.

*/
use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    // expected public fields
    pub short_hand: String, // short option
    pub long_hand: String, // long option
    pub desc: String // description of the flags
}

impl Flag {
    /// `name` is the name of the flag
    /// `d` is the description of the flag
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Flag{
            // short we take the first character of the long one
            short_hand: format!("-{}",name.chars().next().unwrap()),
            long_hand: format!("--{name}"),
            desc:String::from(d),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand,func);
        self.flags.insert(flag.long_hand,func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let (res) = self.flags.get(input);
        match res {
            // Ok(func) => func,
            Some(func) => {
                let result  = func(argv[0], argv[1]);
                match result {
                    Ok(res) => {
                        Ok(res)
                    },
                    Err(err) => {
                        Err(String::from("invalid float literal"))
                    }
                }
            },
            None=> Err(String::from("he")),
        }
    }
}

// division
pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let aa :f64 = String::from(a).parse::<f64>()?;
    let bb : f64 = String::from(b).parse::<f64>()?;
    Ok((aa / bb).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let aa :f64 = String::from(a).parse::<f64>()?;
    let bb : f64 = String::from(b).parse::<f64>()?;
    Ok((aa % bb).to_string())
}

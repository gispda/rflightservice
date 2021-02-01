extern crate serde; 
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::fmt::{self, Formatter, Display};

#[derive(Serialize, Deserialize, Debug)]
pub struct Flights{
    pub id:String,
    pub flightno:String,
    pub corpno:String,
}

impl Display for Flights {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}",self.id, self.flightno, self.corpno)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let n = Flights{
            id:String::from("1"),
            flightno:String::from("8888"),
            corpno:String::from("CA"),
        };
        println!("{}",n);
    }

}
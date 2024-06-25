// use crate::agent::*;

use std::{fmt::Display, iter};

pub enum Predicate_{
    Atom{name : String, is_ : bool, description : String},
    Rule,
} 


// trait 


#[derive( Debug , PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Predicate{
    name : String , 
    is_ : bool , 
    descrption : String
}



// #[allow(Debug, Display, Clone , )]


impl  Predicate {
    pub fn new ( name : String, 
                 staus : bool ,
                 description : String) -> Self{
        Self { name: name, 
            is_ : staus , 
            descrption: description }
    }
    pub fn set_name(&mut self , name : String) {
        self.name = name;
        }
    pub fn is_active(self) -> bool{
        self.is_
    }

}


impl Default for Predicate {
    fn default() -> Self {
        Self {
            name : "".to_string() ,
            is_ : true ,
            descrption: "No descrption!".to_string(),
        }
    }
}
impl Display for Predicate{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std
    ::fmt::Result {
        write!(f, "Predicate {{ name: {}, is_: {}, descrption: {} }}",
        self.name, self.is_, self.descrption)
        }
}


#[derive(Debug, Default)]
pub struct VecPredicate{
    data : Vec<Predicate>
}

impl VecPredicate {
    pub fn new(data : Vec<Predicate>) -> Self {
        Self{data : data}
        }

    pub fn push(&mut self, predicate: Predicate) {
        self.data.push(predicate);
        }
    pub fn iter(&self) -> impl Iterator<Item = &Predicate> {
        self.data.iter()
        }
    pub fn into_iter(self) -> impl Iterator<Item = Predicate> {
        self.data.into_iter()
        }
}

impl Display for VecPredicate  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("{");
        for (i, predicate) in self.iter().enumerate(){
            s.push_str(&format!("{:?} : {:?}\n", i, (predicate.name.clone() , predicate.is_.clone())));
            }
            s.push_str("}");

            write!(f, "{}", s)
    }
}


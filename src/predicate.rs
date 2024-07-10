
use derive_more::{Display, Constructor};

pub enum Predicate_{
    Atom{name : String, is_ : bool, description : String},
    Rule,
} 


// trait 


#[derive( Debug ,Display, PartialEq, Eq, PartialOrd, Ord, Clone, Constructor,)]
#[display(fmt="Predicate{{ name: {name} , is {is_} ,\n {descrption}")]
pub struct Predicate{
    name : String , 
    is_ : bool , 
    descrption : String
}



// #[allow(Debug, Display, Clone , )]


impl  Predicate {
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
            name : "Not Assign".to_string() ,
            is_ : false ,
            descrption: "No descrption!".to_string(),
        }
    }
}


#[derive(Debug, Default, Constructor)]
pub struct VecPredicate{
    data : Vec<Predicate>
}

impl VecPredicate {
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



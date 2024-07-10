///  ### Some Shit As Documet 
/// 
/// todo! write document for trait
pub trait ActionTemplate {
    fn execute(command : String)-> Result<Self, &'static str> where Self: Sized;
}

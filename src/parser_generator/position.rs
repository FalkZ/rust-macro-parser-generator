use super::traits::RawToken;


#[derive(Default, Clone, Debug)]
pub struct Position{
   pub column: u32,
   pub line: u32,
}
pub trait GetPosition {
   fn position(&self)-> Position;
}

impl <T: GetPosition> GetPosition for Vec<T>{
   fn position(&self)-> Position {
         // TODO sensible default
         self.get(0).map(|v| v.position()).unwrap_or_default()
  }
}

impl <T: RawToken> GetPosition for T {
   fn position(&self)-> Position {
       self.raw_token().position()
   }
}

#[macro_use]
extern crate nom;
extern crate car_compress;
extern crate tar;
extern crate walkdir;
pub use self::tar::{
  Builder,
  Archive
};
pub use self::car_compress::{
  Format,
  Quality,
  Comp,
  Decomp
};
mod cli;
mod fsinteract;


fn main() {

  //a lot of CLI logic
  let arg = cli::fetch();
  match fsinteract::top_level(arg) {
    Ok(()) => { 
      //sunglasses emoji
    },
    Err(e) => {
      println!("IO error occured while compressing.\n {:?}", e);
      ::std::process::exit(1);
    }
  };
}

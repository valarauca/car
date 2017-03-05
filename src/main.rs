
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
  Decomp,
};
use self::car_compress::cli::Operation;
use std::io::Write;

fn main() {

  let arg = Operation::from_cli();
  if arg.is_read_action() {
    let reader = match arg.build_reader() {
      Ok(x) => x,
      Err(e) => {
        println!("Could not construct reader");
        println!("{:?}",e);
        ::std::process::exit(1);
      }
    };
    let a = Archive::new(reader);
    match arg.do_read(a) {
      Ok(_) => ::std::process::exit(0),
      Err(e) => {
        println!("Encountered unrecoverable error");
        println!("{:?}",e);
        ::std::process::exit(1);
      }
    };
  } else {
    
    match arg.do_compress() {
      Ok(mut x) => match x.flush() {
        Ok(_) => ::std::process::exit(0),
        Err(e) => {
          println!("Compression succeeded, but flushing the file didn't");
          println!("{:?}",e);
          ::std::process::exit(1);
        }
      },
      Err(e) => {
        println!("Encountered unrecoverable error");
        println!("{:?}",e);
        ::std::process::exit(1);
      }
    };
  }
}

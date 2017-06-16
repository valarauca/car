
#![allow(unused_imports)]
use super::{
  Format,
  Quality,
  App,
  SubCommand,
  ArgMatches,
  Arg,
  io,
  Write,
  Comp,
  PathBuf,
  Operation,
  Builder,
  File
};

use std::io::BufWriter;

extern crate walkdir;
use self::walkdir::WalkDir;

mod tar;
mod snap;
mod lz4;
mod zstd;
mod brotli;
mod gzip;
mod bzip2;
mod xz;

pub fn valid_item(x: String) -> Result<(),String> {
  let p = PathBuf::from(&x);
  match (p.exists(),p.is_file()|p.is_dir()) {
    (true,true) => Ok(()),
    (false,_) => Err(format!("Cannot process {} it does not exist",&x)),
    (true,false) => Err(format!("Cannot process {} it is something special",&x))
  }
}

pub fn item_exists(x: String) -> Result<(),String> {
  let p = PathBuf::from(&x);
  if p.exists() {
    Err(format!("{} exists\nCowardly refusing to delete it",&x))
  } else {
    Ok(())
  }
}

pub fn get_comp_level(x: &ArgMatches) -> Quality {
  if x.is_present("fast") {
    return Quality::FastLow;
  }
  if x.is_present("slow") {
    return Quality::SlowHigh;
  }
  Quality::Default
}

/// Build command
pub fn build<'a>() -> App<'static,'a> {
  SubCommand::with_name("create")
    .about("Create a tar archive")
    .subcommand(tar::build())
    .subcommand(snap::build())
    .subcommand(lz4::build())
    .subcommand(zstd::build())
    .subcommand(brotli::build())
    .subcommand(gzip::build())
    .subcommand(bzip2::build())
    .subcommand(xz::build())
}

/// Get a sub command
pub fn get(x: &ArgMatches) -> Operation {
  match x.subcommand_matches("tar") {
    Option::Some(x) => return tar::get(x),
    Option::None => { },
  };
  match x.subcommand_matches("snappy") {
    Option::Some(x) => return snap::get(x),
    Option::None => { }
  };
  match x.subcommand_matches("lz4") {
    Option::Some(x) => return lz4::get(x),
    Option::None => { },
  };
  match x.subcommand_matches("zstd") {
    Option::Some(x) => return zstd::get(x),
    Option::None => { },
  };
  match x.subcommand_matches("brotli") {
    Option::Some(x) => return brotli::get(x),
    Option::None => { },
  };
  match x.subcommand_matches("gzip") {
    Option::Some(x) => return gzip::get(x),
    Option::None => { },
  };
  match x.subcommand_matches("bzip2") {
    Option::Some(x) => return bzip2::get(x),
    Option::None => { },
  };
  match x.subcommand_matches("xz") {
    Option::Some(x) => return xz::get(x),
    Option::None => { },
  };
  println!("I didn't understand that");
  println!("Try running `--help`");
  ::std::process::exit(1);
}

fn building<W: Write>(c: Comp<W>, items: &[PathBuf]) -> io::Result<Comp<W>> {
  let mut builder = Builder::new(c);
  for path in items.iter() {
    if path.is_dir() {
      for wd in WalkDir::new(path)
                  .into_iter()
                  .filter_map(|x|x.ok())
                  .filter(|x|x.file_type().is_file()) {
        builder.append_path(wd.path())?;
      }
    }
    if path.is_file() {
      builder.append_path(path)?;
    }
  }
  builder.into_inner()
}
  

/// execute compressiong
pub fn exec<W: Write>(x: Comp<W>, items: &[PathBuf]) -> Result<BufWriter<W>,String> { 
  let x = match building(x,items) {
    Ok(x) => x,
    Err(e) => {
      println!("FATAL IO ERROR OCCURED");
      println!("{:?}",e);
      ::std::process::exit(1);
    }
  };
  x.finish()
}




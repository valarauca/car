
use super::{
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

extern crate walkdir;
use self::walkdir::WalkDir;

mod tar;

fn valid_item(x: String) -> Result<(),String> {
  let p = PathBuf::from(&x);
  match (p.exists(),p.is_file()|p.is_dir()) {
    (true,true) => Ok(()),
    (false,_) => Err(format!("Cannot process {} it does not exist",&x)),
    (true,false) => Err(format!("Cannot process {} it is something special",&x))
  }
}

fn item_exists(x: String) -> Result<(),String> {
  let p = PathBuf::from(&x);
  if p.exists() {
    Err(format!("{} exists\nCowardly refusing to delete it",&x))
  } else {
    Ok(())
  }
}

/// Build command
pub fn build<'a>() -> App<'static,'a> {
  SubCommand::with_name("create")
    .about("Create a tar archive")
    .arg(Arg::with_name("file")
      .short("f")
      .long("file")
      .takes_value(true)
      .multiple(true)
      .value_name("FILE/DIR")
      .required(true)
      .validator(valid_item)
      .next_line_help(true)
      .global(true)
      .help("what to tar"))
    .arg(Arg::with_name("output")
      .short("o")
      .long("out")
      .takes_value(true)
      .multiple(false)
      .value_name("OUTFILE")
      .required(true)
      .validator(item_exists)
      .global(true)
      .help("tarball output"))
     .subcommand(tar::build())
}

/// Get a sub command
pub fn get(x: &ArgMatches) -> Operation {
  match x.subcommand_matches("tar") {
    Option::Some(x) => return tar::get(x),
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
pub fn exec<W: Write>(x: Comp<W>, items: &[PathBuf]) -> Result<W,String> { 
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




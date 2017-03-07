
use super::{
  io,
  Read,
  Operation,
  App,
  Arg,
  ArgMatches,
  SubCommand,
  PathBuf,
  Regex,
  RegexFault,
  Entry
};

fn valid_path(x: String) -> Result<(),String> {
  let p = PathBuf::from(&x);
  match (p.exists(),p.is_file()) {
    (true,true) => Ok(()),
    (false,_) => Err(format!("Cannot process {} it does not exist",&x)),
    (true,false) => Err(format!("Cannot process {} it is a directory or link",&x))
  }
}
fn valid_regex(x: String) -> Result<(),String> {
  match Regex::new(&x) {
    Ok(_) => Ok(()),
    Err(RegexFault::CompiledTooBig(val)) => Err(format!("Input regex is too large. Set size limit {:?}", val)),
    Err(RegexFault::Syntax(s)) => Err(format!("Regex Syntax Error: {}", s)),
    Err(_) => Err(format!("Regex Syntax Error. Source undocumented :("))
  }
}
fn valid_dir(x: String) -> Result<(),String> {
  let p = PathBuf::from(&x);
  match (p.exists(),p.is_dir()) {
    (true,true) => Ok(()),
    (false,_) => Err(format!("Cannot extract to {} it does not exist",&x)),
    (true,false) => Err(format!("Cannot extract to {} it is file or link",&x))
  }
}

/// Build the `extract` subcommand
pub fn build<'a>() -> App<'static,'a> {
  SubCommand::with_name("extract")
    .about("Extract contents of a tar")
    .arg(Arg::with_name("file")
      .short("f")
      .long("file")
      .takes_value(true)
      .multiple(false)
      .value_name("INFILE")
      .required(true)
      .validator(valid_path)
      .next_line_help(true)
      .help("file to read"))
    .arg(Arg::with_name("regex")
      .short("r")
      .long("regex")
      .takes_value(true)
      .multiple(false)
      .value_name("REGEX")
      .validator(valid_regex)
      .next_line_help(true)
      .help("Only files which match REGEX will be extracted"))
    .arg(Arg::with_name("out")
      .short("o")
      .long("out")
      .takes_value(true)
      .multiple(false)
      .value_name("OUTPATH")
      .validator(valid_dir)
      .next_line_help(true)
      .help("Where to extract too"))
    .arg(Arg::with_name("xattrs")
      .long("xattrs")
      .takes_value(false)
      .next_line_help(true)
      .help("Indicate to preserve xattrs (Unix only)"))
    .arg(Arg::with_name("perms")
      .long("perms")
      .takes_value(false)
      .next_line_help(true)
      .help("Indicate to perserve perms (Unix only)"))
}

/// Extraction logic
pub fn exec<R: Read>(
        entry: &mut Entry<R>,
        regex: &Option<Regex>,
        to: &Option<PathBuf>,
        xattrs: bool,
        perms: bool)
-> io::Result<()>
{
  let flag = match regex {
    &Option::None => true,
    &Option::Some(ref regex) => {
      let path = entry.path()?;
      match path.file_name() {
        Option::None => false,
        Option::Some(f_name) => match f_name.to_str() {
          Option::None => false,
          Option::Some(f_name_str) => regex.is_match(f_name_str)
        }
      }
    }
  };
  if flag {
    #[cfg(unix)]
    {
      entry.set_unpack_xattrs(xattrs);
      entry.set_preserve_permissions(perms);
    }
    match to {
      &Option::None => entry.unpack_in(".")?,
      &Option::Some(ref p) => entry.unpack_in(p)?
    };
  }
  Ok(()) 
}


#[cfg(unix)]
pub fn get(x: &ArgMatches) -> Operation {
  Operation::Extract(
    PathBuf::from(x.value_of("file").unwrap()),
    match x.value_of("regex") {
      Option::None => None,
      Option::Some(r) => Regex::new(&r).ok()
    },
    match x.value_of("out") {
      Option::None => None,
      Option::Some(o) => Some(PathBuf::from(o))
    },
    x.is_present("xattrs"),
    x.is_present("perms")
  )
}

#[cfg(windows)]
pub fn get(x: &ArgMatches) -> Operation {
  Operation::Extract(
    PathBuf::from(x.value_of("file").unwrap()),
    match x.value_of("regex") {
      Option::None => None,
      Option::Some(r) => Regex::new(&r).ok()
    },
    match x.value_of("out") {
      Option::None => None,
      Option::Some(o) => Some(PathBuf::from(o))
    },
    false,
    false
  )
}



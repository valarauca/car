

use super::{
  print_size,
  io,
  Header,
  Operation,
  App,
  Arg,
  ArgMatches,
  SubCommand,
  PathBuf,
  Regex,
  RegexFault
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

pub fn build<'a>() -> App<'static,'a> {
  SubCommand::with_name("list")
    .about("lists contents of a regex")
    .arg(Arg::with_name("group")
      .long("groupname")
      .takes_value(false)
      .next_line_help(true)
      .help("display group name"))
    .arg(Arg::with_name("user")
      .long("username")
      .takes_value(false)
      .next_line_help(true)
      .help("display username"))
    .arg(Arg::with_name("uid")
      .long("uid")
      .takes_value(false)
      .next_line_help(true)
      .help("display uid"))
    .arg(Arg::with_name("gid")
      .long("gid")
      .takes_value(false)
      .next_line_help(true)
      .help("display gid"))
    .arg(Arg::with_name("size")
      .long("size")
      .takes_value(false)
      .next_line_help(true)
      .help("display file size"))
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
      .help("regex to filter list by"))
}

/// print data
pub fn exec(
        header: &Header,
        regex: &Option<Regex>,
        group: bool,
        user: bool, 
        gid: bool,
        uid: bool,
        size: bool)
-> io::Result<()> 
{
  let flag = match regex {
    &Option::None => true,
    &Option::Some(ref regex) => {
      let path = header.path()?;
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
    if group {
      let g = match header.groupname() {
        Ok(Option::Some(x)) => x,
        Ok(Option::None) => "No groupname",
        Err(_) => "UTF8 ERROR"
      };
      println!("\tGroup Name: {}",g);
    }
    if user {
      let u = match header.username() {
        Ok(Option::Some(x)) => x,
        Ok(Option::None) => "No username",
        Err(_) => "UTF8 ERROR"
      };
      println!("\tUser Name: {}",u);
    }
    if gid {
      println!("\tUser Group ID (gid): 0x{:X}", header.gid()?);
    }
    if uid {
      println!("\tUser ID (uid): 0x{:X}", header.uid()?);
    }
    if size {
      println!("\tSize: {}", print_size(header.size()?));
    }
  }
  Ok(())
}

pub fn get(x: &ArgMatches) -> Operation {
  Operation::List(
    PathBuf::from(x.value_of("file").unwrap()),
    match x.value_of("regex") {
      Option::None => None,
      Option::Some(r) => Regex::new(&r).ok()
    },
    x.is_present("group"),
    x.is_present("user"),
    x.is_present("gid"),
    x.is_present("uid"),
    x.is_present("mtime"),
    x.is_present("size")
  )
}



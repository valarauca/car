
extern crate tar;
pub use self::tar::{
  Archive,
  Entries,
  Entry,
  Header,
  Builder
};

extern crate clap;
pub use self::clap::{
  App,
  Arg,
  ArgMatches,
  SubCommand
};

extern crate regex;
pub use self::regex::{
  Regex,
  Error as RegexFault
};
pub use std::path::{
  Path,
  PathBuf
};
pub use std::fs::{
  OpenOptions,
  File,
};
pub use std::io::{
  self,
  Write,
  Read,
  BufWriter
};

extern crate car_compress; 
use car_compress::{
  Quality,
  Format,
  Comp,
  Decomp
};


mod list;
mod extract;
mod create;

/// Formats a value in human readable
/// this is a dirty hack and not efficient at all
pub fn print_size(a: u64) -> String {
  match a {
    0...1024 => format!("{:?}B",a),
    1025...1048576 => format!("{:.2}KiB", (a as f64)/1024f64),
    1048577...1073741824 => format!("{:.2}MiB", (a as f64)/1048576f64),
    1073741825...1099511627776 => format!("{:.2}GiB", (a as f64)/1073741824f64),
    _ => format!("{:.2}TiB",(a as f64)/1099511627776f64)
  }
}

fn fetch<'a>() -> ArgMatches<'a> {
  App::new("car")
    .set_term_width(80)
    .author("Cody Laeder, <codylaeder@gmail.com>")
    .version("1.0")
    .about("Cody's Archive Reader, TAR compatible CLI tool")
    .subcommand(list::build())
    .subcommand(extract::build())
    .subcommand(create::build())
    .get_matches()
}
fn build_ops(x: &ArgMatches) -> Operation {
  match x.subcommand_matches("list") {
    Option::Some(x) => return list::get(x),
    Option::None => { },
  };
  match x.subcommand_matches("extract") {
    Option::Some(x) => return extract::get(x),
    Option::None => { },
  };
  match x.subcommand_matches("create") {
    Option::Some(x) => return create::get(x),
    Option::None => { },
  };
  println!("I didn't understand that");
  println!("Try running `--help`");
  ::std::process::exit(1);
}

/// Describes what the program is doing
pub enum Operation {
  List(PathBuf,Option<Regex>,bool,bool,bool,bool,bool,bool),
  Extract(PathBuf,Option<Regex>,Option<PathBuf>, bool, bool),
  Create(Comp<File>,Vec<PathBuf>)
}
impl Operation {
  
  /// Construction from CLI
  pub fn from_cli() -> Self {
    let x = fetch();
    build_ops(&x)
  }

  /// Are we reading or writing?
  pub fn is_read_action(&self) -> bool {
    match self {
      &Operation::List(_, _, _, _, _, _, _, _) |
      &Operation::Extract(_, _, _, _, _) => true,
      _ => false
    }
  }

  /// Are we writing?
  #[inline(always)]
  pub fn is_write_action(&self) -> bool {
    ! self.is_read_action()
  }


  /// Create the reader decompressor
  pub fn build_reader(&self) -> io::Result<Decomp<File>> {
    match self {
      &Operation::List(ref p, _, _, _, _, _, _, _) |
      &Operation::Extract(ref p, _, _, _, _) => {
        let f = OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(p)?;
        Decomp::from_unknown(f)
      },
      _ => panic!("Cody you called build_reader on an compress job")
    }
  }

  /// Does compression
  pub fn do_compress(self) -> Result<BufWriter<File>,String> {
    match self {
      Operation::Create(comp,items) => create::exec(comp,&items),
      _ => panic!("Cody you called compress on an extract/list op")
    }
  }

  /// Execute a read operation
  pub fn do_read<R: Read>(&self, x: Archive<R>) -> io::Result<()> {
    let mut x = x;
    let entries = x.entries()?;
    for e in entries {
      let mut e = e?;
      match self {
        &Operation::List(_, ref r, group, user, gid, uid, mtime, size) => {
          let header = e.header();
          list::exec( header, r, group, user, gid, uid, size)?;
        },
        &Operation::Extract(_, ref r, ref outdir, xattrs, perms) => {
          extract::exec( &mut e, r, outdir, xattrs, perms)?;
        },
        _ => panic!("Cody you called do_read on an compress job")
      };
    }
    Ok(())
  }
}

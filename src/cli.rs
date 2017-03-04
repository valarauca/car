
use super::{
  Quality,
  Format
};
use std::path::{
  PathBuf,
  Path
};
use std::env::args;

/*
 * What mode the program is in
 */
pub enum Mode {
  Create(Format),
  Extract,
  Help,
  Version
}

named!(what_do<Mode>, do_parse!(
  z: alt_complete!(
    do_parse!(alt_complete!(
        tag!(b"h") |
        tag!(b"-h") |
        tag!(b"--h") |
        tag!(b"-help")|
        tag!(b"--help") |
        tag!(b"--HELP") |
        tag!(b"HELP") |
        tag!(b"help") |
        tag!(b"H") |
        tag!(b"-H")
      ) >>
      (Mode::Help)
    )|
    do_parse!(alt_complete!(
        tag!(b"-v") |
        tag!(b"--version") |
        tag!(b"-V") |
        tag!(b"--v") |
        tag!(b"--V") |
        tag!(b"-version")
      ) >>
      (Mode::Version)
    )|
    do_parse!(tag!(b"c") >>
      y: alt_complete!(
        do_parse!(alt!(tag!(b"zf")|tag!(b"fz")) >>
          (Mode::Create(Format::Gzip(Quality::Default)))
        )|
        do_parse!(alt!(tag!(b"jf")|tag!(b"fj")) >>
          (Mode::Create(Format::Bzip2(Quality::Default)))
        )|
        do_parse!(alt!(tag!("bJf")|tag!(b"fJ")) >>
          (Mode::Create(Format::Xz(Quality::Default)))
        )|
        do_parse!(alt!(tag!(b"f4")|tag!(b"4f")) >>
          (Mode::Create(Format::Lz4(Quality::Default)))
        )|
        do_parse!(alt!(tag!(b"Df")|tag!(b"fD")) >>
          (Mode::Create(Format::Zstd(Quality::Default)))
        )|
        do_parse!(alt!(tag!(b"Sf")|tag!(b"fS")) >>
          (Mode::Create(Format::Snappy(Quality::Default)))
        )|
        do_parse!(alt!(tag!(b"Bf")|tag!(b"fB")) >>
          (Mode::Create(Format::Brotli(Quality::Default)))
        )|
        do_parse!(tag!(b"f") >> (Mode::Create(Format::Tar(Quality::Default))))
      ) >>
      (y)
    )|
    do_parse!(tag!(b"xf") >> (Mode::Extract))
  ) >>
  (z)
));

const HELP: &'static str = "

CAR

Cody's Archive Reader

Version: 0.0.1

USAGE:
    car.exe [FLAGS] OPTIONS ...

FLAGS:

    THIS IS AN EXTREMELY EARLY RELEASE SO NOT ALL TAR FLAGS
    ARE SUPPORTED


    c: create
    x: extract

    Compression Modes:
      z: gzip
      j: bzip2
      J: XZ (LZMA)
      4: Lz4
      D: Zstd
      S: Snappy
      B: Brotli

    Traditional Tar usage dictates flags maybe combined into cords
    Supported cords:
      
      -creation-

      czf: create gzip file   (.tar.gz)
      cjf: create bzip2 file  (.tar.bzip2)
      cJf: create xz file     (.tar.xz)
      c4f: create lz4 file    (.tar.lz4)
      cDf: create Zstd file   (.tar.zst)
      cSf: create Snappy file (.tar.sz)
      cBf: create Brotli file (.tar.bf)
      cf : create file        (.tar)        (no compression)

      -extraction-
      
      xf : extract file
        the format of what is being extracted is determined at runtime

      -diff-
      COMING SOON

      -list-
      COMING SOONER
      
      -append-
      COMING MAYBE

      -delete-
      COMING ABOUT THE SAME TIME AS -list-
";
const VERSION: &'static str = "v0.0.1";


/*
 * Argument Pre-Processing
 *
 */
fn get_mode(x: &str) -> Mode {
  use super::nom::IResult;
  match what_do(x.as_bytes()) {
    IResult::Done(x,y) => match y {
      Mode::Help => {
        println!("{}",HELP);
        ::std::process::exit(0);
      },
      Mode::Version => {
        println!("{}", VERSION);
        ::std::process::exit(0);
      },
      z=> {
        if x.len() != 0 {
          println!("I didn't understand that command. Please use `-help`");
          ::std::process::exit(1);
        } else {
          z
        }
      }
    },
    IResult::Error(_) |
    IResult::Incomplete(_) => {
      println!("I didn't understand that.");
      println!("Try `-help` to display help");
      ::std::process::exit(1);
    }
  }
}
fn get_args() -> Vec<String> {
  args().skip(1).collect()
}


/// What state a CLI argument is in
pub enum ItemState {
  ExistsFile(PathBuf),
  ExistsDir(PathBuf),
  NotExist(PathBuf)
}
fn is_dir<P: AsRef<Path>>(p: P) -> bool {
  p.as_ref().is_dir()
}
fn is_file<P: AsRef<Path>>(p: P) -> bool {
  p.as_ref().is_file()
}
fn exists<P: AsRef<Path>>(p: P) -> bool {
  p.as_ref().exists()
}
impl ItemState {
  
  /// Figure out what _kind_ of item this is
  pub fn conversion(x: String) -> ItemState {
    if is_dir(&x) {
      return ItemState::ExistsDir(PathBuf::from(x));
    }
    if is_file(&x) {
      return ItemState::ExistsFile(PathBuf::from(x));
    }
    if exists(&x) {
      println!("I'm a really new program I can't handle symlinks");
      ::std::process::exit(1);
    }
    ItemState::NotExist(PathBuf::from(x))
  }

  /// Test if an item exists
  pub fn does_exist(&self) -> bool {
    match self {
      &ItemState::ExistsFile(_) |
      &ItemState::ExistsDir(_) => true,
      _ => false
    }
  }
}

/// Build the CLI arg
pub fn fetch() -> (Mode,ItemState,Vec<ItemState>) {

  let mut v = get_args();
  if v.len() == 0 {
    println!("I didn't understand that. Please use `-help` for help");
    ::std::process::exit(1);
  }
  let mm = v.remove(0);
  let m = get_mode(&mm);
  
  if v.len() == 0 {
    println!("can't provide flags without paths/files to work on");
    ::std::process::exit(1);
  }
  let ii = v.remove(0);
  let i = ItemState::conversion(ii);
  let x: Vec<ItemState> = v.into_iter()
    .map(ItemState::conversion)
    .filter(|x| x.does_exist())
    .collect();
  (m,i,x)
}

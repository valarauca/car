
use super::header::Format;
use std::path::PathBuf;
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
          (Mode::Create(Format::Gzip))
        )|
        do_parse!(alt!(tag!(b"jf")|tag!(b"fj")) >>
          (Mode::Create(Format::Bzip2))
        )|
        do_parse!(alt!(tag!("bJf")|tag!(b"fJ")) >>
          (Mode::Create(Format::Xz))
        )|
        do_parse!(alt!(tag!(b"f4")|tag!(b"4f")) >>
          (Mode::Create(Format::Lz4))
        )|
        do_parse!(alt!(tag!(b"Df")|tag!(b"fD")) >>
          (Mode::Create(Format::Zstd))
        )|
        do_parse!(alt!(tag!(b"Sf")|tag!(b"fS")) >>
          (Mode::Create(Format::Snappy))
        )|
        do_parse!(alt!(tag!(b"Bf")|tag!(b"fB")) >>
          (Mode::Create(Format::Brotli))
        )|
        do_parse!(tag!(b"f") >> (Mode::Create(Format::Tar)))
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

    THIS IS AN EXTREMELY EARLY RELEASE SO NO ALL TAR FLAGS
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
      WHAT IS BEING EXTRACT IS DETECTED AT RUNTIME
      
      xf : extract file

      -diff-
      COMING SOON

      -list-
      COMING SOONER
      
      -append-
      COMING NEVER

      -delete-
      COMING ABOUT THE SAME TIME AS -list-

NOTES:

  Eventually additional options will be supported
    -All the LZMA tuning stuff
    -Brotli Text/Generic modes
    -Zstd dictionary building 
      (is there a framing format for dictionary+data(?))
  
DEFAULTS:

  Generally I looked for the inflection point where high compression
  level resulted in a larger slow down than compression ratio savings.
  The source for this was using Przemyslaw Skibinski (github.com/inikep)'s
  lzbench results.

  Brotli is set to quality level 5 (generic mode)
    (generally should pull off better compression then LZMA at ~2x speed)
  Lz4 is set to quality level 6
  XZ is set to quality level 1
  ZSTD is set to quality level 8
    (this should provide compression on par with gzip -9 with ~4x speed)

";
const VERSION: &'static str = "v0.0.1";


/*
 * Argument Pre-Processing
 *
 */
fn get_mode(x: &str) -> Mode {
  use super::nom::IResult;
  match what_do(x.as_bytes()) {
    IResult::Done(_,y) => match y {
      Mode::Help => {
        println!("{}",HELP);
        ::std::process::exit(0);
      },
      Mode::Version => {
        println!("{}", VERSION);
        ::std::process::exit(0);
      },
      x => x,
    },
    IResult::Error(_) |
    IResult::Incomplete(_) => {
      println!("I didn't understand that.");
      println!("Try -h to display help");
      ::std::process::exit(1);
    }
  }
}
fn get_args() -> Vec<String> {
  args().collect()
}


/*
 * Actual Things to use
 *
 */
pub enum Operation {
  Compress(Format,PathBuf,PathBuf),
  Extract(PathBuf, PathBuf),
}


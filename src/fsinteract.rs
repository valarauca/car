
use std::path::{
  PathBuf,
  Path
};
use std::io::{
  self,
  Write
};
use std::fs::File;
use super::{
  Comp,
  Format,
  Builder,
};
use super::cli::{
  ItemState,
  Mode
};
use super::walkdir::WalkDir;

/// Top level argument entry point
pub fn top_level(x: (Mode,ItemState,Vec<ItemState>)) -> io::Result<()> {
  let (m, a, t) = x;
  match (m,a) {
    (Mode::Create(f),ItemState::NotExist(ref p)) => build(f,p,t),
    (Mode::Extract,ItemState::ExistsFile(p)) => unreachable!(),
    (Mode::Extract,ItemState::NotExist(_)) => {
      println!("I don't understand.");
      println!("I cannot extract something that doesn't exists");
      ::std::process::exit(1)
    },
    (Mode::Extract,ItemState::ExistsDir(_)) => {
      println!("First argument is _what_ is being extracted");
      println!("Not where");
      println!("car xf <FILE> <TO>");
      ::std::process::exit(1)
    },
    (Mode::Create(_),ItemState::ExistsFile(_)) => {
      println!("Cowardly refusing to overwriting an existing file");
      ::std::process::exit(1)
    },
    (Mode::Create(_),ItemState::ExistsDir(_)) => {
      println!("I get what you are saying, eventually this'll work, not today");
      ::std::process::exit(1)
    },
    _ => unreachable!()
  }
}

/// Actually builds an archive
fn build(fmat: Format, p: &Path, e: Vec<ItemState>) -> io::Result<()> {
  let f = File::create(&p)?;
  let compressor = Comp::from_format(fmat,f)?;
  let mut b = Builder::new(compressor);
  for item in e.into_iter() {
    match item {
      ItemState::ExistsDir(ref p) => {
        for i in WalkDir::new(p)
                  .into_iter()
                  .filter_map(|x|x.ok())
                  .filter(|x|x.file_type().is_file()) {
          b.append_path(i.path())?;
        }
      },
      ItemState::ExistsFile(ref p) => {
        b.append_path(p)?;
      },
      _ => break
    };
  }
  let compressor = b.into_inner()?;
  match compressor.finish() {
    Ok(mut x) => {
      x.flush()?;
      let _ = x;
      Ok(())
    },
    Err(e) => {
      println!("\n\nCompressor Error {:?}",e);
      ::std::process::exit(1)
    }
  }
}


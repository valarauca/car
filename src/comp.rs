
use super::header::Format;
use std::io::{
  self,
  Read,
  Write,
  BufReader,
  BufWriter
};
use std::fs::OpenOptions;
extern crate bzip2;
use bzip2::write::BzEncoder;
use bzip2::read::BzDecoder;
use bzip2::Compression as BzComp;
extern crate brotli2;
extern crate libflate;
use libflate::gzip::{
  Encoder as GEn,
  Decoder as GDec
};
extern crate lzw;
extern crate snap;
use snap::{
  Reader as SnapReader,
  Writer as SnapWriter
};
extern crate zstd;


pub enum Decomp<R: Read> {
  Bzip2(BzDecoder<R>),
  Snap(SnapReader<BufReader<R>>),
  Gzip(GDec<BufReader<R>>)
}
impl<R: Read> Read for Decomp<R> {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    match self {
      &mut Decomp::Bzip2(ref mut x) => x.read(buf),
      &mut Decomp::Snap(ref mut x) => x.read(buf),
      &mut Decomp::Gzip(ref mut x) => x.read(buf),
    }
  }
}

pub enum Comp<W: Write> {
  Bzip2(BzEncoder<W>),
  Snap(SnapWriter<W>),
  Gzip(GEn<W>),
}
impl<W: Write> Write for Comp<W> {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    match self {
      &mut Comp::Bzip2(ref mut x) => x.write(buf),
      &mut Comp::Snap(ref mut x) => x.write(buf),
      &mut Comp::Gzip(ref mut x) => x.write(buf),
    }
  }
  fn flush(&mut self) -> io::Result<()> {
    match self {
      &mut Comp::Bzip2(ref mut x) => x.flush(),
      &mut Comp::Snap(ref mut x) => x.flush(),
      &mut Comp::Gzip(ref mut x) => x.flush(),
    }
  }
}
impl<W: Write> Comp<W> {

  /// Complete the compression
  pub fn finish(self) -> Result<W,String> {
    match self {
      Comp::Gzip(x) => match x.finish().unwrap() {
        (w,Option::None) => Ok(w),
        (mut w,Option::Some(e)) => {
          let _ = w.flush();
          Err(format!("{:?}", e))
        }
      },
      Comp::Snap(x) => match x.into_inner() {
        Ok(x) => Ok(x),
        Err(e) => Err(format!("{:?}",e))
      },
      Comp::Bzip2(x) => match x.finish() {
        Ok(x) => Ok(x),
        Err(e) => Err(format!("{:?}",e))
      }
    }
  }

  /// From the 
  pub fn from(f: Format, w: W) -> Comp<W> {
    match f {
      Format::Bzip2 => Comp::Bzip2(
        BzEncoder::new(w,
          BzComp::Default)),
      Format::Gzip => Comp::Gzip( match GEn::new(w) {
        Ok(x) => x,
        Err(e) => panic!("Could not contruct gzip. {:?}", e)
      }),
      Format::Snappy => Comp::Snap(SnapWriter::new(w)),
      _ => panic!("Not supported")
    }
  }
}


pub fn decomp<R: Read>(f: Format, r: R) -> Decomp<R> {
  match f {
    Format::Bzip2 => Decomp::Bzip2(
      BzDecoder::new(r)),
    Format::Snappy => Decomp::Snap(
      SnapReader::new(
        BufReader::with_capacity(16384,r))),
    _ => unreachable!()
  }
}


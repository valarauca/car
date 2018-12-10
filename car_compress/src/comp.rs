
use super::Format;

use std::io::{self, Read, Write, Seek, BufReader, BufWriter};

use super::libbzip::Encode as BzEn;
use super::libbzip::Decode as BzDec;

use super::libbrotli::Encode as BrEn;
use super::libbrotli::Decode as BrDec;

use super::libflate::Encode as GzEn;
use super::libflate::Decode as GzDec;

use super::libxz::Decode as XzDec;
use super::libxz::Encode as XzEn;

use super::libsnap::{Decode as SzDec, Encode as SzEn};

use super::libzstd::{Encode as DzEn, Decode as DzDec};

use super::liblz4::{Encode as LzEn, Decode as LzDec};

/// Abstraction around several _kinds_ of decompressors
pub enum Decomp<R: Read> {
    Bzip2(BzDec<BufReader<R>>),
    Snap(SzDec<BufReader<R>>),
    Gzip(GzDec<BufReader<R>>),
    Brotli(BrDec<BufReader<R>>),
    Zstd(DzDec<BufReader<R>>),
    Lz4(LzDec<BufReader<R>>),
    Xz(XzDec<BufReader<R>>),
    Tar(BufReader<R>),
}
impl<R: Read> Read for Decomp<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            &mut Decomp::Bzip2(ref mut x) => x.read(buf),
            &mut Decomp::Snap(ref mut x) => x.read(buf),
            &mut Decomp::Gzip(ref mut x) => x.read(buf),
            &mut Decomp::Brotli(ref mut x) => x.read(buf),
            &mut Decomp::Zstd(ref mut x) => x.read(buf),
            &mut Decomp::Lz4(ref mut x) => x.read(buf),
            &mut Decomp::Xz(ref mut x) => x.read(buf),
            &mut Decomp::Tar(ref mut x) => x.read(buf),
        }
    }
}
impl<R: Read + Seek> Decomp<R> {
    /// Read the file to determine _how_ to decompress it.
    ///
    /// If the file is no supported this will return `Err(InvalidInput)`
    ///
    ///#Decoder Notes
    ///
    /// Some decoders can throw an error during construction
    ///
    ///* Gzip
    ///* Zstd
    ///* Lz4
    pub fn from_unknown(r: R) -> io::Result<Decomp<R>> {
        let mut r = r;
        let f = Format::from_reader(&mut r)?;
        match f {
            Format::Bzip2(_) => Ok(Decomp::Bzip2(
                BzDec::new(BufReader::with_capacity(131072, r)),
            )),
            Format::Snappy(_) => Ok(Decomp::Snap(
                SzDec::new(BufReader::with_capacity(131072, r)),
            )),
            Format::Gzip(_) => Ok(Decomp::Gzip(
                GzDec::new(BufReader::with_capacity(131072, r))?,
            )),
            Format::Brotli(_) => Ok(Decomp::Brotli(
                BrDec::new(BufReader::with_capacity(131072, r)),
            )),
            Format::Zstd(_) => Ok(Decomp::Zstd(
                DzDec::new(BufReader::with_capacity(131072, r))?,
            )),
            Format::Lz4(_) => Ok(Decomp::Lz4(
                LzDec::new(BufReader::with_capacity(131072, r))?,
            )),
            Format::Zip7(_) | Format::Xz(_) => Ok(Decomp::Xz(
                XzDec::new(BufReader::with_capacity(131072, r)),
            )),
            Format::LZW(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported file type LZW",
            )),
            Format::LZH(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported file type LZH",
            )),
            Format::Tar(_) => Ok(Decomp::Tar(BufReader::with_capacity(131072, r))),
        }
    }
}
impl<R: Read> Decomp<R> {
    /// You already know what you are decompressing
    ///
    /// Quality argument is ignored
    pub fn from_known(f: Format, r: R) -> io::Result<Decomp<R>> {
        match f {
            Format::Bzip2(_) => Ok(Decomp::Bzip2(
                BzDec::new(BufReader::with_capacity(131072, r)),
            )),
            Format::Snappy(_) => Ok(Decomp::Snap(
                SzDec::new(BufReader::with_capacity(131072, r)),
            )),
            Format::Gzip(_) => Ok(Decomp::Gzip(
                GzDec::new(BufReader::with_capacity(131072, r))?,
            )),
            Format::Brotli(_) => Ok(Decomp::Brotli(
                BrDec::new(BufReader::with_capacity(131072, r)),
            )),
            Format::Zstd(_) => Ok(Decomp::Zstd(
                DzDec::new(BufReader::with_capacity(131072, r))?,
            )),
            Format::Lz4(_) => Ok(Decomp::Lz4(
                LzDec::new(BufReader::with_capacity(131072, r))?,
            )),
            Format::Zip7(_) | Format::Xz(_) => Ok(Decomp::Xz(
                XzDec::new(BufReader::with_capacity(131072, r)),
            )),
            Format::LZW(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported file type LZW",
            )),
            Format::LZH(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported file type LZH",
            )),
            Format::Tar(_) => Ok(Decomp::Tar(BufReader::with_capacity(131072, r))),
        }
    }
}


/// Compressor
///
/// This is a write based compressor
pub enum Comp<W: Write> {
    Bzip2(BzEn<BufWriter<W>>),
    Snap(SzEn<BufWriter<W>>),
    Gzip(GzEn<BufWriter<W>>),
    Brotli(BrEn<BufWriter<W>>),
    Zstd(DzEn<BufWriter<W>>),
    Lz4(LzEn<BufWriter<W>>),
    Xz(XzEn<BufWriter<W>>),
    Tar(BufWriter<W>),
}
impl<W: Write> Write for Comp<W> {
    /// Write data to the stream
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            &mut Comp::Xz(ref mut x) => x.write(buf),
            &mut Comp::Bzip2(ref mut x) => x.write(buf),
            &mut Comp::Snap(ref mut x) => x.write(buf),
            &mut Comp::Gzip(ref mut x) => x.write(buf),
            &mut Comp::Brotli(ref mut x) => x.write(buf),
            &mut Comp::Zstd(ref mut x) => x.write(buf),
            &mut Comp::Lz4(ref mut x) => x.write(buf),
            &mut Comp::Tar(ref mut x) => x.write(buf),
        }
    }

    /// This method will always return `Ok(())` if you wish to flush
    /// the underlying stream please call `self.finish()` to get the
    /// writer object back and complete the stream
    fn flush(&mut self) -> io::Result<()> {
        match self {
            _ => Ok(()),
        }
    }
}
impl<W: Write> Comp<W> {
    /// Complete the compression
    ///
    /// This signals for the decompressor to attempt to finish
    /// it's stream by writing any final data out
    pub fn finish(self) -> Result<BufWriter<W>, String> {
        match self {
            Comp::Gzip(x) => {
                match x.finish() {
                    Ok(x) => Ok(x),
                    Err(e) => Err(format!("{:?}", e)),
                }
            }
            Comp::Snap(x) => {
                match x.into_inner() {
                    Ok(x) => Ok(x),
                    Err(e) => Err(format!("{:?}", e)),
                }
            }
            Comp::Bzip2(x) => {
                match x.finish() {
                    Ok(x) => Ok(x),
                    Err(e) => Err(format!("{:?}", e)),
                }
            }
            Comp::Brotli(x) => {
                match x.finish() {
                    Ok(x) => Ok(x),
                    Err(e) => Err(format!("{:?}", e)),
                }
            }
            Comp::Zstd(x) => {
                match x.finish() {
                    Ok(x) => Ok(x),
                    Err(e) => Err(format!("{:?}", e)),
                }
            }
            Comp::Lz4(x) => {
                match x.finish() {
                    (x,Ok(())) => Ok(x),
                    (mut x, Err(e)) => {
                        //an attempt is made
                        let _ = x.flush();
                        Err(format!("{:?}", e))
                    }
                }
            }
            Comp::Xz(x) => {
                match x.finish() {
                    Ok(x) => Ok(x),
                    Err(e) => Err(format!("{:?}", e)),
                }
            }
            Comp::Tar(x) => Ok(x),
        }
    }

    /// Encodes from a format
    ///
    /// #Error:
    ///
    /// Creating some encoders _may_ result in an error Namely:
    ///
    /// *Tar: Tar isn't a compression format, it is an archive format
    ///you combine many files _into_ a tar ball.
    pub fn from_format(f: Format, w: W) -> io::Result<Comp<W>> {
        match f {
            Format::Bzip2(q) => Ok(Comp::Bzip2(
                BzEn::new(BufWriter::with_capacity(131072, w), q.into_bz()),
            )),
            Format::Gzip(q) => Ok(Comp::Gzip(
                GzEn::new(BufWriter::with_capacity(131072, w), q.into_gz()),
            )),
            Format::Snappy(_) => Ok(Comp::Snap(SzEn::new(BufWriter::with_capacity(131072, w)))),
            Format::Brotli(q) => Ok(Comp::Brotli(
                (q.into_brotli())(BufWriter::with_capacity(131072, w)),
            )),
            Format::Zstd(q) => Ok(Comp::Zstd(
                (q.into_zstd())(BufWriter::with_capacity(131072, w))?,
            )),
            Format::Lz4(q) => Ok(Comp::Lz4(
                (q.into_lz4()(BufWriter::with_capacity(131072, w))?),
            )),
            Format::Xz(q) => Ok(Comp::Xz(
                XzEn::new(BufWriter::with_capacity(131072, w), q.into_xz()),
            )),
            Format::Zip7(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported file type 7z",
            )),
            Format::LZW(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported file type LZW",
            )),
            Format::LZH(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported file type LZH",
            )),
            Format::Tar(_) => Ok(Comp::Tar(BufWriter::with_capacity(131072, w))),
        }
    }
}

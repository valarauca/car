

use super::libbzip::BzQuality;
use super::libflate::GzQuality;

use super::libbrotli::{Encode as BrEn, Builder as BrBuilder, Mode as BrMode};

use super::libzstd::Encode as DzEn;

use super::liblz4::{Encode as LzEn, Builder as LzBuilder, BSize as Lz4BSize, BMode as Lz4BMode,
                    Checksum as Lz4Checksum};


use std::path::Path;
use std::io::{self, Read, Seek, SeekFrom};
use std::fs::OpenOptions;
use std::default::Default;

/*
 * Magic Numbers and Extentions
 *
 * This file attempts to codify and enumerate _all_
 * the various compression algorithms and their signatures
 *
 *  Most of the list is from:https://en.wikipedia.org/wiki/List_of_file_signatures
 *
 * Some are just from reading RFC/Standards:
 * Snappy: https://github.com/google/snappy/blob/master/framing_format.txt
 * XZ: http://tukaani.org/xz/xz-file-format.txt
 * Brotli: https://github.com/madler/brotli/blob/501e6a9d03bcc15f0bc8015f4f36054c30f699ca/br-format-v3.txt
 * Zstd: https://github.com/facebook/zstd
 * Zstd (cont.): https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md
 */


/// Describes the ratio/speed trade off
///
/// More or less these are fixed constants I've eyeballed for
/// the _new_ methods, while for Bzip2/Gzip they're effectively
/// standard values.
///
/// _Any_ input for `Format::Snappy` is ignored as it only has
/// 1 mode of operation.
///
/// #Brotli
///
///* `Quality::FastLow`: 2
///* `Quality::Default`: 5
///* `Quality::SlowHigh`: 8
///
/// #Zstd
///
///* `Quality::FastLow`: 1
///* `Quality::Default`: 10
///* `Quality::SlowHigh`: 21
///
/// #Lz4
///
///User input is ignored, default value is always used.
///
/// #Xz
///
///* `Quality::FastLow`: 0
///* `Quality::Default`: 3
///* `Quality::SlowHigh`: 7
#[derive(Clone, Debug)]
pub enum Quality {
    Default,
    FastLow,
    SlowHigh,

    /// Level ranges from 1 to 21
    ZstdSpecial(i32),

    /// Level ranges from 0 to 11
    BrotliSpecial(BrMode, u32, u32, u32),

    /// level ranges from 0 to 16
    Lz4Special(Lz4BSize, Lz4BMode, Lz4Checksum, u32),
}
impl Quality {
    /// Creates a closure which contructs the LZ4 encoder with
    /// the compression information in the enum
    pub fn into_lz4<W: io::Write>(self) -> Box<Fn(W) -> io::Result<LzEn<W>>> {
        match self {
            Quality::FastLow => Box::new(move |w| {
                let mut b = LzBuilder::new();
                b.level(1);
                b.checksum(Lz4Checksum::NoChecksum);
                b.build(w)
            }),
            Quality::SlowHigh => Box::new(move |w| {
                let mut b = LzBuilder::new();
                b.level(16);
                b.block_size(Lz4BSize::Max4MB);
                b.block_mode(Lz4BMode::Linked);
                b.checksum(Lz4Checksum::ChecksumEnabled);
                b.build(w)
            }),
            Quality::Lz4Special(bsize, bmode, csu, lvl) => Box::new(move |w| {
                let mut b = LzBuilder::new();
                b.level(lvl.clone());
                b.block_size(bsize);
                b.block_mode(bmode);
                b.checksum(csu);
                b.build(w)
            }),
            _ => Box::new(move |w| LzBuilder::new().build(w)),
        }
    }

    /// Creates a closure which constructs the ZSTD encoder with
    /// the compression information in the enum
    pub fn into_zstd<W: io::Write>(self) -> Box<Fn(W) -> io::Result<DzEn<W>>> {
        match self {
            Quality::FastLow => Box::new(move |w| DzEn::new(w, 1)),
            Quality::SlowHigh => Box::new(move |w| DzEn::new(w, 21)),
            Quality::ZstdSpecial(qual) => Box::new(move |w| DzEn::new(w, qual.clone())),
            _ => Box::new(move |w| DzEn::new(w, 10)),
        }
    }

    /// Creates a closure which constructs the Brotli encoder with
    /// the compression information in the enum
    pub fn into_brotli<W: io::Write>(self) -> Box<Fn(W) -> BrEn<W>> {
        match self {
            Quality::BrotliSpecial(mode, qual, win, block) => Box::new(move |w| {
                let brotli_mode = mode.clone();
                let quality = qual.clone();
                let window = win.clone();
                let block = block.clone();
                let mut c = BrBuilder::new();
                c.mode(brotli_mode);
                c.quality(quality);
                c.lgwin(window);
                c.lgblock(block);
                BrEn::from_params(w, &c)
            }),
            Quality::FastLow => Box::new(move |w| BrEn::new(w, 2)),
            Quality::SlowHigh => Box::new(move |w| {
                let mut c = BrBuilder::new();
                c.mode(BrMode::Text);
                c.quality(11);
                c.lgwin(24);
                c.lgblock(24);
                BrEn::from_params(w, &c)
            }),
            _ => Box::new(move |w| BrEn::new(w, 5)),
        }
    }

    pub fn into_xz(self) -> u32 {
        match self {
            Quality::FastLow => 3,
            Quality::SlowHigh => 7,
            _ => 0,
        }
    }

    pub fn into_bz(self) -> BzQuality {
        match self {
            Quality::FastLow => BzQuality::Fastest,
            Quality::SlowHigh => BzQuality::Best,
            _ => BzQuality::Default,

        }
    }

    pub fn into_gz(self) -> GzQuality {
        match self {
            Quality::FastLow => GzQuality::Fast,
            Quality::SlowHigh => GzQuality::Best,
            _ => GzQuality::Default,
        }
    }
}
impl Default for Quality {
    /// This returns `Quality::Default` which may suprrise you
    fn default() -> Self {
        Quality::Default
    }
}


/// Describes the compression algortim we're working with.
///
/// This can be detected, or set by the user depending on if
/// they're compressing or decompressing.
#[derive(Clone, Debug)]
pub enum Format {
    LZW(Quality),
    LZH(Quality),
    Gzip(Quality),
    Zip7(Quality),
    Bzip2(Quality),
    Xz(Quality),
    Brotli(Quality),
    Lz4(Quality),
    Snappy(Quality),
    Zstd(Quality),
    Tar(Quality),
}
impl Format {
    /// Returns the _suggested_ or _common_ extension for a file format
    /// this isn't a _hard and fast_ rule, some are identical.
    pub fn get_extension(&self) -> &'static str {
        match self {
            &Format::LZW(_) => "z",
            &Format::LZH(_) => "z",
            &Format::Zip7(_) => "7z",
            &Format::Gzip(_) => "gz",
            &Format::Bzip2(_) => "bz2",
            &Format::Xz(_) => "xz",
            &Format::Brotli(_) => "br",
            &Format::Lz4(_) => "lz4",
            &Format::Snappy(_) => "sz",
            &Format::Zstd(_) => "zst",
            &Format::Tar(_) => "tar",
        }
    }

    /// Try to find out the format of a file
    ///
    /// This will attempt to open the file at path and read the first 16bytes
    /// matching that against a known magic number.
    ///
    /// If the file's type is unknown this method will return `Err(InvalidInput)`
    pub fn from_path<P: AsRef<Path>>(p: P) -> io::Result<Format> {
        let mut f = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(p)?;
        let mut v = Vec::with_capacity(16);
        unsafe { v.set_len(10) };
        f.read_exact(v.as_mut_slice())?;
        let _ = f;
        match what_format(v.as_slice()) {
            Option::Some(f) => Ok(f),
            Option::None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported file type",
            )),
        }
    }

    /// A slightly more efficient way to find a file's type.
    ///
    /// This method will seek to the start, read 16 bytes, then
    /// seek back to the start. The goal of this is to avoid multiple open/close cycles
    ///
    /// If the file's type is unknown this method will return `Err(InvalidInput)`
    pub fn from_reader<R: Read + Seek>(r: &mut R) -> io::Result<Format> {
        let _ = r.seek(SeekFrom::Start(0))?;
        let mut v = Vec::with_capacity(16);
        unsafe { v.set_len(16) };
        let input = r.read(v.as_mut_slice())?;
        let _ = r.seek(SeekFrom::Start(0))?;
        unsafe { v.set_len(input) }
        match what_format(v.as_slice()) {
            Option::Some(f) => Ok(f),
            Option::None => {
                let kind = io::ErrorKind::InvalidInput;
                let msg = format!("Could not identify magic number {:?}", v.as_slice());
                Err(io::Error::new(kind, msg))
            }
        }
    }
}

/*
 * LOOK I WROTE A NICE PARSER TO DO THIS AND IT BROKE
 * BETWEEN STABLE-MSVC AND STABLE-GNU
 * SO YEAH IT IS RECURSIVE DECENT
 */
fn what_format(x: &[u8]) -> Option<Format> {
    match &x[0..2] {
        b"\x1F\x9D" => return Some(Format::LZW(Quality::Default)),
        b"\x1F\xA0" => return Some(Format::LZH(Quality::Default)),
        b"\x1F\x8B" => return Some(Format::Gzip(Quality::Default)),
        b"\x30\x30" | b"\x20\x00" => return Some(Format::Tar(Quality::Default)),
        _ => {}
    };
    match &x[0..3] {
        b"\x37\x7A\xBC" | b"\xAF\x27\x1C" => return Some(Format::Xz(Quality::Default)),
        b"\x42\x5A\x68" => return Some(Format::Bzip2(Quality::Default)),
        b"\x75\x73\x74" | b"\x61\x72\x20" | b"\x61\x72\x00" => {
            return Some(Format::Tar(Quality::Default))
        }
        _ => {}
    };
    match &x[0..4] {
    b"\x81\xCF\xB2\xCE" |
    b"\xCE\xB2\xCF\x81" => return Some(Format::Brotli(Quality::Default)),
    b"\x18\x4D\x22\x04" |
    b"\x04\x22\x4D\x18" => return Some(Format::Lz4(Quality::Default)),
    b"\xB5\x28\xFD\x2F" | // Functionally _any_ block can start a zstd binary ball of joy
    b"\x28\xB5\x2F\xFD" | // this is rarely poorly documented
    b"\x27\xB5\x2F\xFD" | // 
    b"\xFD\x2F\xB5\x27" | // I just add a new entry every time I see an error
    b"\x28\xB5\x2F\xFD" |
    b"\xFD\x2F\xB5\x28" => return Some(Format::Zstd(Quality::Default)),
    _ => { }
  };
    match &x[0..6] {
        b"\xFD\x37\x7A\x58\x5A\x00" => return Some(Format::Xz(Quality::Default)),
        _ => {}
    };
    match &x[0..9] {
        b"\xFF\x06\x00\x73\x4E\x61\x50\x70\x59" => return Some(Format::Snappy(Quality::Default)),
        _ => {}
    };
    None
}

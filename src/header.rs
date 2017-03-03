
use std::path::Path;
use std::io::{
  self,
  Read
};
use std::fs::OpenOptions;
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

#[derive(Copy,Clone,Debug,PartialEq,Eq)] 
pub enum Format {
  LZW,
	LZH,
	Gzip,
	Zip7,
  Bzip2,
  Xz,
  Brotli,
  Lz4,
  Snappy,
  Zstd,
  Tar,
}
impl Format {

	/// Returns the _suggested_ or _common_ extension for a file format
	/// this isn't a _hard and fast_ rule, some are identical.
	pub fn get_extension(&self) -> &'static str {
		match self {
			&Format::LZW => "tar.z",
			&Format::LZH => "tar.z",
			&Format::Zip7 => "tar.7z",
			&Format::Gzip => "tar.gz",
			&Format::Bzip2 => "tar.bz2",
			&Format::Xz => "tar.xz",
			&Format::Brotli => "tar.br",
			&Format::Lz4 => "tar.lz4",
			&Format::Snappy => "tar.sz",
			&Format::Zstd => "tar.zst",
			&Format::Tar => "tar"
		}
	}

  /// Try to find out the format of a file
  pub fn read_format<P: AsRef<Path>>(p: P) -> io::Result<Format> {
    let mut f = OpenOptions::new().read(true).write(false).create(false).open(p)?;
    let mut v = Vec::with_capacity(12);
    unsafe{ v.set_len(12) };
    f.read_exact(v.as_mut_slice())?;
    let _ = f;
    match what_format(v.as_slice()) {
      Option::Some(f) => Ok(f),
      Option::None => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported file type"))
    }
  }
}

/*
 * LOOK I WROTE A NICE PARSER TO DO THIS AND IT BROKE
 * BETWEEN STABLE-MSVC AND STABLE-GNU
 * SO YEAH IT IS RECURSIVE DECENT FUCK OFF
 */
fn what_format(x: &[u8]) -> Option<Format> {
  match &x[0..2] {
    b"\x1F\x9D" => return Some(Format::LZW),
    b"\x1F\xA0" => return Some(Format::LZH),
    b"\x1F\x8B" => return Some(Format::Gzip),
    b"\x30\x30" |
    b"\x20\x00" => return Some(Format::Tar),
    _ => { }
  };
  match &x[0..3] {
    b"\x37\x7A\xBC" |
    b"\xAF\x27\x1C" => return Some(Format::Xz),
    b"\x42\x5A\x68" => return Some(Format::Bzip2),
    b"\x75\x73\x74" |
    b"\x61\x72\x20" |
    b"\x61\x72\x00" => return Some(Format::Tar),
    _ => { }
  };
  match &x[0..4] {
    b"\xCE\xB2\xCF\x81" => return Some(Format::Brotli),
    b"\x04\x22\x4D\x18" => return Some(Format::Lz4),
    b"\xFD\x2F\xB5\x28" => return Some(Format::Zstd),
    _ => { }
  };
  match &x[0..6] {
    b"\xFD\x37\x7A\x58\x5A\x00" => return Some(Format::Xz),
    _ => { }
  };
  match &x[0..9] {
    b"\xFF\x06\x00\x73\x4E\x61\x50\x70\x59" => return Some(Format::Snappy),
    _ => { }
  };
  None
}

			

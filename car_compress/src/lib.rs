
/*
 * Re-Export the other modules
 *
 */


extern crate lz4;
pub mod liblz4 {
    pub use super::lz4::{BlockSize as BSize, BlockMode as BMode, ContentChecksum as Checksum,
                         Decoder as Decode, Encoder as Encode, EncoderBuilder as Builder};
}

extern crate zstd;
pub mod libzstd {
    pub use super::zstd::{Decoder as Decode, Encoder as Encode};
}

extern crate snap;
pub mod libsnap {
    pub use super::snap::{Reader as Decode, Writer as Encode};
}

extern crate xz2;
pub mod libxz {
    pub use super::xz2::read::XzDecoder as Decode;
    pub use super::xz2::write::XzEncoder as Encode;
}


extern crate flate2;
pub mod libflate {
    pub use super::flate2::Compression as GzQuality;
    pub use super::flate2::write::GzEncoder as Encode;
    pub use super::flate2::read::GzDecoder as Decode;
}

extern crate brotli2;
pub mod libbrotli {
    pub use super::brotli2::write::BrotliEncoder as Encode;
    pub use super::brotli2::read::BrotliDecoder as Decode;
    pub use super::brotli2::stream::{CompressParams as Builder, CompressMode as Mode};
}

extern crate bzip2;
pub mod libbzip {
    pub use super::bzip2::write::BzEncoder as Encode;
    pub use super::bzip2::read::BzDecoder as Decode;
    pub use super::bzip2::Compression as BzQuality;
}


mod header;
mod comp;

pub use self::header::{Quality, Format};

pub use self::comp::{Decomp, Comp};

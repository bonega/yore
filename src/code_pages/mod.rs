#[cfg(feature="ebcdic")]
mod cp037;
#[cfg(feature="ebcdic")]
mod cp1140;
mod cp1250;
mod cp1251;
mod cp1252;
mod cp1253;
mod cp1254;
mod cp1255;
mod cp1256;
mod cp1257;
mod cp1258;
mod cp437;
mod cp737;
mod cp850;
mod cp852;
mod cp855;
mod cp857;
mod cp860;
mod cp861;
mod cp862;
mod cp863;
mod cp864;
mod cp865;
mod cp866;
mod cp869;
mod cp874;

#[cfg(feature="ebcdic")]
pub use cp037::*;
#[cfg(feature="ebcdic")]
pub use cp1140::*;
pub use cp1250::*;
pub use cp1251::*;
pub use cp1252::*;
pub use cp1253::*;
pub use cp1254::*;
pub use cp1255::*;
pub use cp1256::*;
pub use cp1257::*;
pub use cp1258::*;
pub use cp437::*;
pub use cp737::*;
pub use cp850::*;
pub use cp852::*;
pub use cp855::*;
pub use cp857::*;
pub use cp860::*;
pub use cp861::*;
pub use cp862::*;
pub use cp863::*;
pub use cp864::*;
pub use cp865::*;
pub use cp866::*;
pub use cp869::*;
pub use cp874::*;

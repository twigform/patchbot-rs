mod ai;
mod general;
mod gifs;
mod utility;
mod wheel;

pub use ai::{ai, promptadd, promptdel, promptview};
pub use general::{cat, consequence, hi, load_status, name, ping, status};
pub use gifs::{rgif, rgifword};
pub use utility::{help, shutdown};
pub use wheel::{wheel, wheeladd};

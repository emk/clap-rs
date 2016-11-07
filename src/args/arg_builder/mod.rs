pub use self::flag::Flag;
pub use self::opt::Opt;
pub use self::positional::Positional;
pub use self::base::{Base, BaseArg, GetBase};
pub use self::valued::{Valued, ValuedArg, GetValued};
pub use self::switched::{Switched, SwitchedArg, GetSwitched};

mod flag;
mod positional;
mod opt;
mod base;
mod valued;
mod switched;

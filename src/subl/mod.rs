pub mod error;
pub use self::error::Error;

pub mod target;
pub use self::target::target;

pub mod run;
pub use self::run::run;

use std;
type Result<T> = std::result::Result<T, Error>;

pub fn start_subl() -> Result<()> {
    let target = target()?;
    run(&target)?;
    Ok(())
}

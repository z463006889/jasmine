mod process;
mod cli;
mod utils;

use enum_dispatch::enum_dispatch;
pub use process::*;
pub use cli::*;
pub use utils::get_reader;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CommandExecutor {
    async fn execute(self)->anyhow::Result<()>;
}
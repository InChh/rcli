use enum_dispatch::enum_dispatch;

pub mod cli;
pub mod process;
pub mod utils;

pub use crate::base64::*;
pub use crate::csv::*;
pub use crate::gen_pass::*;
pub use crate::http::*;
pub use crate::jwt::*;
pub use crate::text::*;
pub use cli::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}

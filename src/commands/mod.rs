// Re-export all commands
mod def;
mod exec;
mod if_cmd;
mod print;
mod fn_def;
mod fn_call;
mod endfn;
mod input_cmd;
mod mov;
mod abort;
mod lib_call;
pub use def::DefCommand;
pub use exec::ExecCommand;
pub use if_cmd::{IfCommand, EndIfCommand};
pub use print::PrintCommand;
pub use fn_def::FnDefCommand;
pub use fn_call::FnCallCommand;
pub use endfn::EndFnCommand;
pub use input_cmd::InputCommand;
pub use lib_call::LibCallCommand;
pub use mov::MovCommand;
pub use abort::AbortCommand;
pub mod registry;

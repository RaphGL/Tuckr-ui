use tuckr::{hooks, symlinks, secrets, fileops};
use crate::Page;

/// Set command
pub use hooks::set_cmd;
/// Add command
pub use symlinks::add_cmd;
/// Rm command
pub use symlinks::remove_cmd;
/// Starus command
pub use symlinks::status_cmd;
/// Encrypt command
pub use secrets::encrypt_cmd;
/// Decrypt command
pub use secrets::decrypt_cmd;
/// From_stow command
pub use fileops::from_stow_cmd;
/// Init command
pub use fileops::init_cmd;
/// Ls_hooks command
pub use fileops::ls_hooks_cmd;
/// Ls_secrets command
pub use fileops::ls_secrets_cmd;
/// Push command
pub use fileops::push_cmd;
/// Pop command
pub use fileops::pop_cmd;
/// Gruopis command
pub use fileops::groupis_cmd;

fn run(cmd: Page) {
    
}
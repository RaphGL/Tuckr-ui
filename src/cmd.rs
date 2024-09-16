#![allow(unused)]
use crate::app::Page;
use tuckr::Cli;
use tuckr::{fileops, hooks, secrets, symlinks};

/// From_stow command
pub use fileops::from_stow_cmd;
/// Gruopis command
pub use fileops::groupis_cmd;
/// Init command
pub use fileops::init_cmd;
/// Ls_hooks command
pub use fileops::ls_hooks_cmd;
/// Ls_secrets command
pub use fileops::ls_secrets_cmd;
/// Pop command
pub use fileops::pop_cmd;
/// Push command
pub use fileops::push_cmd;
/// Set command
pub use hooks::set_cmd;
/// Decrypt command
pub use secrets::decrypt_cmd;
/// Encrypt command
pub use secrets::encrypt_cmd;
/// Add command
pub use symlinks::add_cmd;
/// Rm command
pub use symlinks::remove_cmd;
/// Starus command
pub use symlinks::status_cmd;

pub fn run(cli: Cli) -> (String, std::process::ExitCode) {
	match cli {
		Cli::Set {
			groups,
			exclude,
			force,
			adopt,
		} => hooks::set_cmd(&groups, &exclude, force, adopt),
		Cli::Add {
			groups,
			exclude,
			force,
			adopt,
		} => symlinks::add_cmd(&groups, &exclude, force, adopt),
		Cli::Rm { groups, exclude } => symlinks::remove_cmd(&groups, &exclude),
		Cli::Status { groups } => symlinks::status_cmd(groups),
		Cli::Encrypt { group, dotfiles } => secrets::encrypt_cmd(&group, &dotfiles),
		Cli::Decrypt { groups, exclude } => secrets::decrypt_cmd(&groups, &exclude),
		Cli::FromStow => fileops::from_stow_cmd(),
		Cli::Init => fileops::init_cmd(),
		Cli::LsHooks => fileops::ls_hooks_cmd(),
		Cli::LsSecrets => fileops::ls_secrets_cmd(),
		Cli::Push { group, files } => fileops::push_cmd(group, &files),
		Cli::Pop { groups } => fileops::pop_cmd(&groups),
		Cli::GroupIs { files } => fileops::groupis_cmd(&files),
	}
}

use tuckr::dotfiles::{self, ReturnCode};

/// Exclude groups sufixed with _<os_name>
// fn default_exclude() -> Vec<String> {
// 	tuckr::dotfiles::group_ends_with_target_name(group)
// }

pub fn load_groups(output: &mut String) -> Result<Vec<String>, ReturnCode> {
	let dotfiles_dir = match dotfiles::get_dotfiles_path(output) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{e}");
            return Err(ReturnCode::NoSetupFolder.into());
        }
    }
    .join("Configs");

    let groups: Vec<_> = dotfiles_dir
        .read_dir()
        .unwrap()
        .filter_map(|f| {
            let f = f.unwrap();
            if f.file_type().unwrap().is_dir() {
                Some(f.file_name().into_string().unwrap())
            } else {
                None
            }
        })
        .collect();

	// for group in &groups {
	// 	let dotfile_path = dotfiles_dir.join(group).join(&basepath);

	// 	if !dotfile_path.exists() {
	// 		continue;
	// 	}

	// 	let dotfile = match dotfiles::Dotfile::try_from(dotfile_path) {
	// 		Ok(dotfile) => dotfile,
	// 		Err(err) => {
	// 			eprintln!("{err}");
	// 			continue;
	// 		}
	// 	};

	// 	println!("{}", dotfile.group_name);
	// }
	Ok(groups)
}
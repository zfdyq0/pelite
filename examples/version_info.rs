/*!
Demonstrates how to read the version info from a PE file.
!*/

use std::env;
use std::ffi::OsStr;

fn main() {
	let mut args_os = env::args_os();
	match (args_os.next(), args_os.next(), args_os.next(), args_os.next()) {
		(Some(_), Some(path), Some(lang), None) => {
			let lang = lang.to_str().unwrap()
				.parse().expect("lang parameter");
			print_version_info(&path, lang);
		},
		(Some(_), Some(path), None, None) => {
			print_version_info(&path, 1033);
		},
		_ => {
			eprintln!("Pelite example: Print version info\nUsage: <path> [lang]\n");
		},
	}
}

fn print_version_info(path: &OsStr, lang: u16) {
	// Map the file into memory
	let file_map = pelite::FileMap::open(path)
		.expect("cannot open the file specified");

	// Interpret as a PE image
	let image = pelite::PeFile::from_bytes(file_map.as_ref())
		.expect("file is not a PE image");

	// Extract the resources from the image
	let resources = match image {
		pelite::PeFile::Pe32(pe) => {
			use pelite::pe32::Pe;
			pe.resources()
		},
		pelite::PeFile::Pe64(pe) => {
			use pelite::pe64::Pe;
			pe.resources()
		},
	}.expect("resources not found");

	// Extract the version info from the resources
	let version_info = resources.version_info(lang)
		.expect("version info not found");

	// Print the fixed file info
	if let Some(fixed) = version_info.fixed() {
		println!("VS_FIXEDFILEINFO:\n{:?}", fixed);
	}

	// Print the version info strings
	version_info.for_each_string(|lang, key, value| {
		let lang = String::from_utf16_lossy(lang);
		let key = String::from_utf16_lossy(key);
		let value = String::from_utf16_lossy(value);
		println!("[{}] {:<20} {}", lang, key, value);
	});
}

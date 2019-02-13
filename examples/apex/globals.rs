use pelite;
use pelite::pe64::*;

pub fn print(bin: PeFile) {
	for g in globals(bin) {
		println!("[Global.{:?}]", g.ty_name);
		println!("\taddress = {:#x}", g.address);
	}
}

pub struct Global<'a> {
	pub address: u32,
	pub ty_name: &'a str,
}

pub fn globals(bin: PeFile<'_>) -> Vec<Global<'_>> {
	let image = bin.image();
	let mut globals = Vec::new();
	for i in 0..image.len() / 8 {
		if let Ok(global) = global(bin, i * 8) {
			if !global.ty_name.contains("ConVar") && !global.ty_name.contains("ConCommand") && !global.ty_name.contains("type_info") {
				globals.push(global)
			}
		}
	}

	globals.sort_by_key(|g| g.ty_name);
	globals
}

fn global(bin: PeFile<'_>, offset: usize) -> pelite::Result<Global<'_>> {
	let address = bin.file_offset_to_rva(offset)?;
	let vtable_va = *bin.derva::<u64>(address)?;
	let _vtable_rva = bin.va_to_rva(vtable_va)?;
	let col_ptr = *bin.deref::<Ptr<msvc::RTTICompleteObjectLocator>>((vtable_va - 8).into())?;
	let col = bin.deref(col_ptr)?;
	let type_info = bin.derva::<msvc::TypeDescriptor>(col.type_descriptor)?;
	let _ = bin.va_to_rva(type_info.vftable.into())?;
	if type_info.spare != Ptr::null() {
		return Err(pelite::Error::Null);
	}
	let ty_name = bin.derva_c_str(col.type_descriptor + 16)?.to_str().map_err(|_| pelite::Error::Encoding)?;
	Ok(Global { address, ty_name })
}

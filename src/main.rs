use std::{
	fs::{self, OpenOptions}, 
	io::{BufWriter, Write}, 
};



fn main() {
	let file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open("C:/Users/wakun/AppData/Roaming/nushell/config.nu")
		.unwrap();

	let mut writer = BufWriter::new(file);
	
	writer.write_all(&time_prompt()).unwrap();
	writer.write_all(&path_prompt()).unwrap();
	writer.write_all("$env.PROMPT_COMMAND = {||$\"~\\..\\(path)\n>\" }".as_bytes()).unwrap();
}

fn time_prompt() -> Vec<u8>{
	let contents = fs::read("scripts/time.nu").unwrap();
	
	contents
}

fn path_prompt() -> Vec<u8>{
	let contents = fs::read("scripts/path.nu").unwrap();

	contents
}

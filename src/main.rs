use std::{
	fs::{self, OpenOptions}, 
	io::{BufWriter, Write}, 
};

use crossterm::event::{self, KeyCode};



fn main() {
	let mut terminal = ratatui::init();
	loop{
		terminal.draw(|frame|frame.render_widget("Hello world", frame.area())).unwrap();
		match event::read().unwrap(){
			event::Event::Key(key) =>{
				match key.code {
					KeyCode::Char('q') => break,
					_ => {}
				}
			},
			_ => {}
		}
	}
	ratatui::restore();
}	

fn write_file(){
	let file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open("C:/Users/wakun/AppData/Roaming/nushell/config.nu")
		.unwrap();

	let mut writer = BufWriter::new(file);
	
	writer.write_all(&time_prompt()).unwrap();
	writer.write_all(&path_prompt()).unwrap();
	writer.write_all("\n$env.PROMPT_COMMAND = {||$\"~\\\\..\\\\(path)\\n>\" }".as_bytes()).unwrap();
}

fn time_prompt() -> Vec<u8>{
	let contents = fs::read("scripts/time.nu").unwrap();
	
	contents
}

fn path_prompt() -> Vec<u8>{
	let contents = fs::read("scripts/path.nu").unwrap();

	contents
}

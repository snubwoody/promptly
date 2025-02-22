mod components;
use components::{DateTimeFormat, PathComponent, TimeComponent};

fn main() {
	let mut prompt_string = String::new();

	let path_component = PathComponent::new();
	let path_string = path_component.current_dir();

	let time_component = TimeComponent::new();

	let format = DateTimeFormat::new()
			.hours_12()
			.minutes()
			.seconds()
			.time_seperator(":");

	let now = time_component.now(format);

	let prompt_indicator = ">";

	prompt_string += &path_string;
	prompt_string += &format!(" | {now}");
	prompt_string += &format!("\n{prompt_indicator}");

	println!("{prompt_string}");
}


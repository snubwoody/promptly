mod components;
use components::{Component, PathComponent, TimeComponent};

fn main() {
	let mut prompt_string = String::new();

	let path_component = PathComponent::new();
	let path_string = path_component.output();

	let time_component = TimeComponent::new();

	let now = time_component.output();
	

	let prompt_indicator = ">";

	prompt_string += &path_string;
	prompt_string += &format!(" | {now}");
	prompt_string += &format!("\n{prompt_indicator}");

	let current = std::time::SystemTime::now();

	println!("{prompt_string}");
}


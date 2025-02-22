use super::Component;



pub struct TimeComponent;

impl TimeComponent {
	pub fn new() -> Self{
		Self
	}

	/// Get a string containing the current system time 
	fn now(&self) -> String{
		String::from("Hi")
	}
}

impl Component for TimeComponent {
	fn output(&self) -> String {
		self.now()
	}
}
use chrono::Local;
use super::Component;

// DateTime specifiers 

/// The full gregorian year e.g. `2024`
pub const FULL_YEAR_NUMBER:&str = "%Y";
/// The last two digits of the gregorian year e.g `24`
pub const POST_YEAR_NUMBER:&str = "%Y";
/// The month number, 01 - 12
pub const MONTH_NUMBER:&str = "%m";
/// The abbreviated month name, always three digits e.g. `Jul` 
pub const ABRV_MONTH_NAME:&str = "%b";
/// The full month name i.e. `November` 
pub const FULL_MONTH_NAME:&str = "%B";
/// The day number (01-31) 
pub const DAY_NUMBER:&str = "%d";
/// The abbreviated week day name, always three letters e.g. `Mon` 
pub const ABBR_DAY_NAME:&str = "%a";
/// The full week day name e.g. `Monday` 
pub const FULL_DAY_NAME:&str = "%A";
/// The hour number on 24-hour clocks(00-23) 
pub const HOUR_NUMBER_24:&str = "%H";
/// The hour number on 12-hour clocks(01-12) 
pub const HOUR_NUMBER_12:&str = "%I";
/// `AM` or `PM` in 12-hour clocks  
pub const AM_PM:&str = "%p";
/// The minute number (00-59)  
pub const MINUTE_NUMBER:&str = "%M";
/// The second number (00-60)  
pub const SECOND_NUMBER:&str = "%S";

/// Specify the date time format
#[derive(Debug,Default,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTimeFormat{
	date_seperator:Option<String>,
	time_seperator:Option<String>,
	hour_specifier:Option<String>,
	minute_specifier:Option<String>,
	second_specifier:Option<String>,
	year_specifier:Option<String>,
	month_specifier:Option<String>,
	day_specifier:Option<String>,
}

impl DateTimeFormat{
	pub fn new() -> Self{
		Self::default()
	}

	pub fn full_month(mut self) -> Self{
		self
	}

	/// Display the full gregorian year e.g. `2024`
	pub fn full_year(mut self) -> Self{
		self.year_specifier = Some(String::from(FULL_YEAR_NUMBER));
		self
	}
	
	/// Display the abbreviated gregorian year e.g. `24`
	pub fn abbreviated_year(mut self) -> Self{
		self.year_specifier = Some(String::from(POST_YEAR_NUMBER));
		self
	}
}

pub struct TimeComponent;

impl TimeComponent {
	pub fn new() -> Self{
		Self
	}

	/// Get a string containing the current system time 
	fn now(&self) -> String{
		let now = Local::now();
		
		now.format("%H:%M:%S").to_string()
	}
}

impl Component for TimeComponent {
	fn output(&self) -> String {
		self.now()
	}
}

#[cfg(test)]
mod tests{
    use crate::components::time::{FULL_YEAR_NUMBER, POST_YEAR_NUMBER};

    use super::DateTimeFormat;

	#[test]
	fn year_specifier(){
		let full_year = DateTimeFormat::new().full_year();
		let abrv_year = DateTimeFormat::new().abbreviated_year();
		
		assert_eq!(full_year.year_specifier,Some(String::from(FULL_YEAR_NUMBER)));
		assert_eq!(abrv_year.year_specifier,Some(String::from(POST_YEAR_NUMBER)));
	}
}
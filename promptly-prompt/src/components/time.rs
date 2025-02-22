use chrono::Local;

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
pub const ABRV_DAY_NAME:&str = "%a";
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
	include_am: bool
}

impl DateTimeFormat{
	pub fn new() -> Self{
		Self::default()
	}

	/// Set the seperator between the time component e.g. `12:01:01`
	pub fn time_seperator(mut self,seperator:&str) -> Self{
		self.time_seperator = Some(String::from(seperator));
		self
	}

	/// Display the minute number `0-59`
	pub fn minutes(mut self) -> Self{
		self.minute_specifier = Some(String::from(MINUTE_NUMBER));
		self
	}

	/// Display the second number `0-60`
	pub fn seconds(mut self) -> Self{
		self.second_specifier = Some(String::from(SECOND_NUMBER));
		self
	}

	/// Display the hour number as would appear on a 12-hour clock
	pub fn hours_12(mut self) -> Self{
		self.hour_specifier = Some(String::from(HOUR_NUMBER_12));
		self
	}

	/// Display the hour number as would appear on a 24-hour clock
	pub fn hours_24(mut self) -> Self{
		self.hour_specifier = Some(String::from(HOUR_NUMBER_24));
		self
	}

	/// Display the full month name e.g `November`
	pub fn full_month(mut self) -> Self{
		self.month_specifier = Some(String::from(FULL_MONTH_NAME));
		self
	}
	
	/// Display the abbreviated month name e.g `Jul`
	pub fn abrv_month(mut self) -> Self{
		self.month_specifier = Some(String::from(ABRV_MONTH_NAME));
		self
	}
	
	/// Display the month number e.g `02`
	pub fn month_number(mut self) -> Self{
		self.month_specifier = Some(String::from(MONTH_NUMBER));
		self
	}

	/// Display the full gregorian year e.g. `2024`
	pub fn full_year(mut self) -> Self{
		self.year_specifier = Some(String::from(FULL_YEAR_NUMBER));
		self
	}
	
	/// Display the abbreviated gregorian year e.g. `24`
	pub fn abrv_year(mut self) -> Self{
		self.year_specifier = Some(String::from(POST_YEAR_NUMBER));
		self
	}
}

#[derive(Debug,Clone, Copy,Default)]
pub struct TimeComponent;

impl TimeComponent {
	pub fn new() -> Self{
		Self::default()
	}

	fn format_time(&self,format:&DateTimeFormat) -> String{
		let mut format_string = String::new();

		format_string += &format.hour_specifier.clone().unwrap_or_default();
		format_string += &format.time_seperator.clone().unwrap_or_default();
		format_string += &format.minute_specifier.clone().unwrap_or_default();
		format_string += &format.time_seperator.clone().unwrap_or_default();
		format_string += &format.second_specifier.clone().unwrap_or_default();

		format_string
	}

	fn format_date(&self,format:&DateTimeFormat) -> String{
		todo!()
	}

	/// Get a string containing the current system time 
	pub fn now(&self,format:DateTimeFormat) -> String{

		let mut output_format = String::new();

		let now = Local::now();

		let time = self.format_time(&format);
		output_format.push_str(&time);

		now.format(&output_format).to_string()
	}
}

#[cfg(test)]
mod tests{
    use super::*;

	#[test]
	fn time(){
		let format = DateTimeFormat::new()
			.hours_12()
			.minutes()
			.seconds()
			.time_seperator("-");

		let time = TimeComponent::new();

		let time_format = time.format_time(&format);

		assert_eq!(time_format,String::from("%I-%M-%S"))
	}

	#[test]
	fn year_specifier(){
		let full_year = DateTimeFormat::new().full_year();
		let abrv_year = DateTimeFormat::new().abrv_year();
		
		assert_eq!(full_year.year_specifier,Some(String::from(FULL_YEAR_NUMBER)));
		assert_eq!(abrv_year.year_specifier,Some(String::from(POST_YEAR_NUMBER)));
	}
	
	#[test]
	fn month_specifier(){
		let full_month = DateTimeFormat::new().full_month();
		let abrv_month = DateTimeFormat::new().abrv_month();
		let month_number = DateTimeFormat::new().month_number();
		
		assert_eq!(full_month.month_specifier,Some(String::from(FULL_MONTH_NAME)));
		assert_eq!(abrv_month.month_specifier,Some(String::from(ABRV_MONTH_NAME)));
		assert_eq!(month_number.month_specifier,Some(String::from(MONTH_NUMBER)));
	}
}
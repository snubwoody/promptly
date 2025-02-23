use std::path::PathBuf;

/// Component for getting the current path
pub struct PathComponent{

}

impl PathComponent {
	pub fn new() -> Self{
		Self {  }
	}

	/// Get and format the current directory
	pub fn current_dir(&self) -> String{
		let current_dir = std::env::current_dir().unwrap();
	
		let mut path_string = String::new();
		let dir_components = current_dir.components().collect::<Vec<_>>();
		
		// Trim long paths
		if dir_components.len() > 3{
			let mut path = PathBuf::new();
			let length = dir_components.len();
			
			let last = dir_components.get(length - 1).unwrap();
			let second_last = dir_components.get(length - 2).unwrap();
	
			path.push(second_last);
			path.push(last);
	
			path_string.push_str("~\\..\\");
			path_string.push_str(path.to_str().unwrap());
		
		}else {
			path_string.push_str(current_dir.to_str().unwrap());
		}

		path_string
	}
}

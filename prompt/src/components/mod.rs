mod path;
mod time;
pub use path::PathComponent;
pub use time::TimeComponent;

pub trait Component {
	fn output(&self) -> String;
}


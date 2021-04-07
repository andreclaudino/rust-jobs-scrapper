use serde::{Serialize, Deserialize};
//TODO: Include job meta (salary, location, job-type, etc)
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Job {
	pub page_url: String,
	pub title: Option<String>,
	pub company: Option<String>,
	pub description: Option<String>,
}

impl Job {

	pub fn new(page_url: String, title: Option<String>, company: Option<String>, description: Option<String>) -> Job {
		Job {
			title,
			company,
			description,
			page_url
		}
	}
}
use serde::{Serialize, Deserialize};
//TODO: Include job meta (salary, location, job-type, etc)
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Job {
	pub page_url: String,
	pub title: Option<String>,
	pub company: Option<String>,
	pub description: Option<String>,
	pub main_category: Option<String>,
	pub sub_category: Option<String>,
}

impl Job {

	pub fn new(page_url: String, title: Option<String>, company: Option<String>, description: Option<String>) -> Job {
		Job {
			title,
			company,
			description,
			page_url,
			main_category: None,
			sub_category: None,
		}
	}

	pub fn set_category(&mut self, main_category: &String, sub_category: &String) {
		self.main_category = Some(main_category.clone());
		self.sub_category = Some(sub_category.clone());
	}
}
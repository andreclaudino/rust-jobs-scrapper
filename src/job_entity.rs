//TODO: Include job meta (salary, location, job-type, etc)
#[derive(Debug)]
pub struct Job {
	pub title: Option<String>,
	pub company: Option<String>,
	pub description: Option<String>,
}

impl Job {

	pub fn new(title: Option<String>, company: Option<String>, description: Option<String>) -> Job {
		Job {
			title,
			company,
			description,
		}
	}
}
use structopt::StructOpt;

const DEFAULT_INDEED_URL: &str = "https://www.indeed.com/";

#[derive(StructOpt, Debug)]
#[structopt(name = "scrap-indeed")]
pub struct Cli {

	#[structopt(long="gecko-server")]
	pub gecko_server_url: String,

	#[structopt(long="category")]
    pub category_title: String,

	#[structopt(long="job-title")]
    pub job_title: String,

	#[structopt(long="indeed-url",
				default_value=DEFAULT_INDEED_URL)]
	pub indeed_url: String
}

impl Cli {
	pub fn parse() -> Cli {
		Cli::from_args()
	}
}
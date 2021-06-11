use crate::job_entity::Job;
use crate::utils::pause;
use fantoccini::{Client, Locator};
use crate:: utils::FantocciniResult;

const TITLE_SELECTOR: &str = "h1.jobsearch-JobInfoHeader-title";
const COMPANY_SELECTOR: &str = "div.icl-u-lg-mr--sm.icl-u-xs-mr--xs > a";
const DESCRIPTION_SELECTOR: &str = "div#jobDescriptionText";
const SECONDS_TO_PAUSE: u64 = 4;


pub async fn process_job_detail_page(c: &mut Client, job_page_link: &str) -> FantocciniResult<Job> {

	c.goto(job_page_link)
		.await
		.expect(format!("Error while loading '{}' job page", job_page_link).as_str());

	let page_url = c.current_url().await?.to_string();
	let title = load_element(c, TITLE_SELECTOR).await;
	let company = load_element(c, COMPANY_SELECTOR).await;
	let description = load_element(c, DESCRIPTION_SELECTOR).await;

	pause(SECONDS_TO_PAUSE);
	c.back().await?;

	let job = Job::new(page_url, title, company, description);

	Ok(job)
}

async fn load_element(c: &mut Client, css_selector: &str) -> Option<String> {
	let find_result = c.find(Locator::Css(css_selector)).await;

	match find_result {
		Ok(mut element) => Some(element.text().await.unwrap()),
		Err(_) => None
	}
}
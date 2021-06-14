use fantoccini::{Client, Locator};
use url::{self, Url};

use crate::utils::FantocciniResult;

const EXPECTED_INDEED_COM: &str = "www.indeed.com";
const BROWSE_JOBS_SELECTOR: &str = r#"a[data-gnav-element-name="BrowseJobs"]"#;

pub async fn force_to_ideed_com(mut c: Client, indeed_url: &str) -> FantocciniResult<Client> {

	c.goto(indeed_url).await?;
	let expected_url = Url::parse(indeed_url)?;

	if c.current_url().await? != expected_url {
        c.find(Locator::LinkText(EXPECTED_INDEED_COM)).await.
			expect(format!("Could not find {} link", EXPECTED_INDEED_COM).as_str())
			.click().await
			.expect(format!("Could not click {} link", EXPECTED_INDEED_COM).as_str());
     }

	Ok(c)
}

pub async fn navigate_to_browse_jobs_page(mut c: Client) -> FantocciniResult<Client> {   
	c.find(Locator::Css(BROWSE_JOBS_SELECTOR)).await
		.expect(format!("Could not find {} link", BROWSE_JOBS_SELECTOR).as_str())
		.click().await
		.expect(format!("Could not click {} link", BROWSE_JOBS_SELECTOR).as_str());

	Ok(c)
}

pub async fn navigate_to_category_page(mut c: Client, category_title: &str) -> FantocciniResult<Client> {

	let category =
		c.find(Locator::LinkText(category_title))
			.await
			.expect(format!("Could not find '{}' title", category_title).as_str());

    category.click()
		.await
		.expect(format!("Could not click '{}' category", category_title).as_str());

	Ok(c)
}

pub async fn load_openings_page(mut c: Client, job_title: &str) -> FantocciniResult<Client> {
	c.find(Locator::LinkText(job_title))
		.await
		.expect(format!("Could not find '{}' title", job_title).as_str())
		.click()
		.await
		.expect(format!("Could not click '{}' title", job_title).as_str());

	Ok(c)
}
use fantoccini::{Client, Locator};
use url::{self, Url};

use crate::utils::FantocciniResult;

const EXPECTED_INDEED_COM: &str = "www.indeed.com";
const BROWSE_JOBS_LABEL: &str = "Browse Jobs";

pub async fn force_to_ideed_com(c: &mut Client, indeed_url: &str) -> FantocciniResult<()> {
	
	c.goto(indeed_url).await?;
	let expected_url = Url::parse(indeed_url)?;

	if c.current_url().await? != expected_url {
        c.find(Locator::LinkText(EXPECTED_INDEED_COM)).await.
			expect(format!("Could not find {} link", EXPECTED_INDEED_COM).as_str())
			.click().await
			.expect(format!("Could not click {} link", EXPECTED_INDEED_COM).as_str());
     }

	Ok(())
}

pub async fn navigate_to_browse_jobs_page(c: &mut Client) -> FantocciniResult<()> {   
	c.find(Locator::LinkText(BROWSE_JOBS_LABEL)).await
		.expect(format!("Could not find {} link", BROWSE_JOBS_LABEL).as_str())
		.click().await
		.expect(format!("Could not click {} link", BROWSE_JOBS_LABEL).as_str());

	Ok(())
}

pub async fn navigate_to_category_page(c: &mut Client, category_title: &str) -> FantocciniResult<()> {
	
	let category = 
		c.find(Locator::LinkText(category_title))
			.await
			.expect(format!("Could not find '{}' title", category_title).as_str());
    
    category.click()
		.await
		.expect(format!("Could not click '{}' category", category_title).as_str());

	Ok(())
}

pub async fn load_openings_page(c: &mut Client, job_title: &str) -> FantocciniResult<()> {
	c.find(Locator::LinkText(job_title))
		.await
		.expect(format!("Could not find '{}' title", job_title).as_str())
		.click()
		.await
		.expect(format!("Could not click '{}' title", job_title).as_str());

	Ok(())
}
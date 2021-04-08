use std::{thread::sleep, time::Duration};
use rand::seq::SliceRandom;
use std::sync::mpsc::Sender;

use fantoccini::{Client, Locator};
use crate::{job_entity::Job, job_page::process_job_detail_page, utils::FantocciniResult};
use async_recursion::async_recursion;

const DATEPOST_SELECTOR: &str = "div#filter-dateposted";
const FILTER_OPTION: &str = "Last 24 hours";
const JOB_OPENING_SELECTOR: &str = "div.jobsearch-SerpJobCard > h2.title > a";
const NEXT_PAGE_SELECTOR: &str = r#"nav[role="navigation"] > div > ul > li > a[aria-label="Next"]"#;


pub async fn set_date_filters(c: &mut Client) -> FantocciniResult<()> {
	c.find(Locator::Css(DATEPOST_SELECTOR)).await?.click().await?;
    c.find(Locator::LinkText(FILTER_OPTION)).await?.click().await?;

	Ok(())
}

pub async fn load_opening_titles(c: &mut Client) -> FantocciniResult<Vec<String>> {
	let openings_titles =
		c.find_all(Locator::Css(JOB_OPENING_SELECTOR))
			.await
			.expect(format!("Could not load list of job titles").as_str());

    let mut job_page_links = Vec::<String>::new();

	for mut opening in openings_titles {
        let link =
			opening.attr("href")
			.await?
			.unwrap();

        job_page_links.push(link);
    }

	Ok(job_page_links)
}

pub async fn apply_filters(mut c: Client) -> FantocciniResult<Client> {
	refresh(&mut c).await?;
    set_date_filters(&mut c).await?;

	Ok(c)
}

pub async fn refresh(c: &mut Client) -> FantocciniResult<()> {
	sleep(Duration::from_secs(2));
	c.refresh().await
}


#[async_recursion]
pub async fn process_openings_list_page(mut c: Client, tx: Sender<Vec<Job>>) -> FantocciniResult<()>{
    let mut job_page_links = load_opening_titles(&mut c).await?;
	job_page_links.shuffle(&mut rand::thread_rng());

	let mut jobs = Vec::<Job>::new();

    for job_page_link in job_page_links {
        let job = process_job_detail_page(&mut c, job_page_link.as_str()).await?;
		jobs.push(job);
    }

	tx.send(jobs).expect("Failed to send jobs");

	navigate_next_page(c, tx).await
}

#[async_recursion]
async fn navigate_next_page(mut c: Client, tx: Sender<Vec<Job>>) -> FantocciniResult<()> {
	let button_next_page_result = c.find(Locator::Css(NEXT_PAGE_SELECTOR)).await;

    match button_next_page_result {
        Ok(element) => {
            element.click().await?;
            process_openings_list_page(c, tx).await?;
			return Ok(());
        },
        Err(error) => {
			return Err(error);
		}
    }
}
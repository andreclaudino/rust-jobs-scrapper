use std::{thread::sleep, time::Duration};

use fantoccini::{Client, Locator};
use crate::{job_entity::Job, job_page::process_job_detail_page, utils::FantocciniResult};

const DATEPOST_SELECTOR: &str = "div#filter-dateposted";
const FILTER_OPTION: &str = "Last 24 hours";
const JOB_OPENING_SELECTOR: &str = "div.jobsearch-SerpJobCard > h2.title > a";


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

pub async fn process_openings_list_page(c: &mut Client) -> FantocciniResult<Vec<Job>>{
    refresh(c).await?;
    set_date_filters(c).await?;
    
    let job_page_links = load_opening_titles(c).await?;
    
	let mut jobs = Vec::<Job>::new();

    for job_page_link in job_page_links {
        let job = process_job_detail_page(c, job_page_link.as_str()).await?;
		jobs.push(job);
    }
        
    Ok(jobs)
}

pub async fn refresh(c: &mut Client) -> FantocciniResult<()> {
	sleep(Duration::from_secs(2));
	c.refresh().await
}

// async fn navigate_next_page() -> FantocciniResult<()> {
// 	let button_next_page_result = c.find(Locator::Css("nav > div > ul > li > a")).await;
//     match button_next_page_result {
//         Ok(element) => {
//             let next_page_link = element.clone().attr("href").await?.unwrap();
//             print!("Next page link: {}", next_page_link);
//             c.goto(next_page_link.as_str()).await?;
//             process_openings_list_page(c).await?;
//         },
//         Err(_) => print!("Finished")
//     }
// }
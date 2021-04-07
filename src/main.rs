mod cli;
mod first_navigates;
mod openings_navigate;
mod utils;
mod job_entity;
mod job_page;
use serde_json;

use cli::Cli;
use fantoccini::{ClientBuilder, Client};
use first_navigates::{force_to_ideed_com, load_openings_page, navigate_to_browse_jobs_page, navigate_to_category_page};
use openings_navigate::process_openings_list_page;
use tokio;
use utils::FantocciniResult;


#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {

    let args = cli::Cli::parse();
    let c = &mut ClientBuilder::native().connect(args.gecko_server_url.as_str()).await.unwrap();

    match process_page(c, args).await {
        Ok(_) => {
            c.close().await
        },
        Err(_) => {
            c.close().await
        }
    }
}

async fn process_page(c: &mut Client, args: Cli) -> FantocciniResult<()> {
    force_to_ideed_com(c, args.indeed_url.as_str()).await?;
    navigate_to_browse_jobs_page(c).await?;
    navigate_to_category_page(c, args.category_title.as_str()).await?;
    load_openings_page(c, args.job_title.as_str()).await?;
    let jobs = process_openings_list_page(c).await?;

    for job in jobs {
        println!("{}", serde_json::to_string(&job)?);
        println!("\n");
    }

    Ok(())
}
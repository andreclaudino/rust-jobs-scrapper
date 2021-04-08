mod cli;
mod first_navigates;
mod openings_navigate;
mod utils;
mod job_entity;
mod job_page;
use serde_json;
use std::sync::mpsc::channel;

use cli::Cli;
use fantoccini::{ClientBuilder, Client};
use first_navigates::{force_to_ideed_com, load_openings_page, navigate_to_browse_jobs_page, navigate_to_category_page};
use openings_navigate::{apply_filters, process_openings_list_page};
use tokio;
use utils::FantocciniResult;
use uuid::Uuid;
use std::io::Write;
use std::fs::File;


#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {

    let args = cli::Cli::parse();
    let mut c = ClientBuilder::native().connect(args.gecko_server_url.as_str()).await.unwrap();
    
    match process_page(c.clone(), args).await {
        Ok(_) => {
            c.close().await
        },
        Err(error) => {
            println!("An error happened: {}", error);
            c.close().await
        }
    }
}

async fn initial_processing(c: Client, args: Cli) -> FantocciniResult<Client>{
    let c = force_to_ideed_com(c, args.indeed_url.as_str()).await?;
    let c = navigate_to_browse_jobs_page(c).await?;
    let c = navigate_to_category_page(c, args.category_title.as_str()).await?;
    let c = load_openings_page(c, args.job_title.as_str()).await?;
    let c = apply_filters(c).await?;

    Ok(c)
}

async fn process_page(c: Client, args: Cli) -> FantocciniResult<()> {
    let output_folder = args.output_folder.clone();
    let c = initial_processing(c.clone(), args).await?;

    let (jobs_sender, jobs_receiver) = channel();

    tokio::spawn(async move {
        process_openings_list_page(c, jobs_sender).await.unwrap();
    });
    
    while let Ok(jobs) = jobs_receiver.recv() {

        let file_path = format!("{}/{}.json", output_folder, Uuid::new_v4());
        let file_create_error_message = format!("Erro creating file {}", file_path);
        let mut output_file = File::create(file_path).expect(file_create_error_message.as_str());

        for job in jobs.iter() {
            let buffer = format!("{}\n", serde_json::to_string(job).unwrap());
            output_file.write(buffer.as_bytes())?;
        }
    }

    Ok(())
}
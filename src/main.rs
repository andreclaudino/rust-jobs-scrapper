mod cli;
mod first_navigates;
mod openings_navigate;
mod utils;
mod job_entity;
mod job_page;
mod gs_client;

use serde_json;
use std::sync::mpsc::channel;

use cli::Cli;
use fantoccini::{Client, ClientBuilder, Locator};
use first_navigates::{force_to_ideed_com, load_openings_page, navigate_to_browse_jobs_page, navigate_to_category_page};
use openings_navigate::{apply_filters, process_openings_list_page};
use tokio;
use utils::FantocciniResult;
use uuid::Uuid;
use std::io::Write;
use std::fs::File;


const ACCEPT_COOKIES_SELECTOR : &str = "#onetrust-accept-btn-handler";

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {

    let args = cli::Cli::parse();
    let connection_error_message = format!("Error connecting to webdriver at {}", args.gecko_server_url);
    let mut c = ClientBuilder::native().connect(args.gecko_server_url.as_str()).await.expect(connection_error_message.as_str());

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

async fn initial_processing(c: Client, args: &Cli) -> FantocciniResult<Client>{
    let c = accept_cookies(c).await?;
    let c = force_to_ideed_com(c, args.indeed_url.as_str()).await?;
    let c = navigate_to_browse_jobs_page(c).await?;
    let c = navigate_to_category_page(c, args.category_title.as_str()).await?;
    let c = load_openings_page(c, args.job_title.as_str()).await?;
    let c = apply_filters(c).await?;

    Ok(c)
}

async fn accept_cookies(mut c: Client) -> FantocciniResult<Client>{
    match c.find(Locator::Css(ACCEPT_COOKIES_SELECTOR)).await {
        Ok(button) => {
            button.click().await?;
        },
        Err(_) => {}
    };

    Ok(c)
}

async fn process_page(c: Client, args: Cli) -> FantocciniResult<()> {
    let c = initial_processing(c.clone(), &args).await?;

    let (jobs_sender, jobs_receiver) = channel();

    tokio::spawn(async move {
        process_openings_list_page(c, jobs_sender).await.unwrap();
    });

    let gcloud_client = cloud_storage::Client::default();

    let output_folder = args.output_folder.clone();
    let no_protocol_output_folder =
        output_folder
            .clone()
            .replacen("gs://", "", 1);

    let parsed_output_folder: Vec<_> =
        no_protocol_output_folder
        .splitn(2, "/")
        .collect();

    let bucket = parsed_output_folder.get(0).unwrap().to_string();
    let key_preffix = parsed_output_folder.get(1).unwrap();

    while let Ok(mut jobs) = jobs_receiver.recv() {

        let file_name = format!("{}.json", Uuid::new_v4());
        let local_file_path = std::env::temp_dir().join(&file_name);
        let key = format!("{}/{}", key_preffix, file_name);

        let file_create_error_message = format!("Erro creating file {:?}", local_file_path);
        let mut temporary_file = File::create(&local_file_path).expect(file_create_error_message.as_str());

        for job in jobs.iter_mut() {
            job.set_category(&args.category_title, &args.job_title);
            let buffer = format!("{}\n", serde_json::to_string(job).unwrap());
            temporary_file.write(buffer.as_bytes())?;
        }

        match gs_client::upload_to_gs(&gcloud_client, &local_file_path, &bucket, &key).await {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error while uploading file gs://{}/{}: {}", &bucket, &key, err)
            }
        };
        std::fs::remove_file(local_file_path)?;
    };


    Ok(())
}
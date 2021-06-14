use std::{thread::sleep, time::Duration};

use fantoccini::Client;

pub type FantocciniResult<T> = Result<T, fantoccini::error::CmdError>;


pub async fn refresh(c: &mut Client) -> FantocciniResult<()> {
	pause(2);
	c.refresh().await
}

pub fn pause(seconds: u64) {
	let duration = Duration::from_secs(seconds);
	sleep(duration);
}
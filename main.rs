use log::{error, info, trace, warn};
use rand::Rng;
use reqwest::blocking::get;
use serde_json::{from_slice, json, to_string_pretty, Value};
//use simple_logger::SimpleLogger;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use chrono::{Local, TimeZone};
fn tz_offset() -> String {
    // Get the local timezone and current datetime
    let local_dt = Local::now();
    let local_tz = local_dt.timezone();
    // Get the offset in seconds from UTC
    let offset = local_tz
        .offset_from_utc_datetime(&local_dt.naive_utc())
        .local_minus_utc();

    //Converting The Offset Which Is In Seconds Into Hours
    let offset_hrs = offset / 3600;
    let sign = if offset >= 0 { "+" } else { "-" };

    let offset_strng = format!("(GMT {sign}{offset_hrs})");
    offset_strng
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    //SimpleLogger::new().init().unwrap();


    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp_nanos()
        .init();
    let tz_ofst = tz_offset();
    warn!("The Time Zone Offset Is : {tz_ofst}");
    // Replace this with the actual URL you want to request
    let url = "https://webscraper.io/test-sites/e-commerce/allinone";
    let wrt_vl = json!({
        "Counter": 0

    });
    let _ = fs::write("./counter.json", to_string_pretty(&wrt_vl).unwrap());
    loop {
        let f = fs::read("./counter.json").unwrap();
        let mut cntr: Value = from_slice(&f).unwrap();
        let calls = cntr["Counter"].take();
        let mut num = calls.as_u64().unwrap();
        info!("Number From The File is: {num}\n");
        num += 1;
        if num >= 1480 {
            warn!("Reached 1480 Requests\n");

            break;
        }

        let wrt_vl = json!({
            "Counter":num

        });

        // Log an info message
        info!("Iteration Number: {num}\n");

        // Log a warning message
        //warn!("This is a warning message.");

        // Log an error message
        //error!("This is an error message.");

        let _ = fs::write("./counter.json", to_string_pretty(&wrt_vl).unwrap());

        // Generate a random delay between 55 and 85 seconds
        let delay = rand::thread_rng().gen_range(55..=85);

        // Send the GET request
        let response = get(url)?;

        // Ensure the response status is successful (200 OK)
        if !response.status().is_success() {
            error!(
                "Error: Request failed with status code: {}\n",
                response.status()
            );
            continue; // Skip to the next iteration
        }
        trace!("Sleeping For : {delay} Seconds !!!\n");

        sleep(Duration::from_secs(delay));
        // Get the response body as a string
        let body = response.text()?;

        // Get the current directory
        let current_dir = std::env::current_dir()?;

        // Construct the filename for the output file
        let filename = current_dir.join("response.json");

        // Create the output file
        let mut file = File::create(filename)?;

        // Write the response body to the file
        file.write_all(body.as_bytes())?;
        info!("Response successfully written to response.json\n");
    }
    Ok(())
}

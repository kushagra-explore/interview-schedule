use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};


use crate::CandidateInterviewDetails;

pub fn write_interview_details_csv(interview_details: Vec<CandidateInterviewDetails>) -> Result<(), Box<dyn Error>> {
    let file_name = format!("interview_details-{}.csv", SystemTime::now().duration_since(UNIX_EPOCH).expect("Time Went backwards").as_secs());
    let mut wtr = csv::Writer::from_path(file_name.clone())?;
    for record in interview_details {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    println!("CSV with interview details: {}", file_name);

    Ok(())

}
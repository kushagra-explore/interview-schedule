use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};


use crate::CandidateInterviewDetails;

pub fn write_interview_details_csv(interview_details: Vec<CandidateInterviewDetails>) -> Result<(), Box<dyn Error>> {
    let file_name = format!("interview_details-{}.csv", SystemTime::now().duration_since(UNIX_EPOCH).expect("Time Went backwards").as_secs());
    let mut wtr = csv::Writer::from_path(file_name.clone())?;
    wtr.write_record(&["Candidate Name", "R1 Interviewer ", "R1 Shadow", "R1 Time Slot", "R2 Interviewer ", "R2 Shadow", "R2 Time Slot"])?;
    for record in interview_details {
        if record.interview_details.is_empty() {
            wtr.write_record(&[record.candidate.name])?;
        } else if record.interview_details.len() == 1 {
            let interview_detail = &record.interview_details[0];
            wtr.write_record(&[record.candidate.name, interview_detail.interviewer.clone(),"".to_string(), interview_detail.slot_human_friendly.clone()])?;
        }
        else {
            let interview_detail_1 = &record.interview_details[0];
            let interview_detail_2 = &record.interview_details[1];
            wtr.write_record(&[record.candidate.name, interview_detail_1.interviewer.clone(), "".to_string(), interview_detail_1.slot_human_friendly.clone(),
            interview_detail_2.interviewer.clone(), "".to_string(), interview_detail_2.slot_human_friendly.clone()])?;
           
        }
    }

    wtr.flush()?;
    println!("CSV with interview details: {}", file_name);

    Ok(())

}
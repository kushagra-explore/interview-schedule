use std::{
    error::Error, ffi::OsString, fs::File, io,
};

use crate::{Candidate, Experience, InterviewLogistics, InterviewRound, Interviewer};

const ALL_SLOTS_STRING: &str = "all";

const CANDIDATE_ARG   : usize = 1;
const INTERVIEWER_ARG : usize = 2;

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
#[cfg(test)]
fn get_arg(pos: usize) -> Result<OsString, Box<dyn Error>> {
    println!("Running in test mode");
    if pos == CANDIDATE_ARG {
        return Ok(OsString::from(r#"C:\Users\kvarshney\repos\scheduling_interview\src\SamplePanelCan.csv"#))
    }
    Ok(OsString::from(r#"C:\Users\kvarshney\repos\scheduling_interview\src\SamplePanelIn.csv"#))
}

#[cfg(not(test))]
fn get_arg(pos: usize) -> Result<OsString, Box<dyn Error>> {
    match std::env::args_os().nth(pos) {
        None => Err(From::from(format!("Expected {} argument, but got {}", pos, std::env::args().len()))),
        Some(file_path) => Ok(file_path),
    }
}

pub fn get_csv_data_candidate(interview_logistics: &mut InterviewLogistics) -> Result<(), Box<dyn Error>> {
    let file_path = get_arg(CANDIDATE_ARG)?;
    println!("Reading from file: {:?}", file_path);
    let file = File::open(file_path)?;
    println!("Reading from file done");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let name = String::from(&record[3]);
        let experience = convert_to_exp(&record[2]);
        let slot = convert_to_slots(&record[5]);
        match experience {
            Ok(exp) => {
                interview_logistics.add_candidate(Candidate::new(name, exp, slot))
            },
            Err(_err) => {
                  println!("Invalid exp {} for record: {}", &record[0], &record[2]);
                  continue
            }
        }
        
    }
    Ok(())
}

pub fn get_csv_data_interview(interview_logistics: &mut InterviewLogistics) -> Result<(), Box<dyn Error>> {
    let file_path = get_arg(INTERVIEWER_ARG)?;
    println!("Reading from file: {:?}", file_path);
    let file = File::open(file_path)?;
    println!("Reading from file done");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let name = String::from(&record[3]);
        let experience = convert_to_exp(&record[1])?;
        let slot = convert_to_slots(&record[5]);
        let eligible_rounds =  match &record[2] {
            "R1" => Some(InterviewRound::R1),
            "R2" => Some(InterviewRound::R2),
            "R3" => Some(InterviewRound::R3),
            _   => None
        };

        match eligible_rounds {
            Some(eligible_r) => {
                interview_logistics.add_interviewer(Interviewer::new(name, experience, slot, eligible_r))
            },
            None => {
                  println!("Invalid interviewer data {} for record: {}", &record[0], &record[2]);
                  continue
            }
        }
        
    }
    Ok(())
}

fn convert_to_exp(exp: &str) -> Result<Experience, Box<dyn Error>> {
    match exp {
        "SE1" | "SE" => Ok(Experience::SE1),
        "SE2" => Ok(Experience::SE2),
        "Senior" | "SSE" => Ok(Experience::Senior),
        _ => Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Invalid Experience")))
    }
}
/// Converts a string of slots to an array of slots of size 13.
/// Slots string is expected to be in the format "1;2;3;4;5" and so on
/// Returns a vector of slots
fn convert_to_slots(slots: &str) -> [bool; 13]  {
    
    if slots.to_lowercase() == ALL_SLOTS_STRING {
       return [true; 13];
    }
    let mut slot_array = [false; 13];
    let slots: Vec<&str> = slots.split(";").collect();

    for slot in slots {
        let slot = slot.parse::<usize>().unwrap();
        if slot > 12 {
            println!("Invalid slot {}", slot);
            continue;
        }
        slot_array[slot] = true;
    }
    slot_array
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate() {
        let mut interview_logistics = InterviewLogistics::new();
        get_csv_data_candidate(&mut interview_logistics).unwrap();
        get_csv_data_interview(&mut interview_logistics).unwrap();
        assert_eq!(interview_logistics.candidate.len(), 10);
        println!("Candidates : {:?}", interview_logistics.candidate);
        println!("Interview se2: {:?}", interview_logistics.se2_interviewers);
        println!("Interview senior: {:?}", interview_logistics.senior_interviewers);
    }
}
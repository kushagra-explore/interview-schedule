
use std::{cmp::Reverse, collections::{BTreeMap, HashMap, HashSet, VecDeque}, fmt::{self, Display}};
use serde::{Serialize, Deserialize};
use colored::*;
use crate::csv_reader::{get_csv_data_candidate, get_csv_data_interview};
use crate::csv_writer::write_interview_details_csv;

mod csv_reader;
mod csv_writer;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
enum Experience {
    SE1 = 0,
    SE2 = 1,
    Senior = 2
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
enum InterviewRound {
    #[serde(rename = "R1")]
    R1 = 0,
    #[serde(rename = "R2")]
    R2 = 1,
    #[serde(rename = "R3")]
    R3 = 2
}

impl std::fmt::Display for InterviewRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
struct Candidate {
    serial_num: u8,
    name: String,
    experience: Experience,
    ///
    /// Slots can only be hourly, like {1->9, 2->930, 3->10, 4->1030} Odd Slots-> Hour time, Even Slots->30 mins past the hour
    /// So, 1->9 means 9-10, 2->930 means 9:30-10:30, 3->10 means 10-11, 4->1030 means 10:30-11:30
    /// Slot nth = 8:30 + (n/2) *60 mins
    /// if the avail[1] = true, X is available for the slot
    ///
    availability: [bool; 13],
    schedule: Vec<u8>,
}
#[derive(Debug, Eq, PartialEq, Clone)]
struct Interviewer {
    name: String,
    interview_experience: Experience,
    /*
    * Slots can only be hourly, like {1->9, 2->930, 3->10, 4->1030} Odd Slots-> Hour time, Even Slots->30 mins past the hour
    * So, 1->9 means 9-10, 2->930 means 9:30-10:30, 3->10 means 10-11, 4->1030 means 10:30-11:30
    * Slot nth = 8:30 + (n/2) *hr(60 mins)
    * if the avail[1] = true, X is available for the slot
    */
    availability: [bool; 13],
    eligible_rounds: InterviewRound,
    schedule: Vec<u8>,
    interviews_count: u8,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CandidateInterviewDetails {
    candidate: Candidate,
    interview_details: Vec<InterviewDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct InterviewDetail {
    interviewer: String,
    slot: u8,
    slot_human_friendly: String,
    round: InterviewRound
}

impl InterviewDetail {
    fn new(interviewer: String, slot: u8, round: InterviewRound) -> InterviewDetail {
        InterviewDetail {
            interviewer,
            slot,
            slot_human_friendly: format!("{hour:0>2}:{min:0>2}",  hour = convert_to_24_hour_format(8 + (slot as f32/2.0).ceil() as u8), min=  30 * (1- slot % 2)),
            round: round
        }
    }
}

impl Display for InterviewDetail {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {} {} {} {} {}", "interviewing with".blue(), self.interviewer.yellow(),
        "at slot".green(), self.slot.to_string().yellow(), "for round".red(), self.round, "at time".green(), self.slot_human_friendly.yellow())
    }
}

impl Display for CandidateInterviewDetails {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {:?}", self.candidate.name.yellow(), self.interview_details)
    }
}

/// Interviews start at 9 hence any hour time before that in wall clock mode must be post meridian
fn convert_to_24_hour_format(hour: u8) -> u8 {
    
    if hour > 8 {
        hour
    } else {
        hour + 12
    }
    
}

#[derive(Debug)]
struct InterviewLogistics {
    senior_interviewers: VecDeque<Interviewer>,
    se2_interviewers: VecDeque<Interviewer>,
    candidate: VecDeque<Candidate>,
    interviewer_candidate_map: HashMap<String, HashSet<String>>
}

impl InterviewLogistics {
    fn new() -> InterviewLogistics {
        InterviewLogistics {
            senior_interviewers: VecDeque::new(),
            se2_interviewers: VecDeque::new(),
            candidate: VecDeque::new(),
            interviewer_candidate_map: HashMap::new()
        }
    }

    fn add_interviewer(&mut self, interviewer: Interviewer) {
        match interviewer.interview_experience {
            Experience::Senior => {
                self.senior_interviewers.push_back(interviewer);   
            }
            Experience::SE2 => {
                self.se2_interviewers.push_back(interviewer);
            }
            _ => println!("Invalid Interviewer, Not including in the list, but continuing, Pls review the list {:?}", interviewer)
        }
    }

    fn add_candidate(&mut self, candidate: Candidate) {
        self.candidate.push_back(candidate);
    }

    fn sort_interviewers(&mut self) {
        self.senior_interviewers.make_contiguous().sort_by_key(|i: &Interviewer| {
            (Reverse(i.interviews_count), i.first_available_slot().unwrap_or(100))
        });

        self.se2_interviewers.make_contiguous().sort_by_key(|i: &Interviewer| {
            (Reverse(i.interviews_count), i.first_available_slot().unwrap_or(100), )
        });
    }

    fn sort_candidates(&mut self) {
        self.candidate.make_contiguous().sort_by_key(|c: &Candidate | {
            (Reverse(c.experience), c.first_available_slot().unwrap_or(100))
        });
    }

    fn get_interviewer(&mut self, exp: Experience) -> Option<Interviewer> {
        match exp {
            Experience::Senior => self.senior_interviewers.pop_front(),
            Experience::SE2 | Experience::SE1 => {
                if self.se2_interviewers.is_empty() {
                    self.senior_interviewers.pop_front()
                } else if self.senior_interviewers.is_empty() {
                    self.se2_interviewers.pop_front()
                } else {
                    if self.senior_interviewers[0].interviews_count > self.se2_interviewers[0].interviews_count {
                        self.senior_interviewers.pop_front()
                    } else if self.se2_interviewers[0].first_available_slot().unwrap_or(100) > self.senior_interviewers[0].first_available_slot().unwrap_or(100) {
                        self.senior_interviewers.pop_front()
                    }
                    else {
                        self.se2_interviewers.pop_front()
                    }
                }
            },
        }
    }

    fn get_candidate(&mut self) -> Option<Candidate> {
        self.candidate.pop_front()
    }
}

impl Interviewer {
    fn new(name: String, interview_experience: Experience, availability: [bool; 13], eligible_rounds: InterviewRound) -> Interviewer {
        Interviewer {
            name,
            interview_experience,
            availability,
            eligible_rounds,
            schedule: Vec::new(),
            interviews_count: 3
        }
    }

    fn first_available_slot(&self) -> Option<u8> {
        for slot in 1 .. self.availability.len() {
            if self.availability[slot] {
                return Some(slot as u8);
            }
        }
        None
    }
    
}

impl Candidate {
    fn new(serial_num: u8, name: String, experience: Experience, availability: [bool; 13]) -> Candidate {
        Candidate {
            serial_num: serial_num,
            name,
            experience,
            availability,
            schedule: Vec::new()
        }
    }
    #[allow(dead_code)]
    fn new_with_full_availability(serial_num: u8, name: String, experience: Experience) -> Candidate {
        Candidate {
            serial_num: serial_num,
            name,
            experience,
            availability : [true; 13],
            schedule : Vec::new()
        }
    }

    fn first_available_slot(&self) -> Option<u8> {
        for slot in 1 .. self.availability.len() {
            if self.availability[slot] {
                return Some(slot as u8);
            }
        }
        None
    }
    
}

/// Used for grouping interviews of a candidate together, This makes output CSV more readable
// fn sort_interview_details(interview_details: &mut Vec<InterviewDetail>) {
//     interview_details.sort_by_key(|i: &InterviewDetail| {
//         i.candidate.clone()
//     });
// }


fn main() {

   

    let mut interview_logistics = InterviewLogistics::new();

    // if let Err(err) = example(interview_logistics) {
    //     println!("Error reading CSV {}", err);
    //     process::exit(1);
    // }
    
    if let Err(err) = get_csv_data_interview(&mut interview_logistics) {
        panic!("Error reading Candidate CSV {}", err);
    }

    if let Err(err) = get_csv_data_candidate(&mut interview_logistics) {
        panic!("Error reading Interviewer CSV {}", err);
    }
    

    interview_logistics.sort_interviewers();
    interview_logistics.sort_candidates();

    // Map for storing the interview details for each candidate, serial number being the key
    let mut interview_details : BTreeMap<u8, CandidateInterviewDetails> = BTreeMap::new(); 

    loop {

        let mut candidate = match interview_logistics.get_candidate() {
            Some(c) => c,
            None => break
        };

        let mut is_allocated = false;

        println!("{} {} {} {}","Allocation for candidate".cyan(), candidate.name.red(),  "for round".cyan(), (candidate.schedule.len() +1).to_string().green());

        loop{

            //Creating mutable reference to borrow candidate here
            //let cand_ref = &mut candidate;

            let mut interviewer = match interview_logistics.get_interviewer(candidate.experience) {
                Some(i) => i,
                None => {
                    println!("{}",
                        format!("---------No interviewer available for this candidate {} exp: {:?}", candidate.name, candidate.experience).red());
                    break
                }
            };

            if interviewer.interviews_count == 0 {
                println!("---------This interviewer cant take more interviews {}", interviewer.name);
                continue
            }

            if let Some(candidate_list) = interview_logistics.interviewer_candidate_map.get(&interviewer.name) {
                if candidate_list.contains(&candidate.name) {
                    println!("---------This interviewer is already assigned to this candidate, Skipping this interviewer {}", interviewer.name);
                    continue
                }
            }

            let slot = can_interview(&candidate, &interviewer);

            println!("---------For Interviewer {}, value of can_interview : {:?}", interviewer.name.to_string().red(), slot);

            match slot {
                None => {
                    interview_logistics.add_interviewer(interviewer);
                    continue
                },
                Some(-1) => {
                    println!("{}", "This is unexpected to have someone who cant take interview to reach till here, debug the issue".bold().red());
                    println!("Retrying the allocation for candidate, But not the interviewer {:?}", interviewer);
                    continue
                },
                Some(0) => {
                    interview_logistics.add_interviewer(interviewer);
                    break
                },
                Some(slot) => {
                    is_allocated = true;
                    candidate.schedule.push(slot as u8);
                    interviewer.schedule.push(slot as u8);
                    interviewer.interviews_count -= 1;
                    interviewer.availability[slot as usize] = false;
                    candidate.availability[slot as usize] = false;
                    if slot < 12 {
                        interviewer.availability[slot as usize + 1] = false;
                        candidate.availability[slot as usize + 1] = false;
                    }

                    if slot > 1{
                        interviewer.availability[slot as usize - 1] = false;
                        candidate.availability[slot as usize - 1] = false;
                    }
                    //Want to give interviewer and candidates some buffer time between interviews
                    if slot < 11 {
                        interviewer.availability[slot as usize + 2] = false;
                        candidate.availability[slot as usize + 2] = false;
                    } 
                    
                    if slot > 2{
                        interviewer.availability[slot as usize - 2] = false;
                        candidate.availability[slot as usize - 2] = false;
                    }

                    interview_logistics.interviewer_candidate_map.entry(interviewer.name.clone())
                    .or_insert(HashSet::new()).insert(candidate.name.clone());
                    interview_details.entry(candidate.serial_num).or_insert(CandidateInterviewDetails{candidate: candidate.clone(), 
                        interview_details: Vec::new()});
                    interview_details.get_mut(&candidate.serial_num).unwrap().interview_details.push(InterviewDetail::new(interviewer.name.clone(), slot as u8,
                    match candidate.schedule.len() {
                        1 => InterviewRound::R1,
                        2 => InterviewRound::R2,
                        3 => InterviewRound::R3,
                        _ => InterviewRound::R1
                    }));
                    println!("{}",
                        format!("---------Interviewer {} is allocated for candidate {} at slot {}", interviewer.name, candidate.name, slot).blue());
                    if interviewer.interviews_count > 0 {
                        interview_logistics.add_interviewer(interviewer);
                    }
                    break
                }
            }
        }

        if !is_allocated {
            println!("---------No interviewer allocation for this candidate {:?}", candidate.name);
        }

        if is_allocated && candidate.schedule.len() < 2{
            println!("---------Adding candidate back to the queue for next round of interview");
            interview_logistics.add_candidate(candidate);
        }
       
    
    }

    //sort_interview_details(&mut interview_details);

    for interview_detail in interview_details.iter()  {
        println!("{} {}", interview_detail.0, interview_detail.1);
    }

    write_interview_details_csv(interview_details.into_values().collect()).unwrap();

}

//Return slot if availble else 0
fn can_interview(candidate: &Candidate, interviewer: &Interviewer) -> Option<i8> {
    // Check round eligibility
    if candidate.experience > interviewer.interview_experience { 
       return None;
    }
    if interviewer.interviews_count == 0 {
        return Some(-1);
    }
    if (interviewer.eligible_rounds as usize) < candidate.schedule.len() {
        return Some(-2);
    }
    let slot = match_slot(&candidate.availability, &interviewer.availability);
    if slot < 1 {
        return Some(0);
    }

    //Uncomment if some interviewers are eligible for other rounds 
    // if candidate.schedule.len() >= interviewer.eligible_rounds as usize {
    //     return false;
    // }

    Some(slot)
}

fn match_slot(candidate_slot : &[bool], interviewer_slot: &[bool]) -> i8 {
    for slot in 1 .. candidate_slot.len() {
        if candidate_slot[slot] && interviewer_slot[slot] {
            return slot as i8;
        }
    }
    0 //No slot available
}
/*Given a list of interviewers and candidates with their preferred availability
Candidates ranked based on experience as SE1/SE2/Senior
Interviewers marked eligible for R1/R2/R3 , SE1/SE2/Senior, ranked based on interviewing experience
Some interviewers are shadow and some are shadow-eligible
Max 3 interviews per person, if out of interviewers, batch the interviews  */
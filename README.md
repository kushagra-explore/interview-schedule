# interview-schedule
## Run like this, providing Candidate CSV and Interviewer CSV in the same order 
cargo run "C:\Users\kvarshney\repos\scheduling_interview\src\SamplePanelCan.csv" "C:\Users\kvarshney\repos\scheduling_interview\src\SamplePanelIn.csv"

## Candidate CSV sample 
| S.No | Level | Role | Candidate Name | Exp | Slot Available |
|------|-------|------|----------------|-----|----------------|
| 1    | L63   | SSE  | C1             | 9   | 2;3;4          |

Level: L59, L60 and so on
Role: SE1, SE2, SSE only

## Interviewer CSV sample
| S.No | Level | Eligible Rounds | Interview Name | Exp | Slot Available |
|------|-------|----------------|----------------|-----|----------------|
| 1    | SE2   | R2             | I1             | 2   | All            |

Level: SE1, SE2, SSE only
Eligible Rounds: R2 and R3 only


### How to convert time to slots
Slots are numbered from 1 to 13 in whole numbers. 
Slots can only be hourly, like {1->9, 2->930, 3->10, 4->1030} Odd Slots-> Hour time, Even Slots->30 mins past the hour
1->9 means 9-10, 2->930 means 9:30-10:30, 3->10 means 10-11, 4->1030 means 10:30-11:30
Slot nth = 8:30 + (n/2) *60 mins
If someone is avaialable for full day provide *"all"* as in the slot available column

### Limitations
1. Currently shadow scheduling is not available
2. Slots have to be provided in the integer formats, However this can be changed to some human friendly input
3. Only supports R1 and R2 scheduling
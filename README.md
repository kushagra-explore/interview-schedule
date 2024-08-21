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


### Sample Inputs and outputs

#### Candidate Sample
| S.No | Level    | Role | Candidate Name | Exp | Slot Available |
|------|----------|------|----------------|-----|----------------|
| 1    | L63      | SSE  | C1             | 9   | 2;3;4          |
| 2    | L63      | SSE  | C2             | 8.5 | 1;2;3          |
| 3    | L63      | SSE  | C3             | 7   | 1;2;3;4;5      |
| 4    | L63      | SSE  | C4             | 7   | 4;5            |
| 5    | L63      | SSE  | C5             | 7   | 1;2;3;5;7      |
| 6    | L61/62   | SE2  | C6             | 6.9 | 1;4;7          |
| 7    | L61/62   | SE2  | C7             | 6   | 1;2;5;6        |
| 8    | L61/62   | SE2  | C8             | 6   | 3;6            |
| 9    | L61/62   | SE2  | C9             | 6   | 2              |
| 10   | L61/62   | SE2  | C10            | 6   | 1              |

#### Interviewer Sample
| S.No | Level | Eligible Rounds | Interview Name | Exp | Slot Available |
|------|-------|----------------|----------------|-----|----------------|
| 1    | SE2   | R2             | I1             | 2   | All            |
| 2    | SE2   | R2             | I2             | 2   | All            |
| 3    | SE2   | R2             | I3             | 2   | All            |
| 4    | SSE   | R2             | I4             | 2   | All            |
| 5    | SSE   | R2             | I5             | 2   | 2;3;4          |

#### Interview Details Output 
| Interviewer | Candidate | Slot | Slot_human_friendly |
|-------------|-----------|------|---------------------|
| I5          | C1        | 3    | 10:0                |
| I3          | C10       | 1    | 9:0                 |
| I4          | C2        | 1    | 9:0                 |
| I5          | C3        | 2    | 9:30                |
| I4          | C4        | 4    | 10:30               |
| I4          | C5        | 2    | 9:30                |
| I1          | C6        | 1    | 9:0                 |
| I3          | C6        | 4    | 10:30               |
| I2          | C7        | 1    | 9:0                 |
| I1          | C7        | 5    | 11:0                |
| I2          | C8        | 3    | 10:0                |
| I3          | C8        | 6    | 11:30               |
| I1          | C9        | 2    | 9:30                |

#### Identified Bugs
1. ~~Return round name in the result~~ -- fixed
2. ~~Slot mark as true, once assigned so that same slot doesnt come~~ -- fixed
3. ~~Fix time formatting in output (Current: 9:0, should be 09:00)~~ -- fixed
4. Alow >SSE level in interviewer (For now as a workaround all > SSE can be put as SSE if they need to conduct R1/R2)

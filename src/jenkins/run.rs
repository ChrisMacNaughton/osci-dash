use chrono::NaiveDateTime;

use schema::{runs, jobs};

use super::Job;


#[derive(Identifiable, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Job)]
#[table_name = "runs"]
pub struct Run {
    id: i32,
    job_id: i32,
    created_at: NaiveDateTime,
    pass: bool,
    runtime: i32,
}
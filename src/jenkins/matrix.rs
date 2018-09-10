use schema::matrices;
use super::Job;

#[derive(Identifiable, Insertable, Queryable, PartialEq, Debug)]
#[table_name = "matrices"]
pub struct Matrix {
    id: i32,
    name: String,
}


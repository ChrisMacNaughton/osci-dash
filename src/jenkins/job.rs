use schema::{matrices, jobs};
use super::{Matrix, Run};


#[derive(Identifiable, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Matrix)]
#[table_name = "jobs"]
pub struct Job {
    id: i32,
    ubuntu_version: String,
    openstack_version: String,
    matrix_id: i32,
}
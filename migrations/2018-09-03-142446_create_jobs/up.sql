-- Your SQL goes here
CREATE TABLE jobs  (
  id SERIAL PRIMARY KEY,
  matrix_id INTEGER NOT NULL,
  ubuntu_version TEXT NOT NULL,
  openstack_version TEXT NOT NULL,
)
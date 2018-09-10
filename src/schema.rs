table! {
    matrices {
        id -> Integer,
        name -> Text,
    }
}

table! {
    jobs {
        id -> Integer,
        ubuntu_version -> Text,
        openstack_version -> Text,
        matrix_id -> Integer,
    }
}

table! {
    runs {
        id -> Integer,
        job_id -> Integer,
        pass -> Bool,
        created_at -> Timestamp,
        runtime -> Integer,
    }
}
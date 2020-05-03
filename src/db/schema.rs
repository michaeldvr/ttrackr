table! {
    task (id) {
        id -> Integer,
        created -> Timestamp,
        taskname -> Text,
        notes -> Nullable<Text>,
        duration -> Integer,
        duedate -> Nullable<Timestamp>,
        done -> Bool,
    }
}

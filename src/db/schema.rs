table! {
    task (id) {
        id -> Integer,
        created -> Timestamp,
        taskname -> Text,
        notes -> Nullable<Text>,
        allocated -> Integer,
        duedate -> Nullable<Timestamp>,
        done -> Bool,
    }
}

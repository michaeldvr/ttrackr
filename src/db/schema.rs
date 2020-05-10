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

table! {
    worklog (id) {
        id -> Integer,
        task_id -> Integer,
        started -> Timestamp,
        stopped -> Nullable<Timestamp>,
        duration -> Integer,
        ignored -> Bool,
    }
}

joinable!(worklog -> task (task_id));

allow_tables_to_appear_in_same_query!(task, worklog,);

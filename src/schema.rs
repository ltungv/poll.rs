table! {
    options (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        done -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}

table! {
    votes (id) {
        id -> Integer,
        user_id -> Integer,
        option_id -> Integer,
        ord -> Integer,
    }
}

joinable!(votes -> options (option_id));
joinable!(votes -> users (user_id));

allow_tables_to_appear_in_same_query!(
    options,
    users,
    votes,
);

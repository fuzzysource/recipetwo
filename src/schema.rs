table! {
    encounters (id) {
        id -> Integer,
        word -> Text,
        timestamp -> BigInt,
        source -> Text,
    }
}

table! {
    words (id) {
        id -> Integer,
        word -> Text,
        meaning -> Text,
        url -> Text,
        example -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    encounters,
    words,
);

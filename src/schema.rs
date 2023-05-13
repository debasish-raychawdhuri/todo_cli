diesel::table! {
    todos (id) {
        id -> Text,
        description -> Text,
        completed -> Bool,
    }
}

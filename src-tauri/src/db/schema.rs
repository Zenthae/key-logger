// @generated automatically by Diesel CLI.

diesel::table! {
    key_event (id) {
        id -> Integer,
        event_time -> Integer,
        key_name -> Text,
    }
}

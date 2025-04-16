// @generated automatically by Diesel CLI.

#[cfg(feature = "server")]
pub mod guard {

    diesel::table! {
    dartleg (id) {
        id -> Integer,
        set_id -> Integer,
        leg_order -> Integer,
        start_score -> Integer,
        status -> Text,
    }
}

    diesel::table! {
    dartmatch (id) {
        id -> Integer,
        status -> Text,
        title -> Nullable<Text>,
    }
}

    diesel::table! {
    dartset (id) {
        id -> Integer,
        match_id -> Integer,
        set_order -> Integer,
        leg_amount -> Integer,
        status -> Text,
    }
}

    diesel::table! {
    score (id) {
        id -> Integer,
        dart_leg_id -> Integer,
        throw_order -> Integer,
        thrown -> Integer,
        remaining -> Integer,
        double_attempt -> Nullable<Integer>,
        deleted -> Bool,
    }
}

    diesel::table! {
    statusType (id) {
        id -> Integer,
        dart_type -> Text,
    }
}

    diesel::joinable!(dartleg -> dartset (set_id));
    diesel::joinable!(dartset -> dartmatch (match_id));
    diesel::joinable!(score -> dartleg (dart_leg_id));

    diesel::allow_tables_to_appear_in_same_query!(
    dartleg,
    dartmatch,
    dartset,
    score,
    statusType,
);
}

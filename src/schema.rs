// @generated automatically by Diesel CLI.

#[cfg(feature = "server")]
pub mod schema {
    diesel::table! {
        dartmatch (id) {
            id -> Nullable<Integer>,
            status -> Text,
        }
    }

    diesel::table! {
        dartset (id) {
            id -> Nullable<Integer>,
            match_id -> Nullable<Integer>,
            status -> Text,
        }
    }

    diesel::table! {
        leg (id) {
            id -> Nullable<Integer>,
            set_id -> Nullable<Integer>,
            status -> Text,
        }
    }

    diesel::table! {
        score (id) {
            id -> Nullable<Integer>,
            leg_id -> Nullable<Integer>,
            throw_order -> Nullable<Integer>,
            thrown -> Nullable<Integer>,
            remaining -> Nullable<Integer>,
            deleted -> Bool,
        }
    }

    diesel::table! {
        statusType (id) {
            id -> Nullable<Integer>,
            #[sql_name = "type"]
            type_ -> Nullable<Text>,
        }
    }

    diesel::joinable!(dartset -> dartmatch (match_id));
    diesel::joinable!(leg -> dartset (set_id));
    diesel::joinable!(score -> leg (leg_id));

    diesel::allow_tables_to_appear_in_same_query!(
        dartmatch,
        dartset,
        leg,
        score,
        statusType,
    );
}
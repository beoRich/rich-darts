// @generated automatically by Diesel CLI.

#[cfg(feature = "server")]
pub mod table {

    diesel::table! {
        dartmatch (id) {
            id -> Integer,
            status -> Text,
        }
    }

    diesel::table! {
        dartset (id) {
            id -> Integer,
            match_id -> Integer,
            status -> Text,
        }
    }

    diesel::table! {
        leg (id) {
            id -> Integer,
            set_id -> Integer,
            status -> Text,
        }
    }

    diesel::table! {
        score (id) {
            id -> Integer,
            leg_id -> Integer,
            throw_order -> Integer,
            thrown -> Integer,
            remaining -> Integer,
            deleted -> Bool,
        }
    }

    diesel::table! {
        statusType (id) {
            id -> Integer,
            #[sql_name = "type"]
            type_ -> Text,
        }
    }

    diesel::joinable!(dartset -> dartmatch (match_id));
    diesel::joinable!(leg -> dartset (set_id));
    diesel::joinable!(score -> leg (leg_id));

    diesel::allow_tables_to_appear_in_same_query!(dartmatch, dartset, leg, score, statusType,);
}

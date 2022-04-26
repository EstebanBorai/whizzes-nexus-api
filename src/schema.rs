table! {
    use diesel::sql_types::*;
    use crate::modules::user::entity::{Gender, Pronoun};

    posts (id) {
        id -> Uuid,
        user_id -> Uuid,
        content -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::modules::user::entity::{Gender, Pronoun};

    users (id) {
        id -> Uuid,
        name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Varchar,
        birthdate -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        gender -> Nullable<Gender>,
        pronoun -> Nullable<Pronoun>,
        gender_name -> Nullable<Varchar>,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);

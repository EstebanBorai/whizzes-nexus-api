table! {
    use diesel::sql_types::*;
    use crate::modules::user::repository::{PgGender, PgPronoun};

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
    use crate::modules::user::repository::{PgGender, PgPronoun};

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
        gender -> Nullable<PgGender>,
        pronoun -> Nullable<PgPronoun>,
        gender_name -> Nullable<Varchar>,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(posts, users,);

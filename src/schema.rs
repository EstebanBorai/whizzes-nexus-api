table! {
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
    }
}

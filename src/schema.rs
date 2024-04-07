// @generated automatically by Diesel CLI.

diesel::table! {
    datasiswa (id) {
        id -> Int4,
        name -> Varchar,
        kelas -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    nilaisiswa (id) {
        id -> Int4,
        datasiswa_id -> Int4,
        #[max_length = 64]
        nilai -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 64]
        matapelajaran -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::joinable!(nilaisiswa -> datasiswa (datasiswa_id));
diesel::joinable!(users_roles -> roles (role_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    datasiswa,
    nilaisiswa,
    roles,
    users,
    users_roles,
);

table! {
    companys (id) {
        id -> Int4,
        password_hash -> Text,
        phone -> Text,
        region_id -> Int4,
    }
}

table! {
    houms (id) {
        id -> Int4,
        company_id -> Int4,
        description -> Nullable<Text>,
    }
}

table! {
    pages (id) {
        id -> Int4,
        route_name -> Text,
        page_name -> Text,
        description -> Nullable<Text>,
        path -> Text,
    }
}

table! {
    regions (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    routes (id) {
        id -> Int4,
        name -> Text,
        route -> Text,
        description -> Nullable<Text>,
        publication -> Int2,
    }
}

table! {
    statics (id) {
        id -> Int4,
        page_id -> Int4,
        name -> Nullable<Text>,
        type_file -> Text,
        status -> Int2,
        mask -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        password_hash -> Text,
        phone -> Text,
        region_id -> Int4,
        home_id -> Int4,
    }
}

joinable!(companys -> regions (region_id));
joinable!(houms -> companys (company_id));
joinable!(statics -> pages (page_id));
joinable!(users -> houms (home_id));
joinable!(users -> regions (region_id));

allow_tables_to_appear_in_same_query!(
    companys,
    houms,
    pages,
    regions,
    routes,
    statics,
    users,
);

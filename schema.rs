table! {
    employees (id) {
        id -> Int4,
        name -> Varchar,
        role -> Varchar,
        company -> Varchar,
        address -> Varchar,
    }
}

/*
table! {
    employeess (id) {
        id -> Int4,
        item_name -> Varchar,
        role -> Varchar,
        company -> Varchar,
        address -> Varchar,
    }
}

table! {
    groceries (id) {
        id -> Int4,
        item_name -> Varchar,
        quantity -> Int4,
        price -> Int4,
        item_type -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    employees,
    employeess,
    groceries,
    posts,
);
*/
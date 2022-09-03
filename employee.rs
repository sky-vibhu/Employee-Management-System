use crate::schema::employees;

//Workers
#[derive(Queryable)]

pub struct Employees {
    pub id: i32,
    pub name: String,
    pub role: String,
    pub company: String,
    pub address: String
}

#[derive(Insertable)]
#[table_name= "employees"]

pub struct NewEmploy {
    pub name: String,
    pub role: String,
    pub company: String,
    pub address: String
}
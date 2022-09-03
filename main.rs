#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io;

pub mod schema;
pub mod employee;

use employee::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// Function to list all the employees in our table
fn show_employees() {
    let connection = establish_connection();
    let results = schema::employees::table.load::<Employees>(&connection).expect("Some Error occured");
    println!("Employees");
    println!("--------------------------------\n");

    if results.len() > 0 {
        for item in results {
            println!("Item ID: {}", item.id);
            println!("Name: {}", item.name);
            println!("Role: {}", item.role);
            println!("Company: {}", item.company);
            println!("Address: {}", item.address);
            println!("--------------\n");
        }
    }
    else {
        println!("There are no Employees in your database");
    }
    println!("-------------------------------------");

}


// Function to insert new Employee in database
fn insert_employ() {
    let mut name1 = String::new();
    let mut role = String::new();
    let mut company = String::new();
    let mut address = String::new();
    
    let connection = establish_connection();
    println!("Enter the name of the Employee: ");
    io::stdin().read_line(&mut name1).unwrap();
    let name1: String = name1.to_lowercase().trim().parse().unwrap();

    println!("Enter the role of the employee: ");
    io::stdin().read_line(&mut role).unwrap();
    let role: String = role.to_lowercase().trim().parse().unwrap();

    println!("Enter the company of the employee: ");
    io::stdin().read_line(&mut company).unwrap();
    let company: String = company.to_lowercase().trim().parse().unwrap();

    println!("Enter the address of the employee: ");
    io::stdin().read_line(&mut address).unwrap();
    let address: String = address.to_lowercase().trim().parse().unwrap();

    let new_employ = NewEmploy {
        name: name1,
        role: role,
        company: company,
        address: address
    };

    diesel::insert_into(schema::employees::table).values(&new_employ).get_result::<Employees>(&connection).expect("Could not insert new employ");
    println!("New employ added\n");
}


// Function to update item
fn update_employ() {
    let connection = establish_connection();

    let mut id = String::new();
    println!("Enter the id of employ that you want to update");
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap();

    let item = schema::employees::table.find(id);

    let mut field = String::new();
    println!("Enter the name of field that you would like to change");
    io::stdin().read_line(&mut field).unwrap();
    let field: String = field.trim().parse().unwrap();

    let mut new_value = String::new();
    println!("Enter the new value");
    io::stdin().read_line(&mut new_value).unwrap();
    let new_value: String = new_value.trim().parse().unwrap();

    match field.as_ref() {
        "name" => match diesel::update(item).set(schema::employees::name.eq(new_value)).get_result::<Employees>(&connection) {
            Ok(some) => some,
            Err(_) => {println!("Please enter the correct id"); return}
        },
        "address" => match diesel::update(item).set(schema::employees::address.eq(new_value)).get_result::<Employees>(&connection) {
            Ok(some) => some,
            Err(_) => {println!("Please enter the correct id"); return}
        },
        "role" => match diesel::update(item).set(schema::employees::role.eq(new_value)).get_result::<Employees>(&connection) {
            Ok(some) => some,
            Err(_) => {println!("Please enter the correct id"); return}
        },
        "company" => match diesel::update(item).set(schema::employees::company.eq(new_value)).get_result::<Employees>(&connection) {
            Ok(some) => some,
            Err(_) => {println!("Please enter the correct id"); return}
        },
        _ => {println!("Could not recognize the field"); return}
    };

    println!("Update succesful")
}


// function to delete an item
fn delete_employee() {
    let connection = establish_connection();

    let mut id = String::new();
    println!("Enter the id of employee that you want to delete");
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap();

    diesel::delete(schema::employees::table.find(id)).execute(&connection).expect("Could not delete the item");
    println!("One item deleted");
}


fn main() {
    println!("Welcome to your local Employees Management\n");

    loop {
        println!("\nDo you want to:\n'Show'\n'Add'\n'Change'\n'Delete'\n'Exit'");
        println!("Type Any keyword to start the process: ");

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command: String = command.to_lowercase().trim().parse().unwrap();

        match command.as_ref() {
            "show" => show_employees(),
            "add" => insert_employ(),
            "change" => update_employ(),
            "delete" => delete_employee(),
            "exit" => break,
            _ => println!("Not a correct key word")
        }
    }
    println!("Thank you for using the service\nBye Byee");
}

use class_util_server_db::{connect, models, schema};

use std::boxed;
use std::collections::HashMap;
use std::env::args;
use std::error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde_json;

const PATHWAYS: &[&[&str]] = &[
    &["AR01", "CLE Area 1"],
    &["AR02", "CLE Area 2"],
    &["AR03", "CLE Area 3"],
    &["AR04", "CLE Area 4"],
    &["AR05", "CLE Area 5"],
    &["AR06", "CLE Area 6"],
    &["AR07", "CLE Area 7"],
    &["G01A", "Pathway 1a"],
    &["G01F", "Pathway 1f"],
    &["G02", "Pathway 2"],
    &["G03", "Pathway 3"],
    &["G04", "Pathway 4"],
    &["G05A", "Pathway 5a"],
    &["G05F", "Pathway 5f"],
    &["G06A", "Pathway 6a"],
    &["G06D", "Pathway 6d"],
    &["G07", "Pathway 7"],
];

fn main() {
    let connection = connect();

    assert_empty(&connection);

    create_pathways(&connection);

    let arg = args()
        .nth(1)
        .expect("Provide the path to the pathways json as an argument");
    let path = Path::new(&arg);
    let associations = read_json(&path);
    associate_pathways(&connection, associations);
}

fn assert_empty(conn: &PgConnection) {
    let should_blow_up: bool =
        diesel::select(diesel::dsl::exists(schema::pathways::table.limit(1)))
            .get_result(conn)
            .unwrap();
    assert!(!should_blow_up, "There are pre-existing pathways");
}

fn create_pathways(conn: &PgConnection) {
    for entry in PATHWAYS {
        diesel::insert_into(schema::pathways::table)
            .values(&models::NewPathway { name: entry[1] })
            .execute(conn)
            .expect("Error saving new pathway");
    }
}

fn read_json(path: &Path) -> HashMap<String, Vec<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn associate_pathways(conn: &PgConnection, associations: HashMap<String, Vec<String>>) {
    for entry in PATHWAYS {
        let key = entry[0];
        let name = entry[1];
        let courses = associations.get(key).unwrap();
        let pathway_id = get_pathway(conn, &name).unwrap();

        for course_str in courses {
            match get_course(conn, course_str) {
                Ok(course_id) => {
                    diesel::insert_into(schema::pathway_courses::table)
                        .values(&models::NewPathwayCourse {
                            pathway_id,
                            course_id,
                        })
                        .execute(conn)
                        .expect("Error saving new pathway_course");
                }
                Err(msg) => println!("Problem reading course {}: {:#?}", course_str, msg),
            };
        }
    }
}

fn get_pathway(conn: &PgConnection, name: &str) -> Result<i32, diesel::result::Error> {
    schema::pathways::table
        .select(schema::pathways::id)
        .filter(schema::pathways::name.eq(name))
        .first(conn)
}

fn get_course(conn: &PgConnection, course_str: &str) -> Result<i32, Box<dyn error::Error>> {
    let parts = course_str.split('|').collect::<Vec<&str>>();
    let department_and_number = parts[0].split(' ').collect::<Vec<&str>>();
    let department = department_and_number[0];
    let number = department_and_number[1].parse::<i32>()?;
    let title = parts[1];
    let hours = parts[2].parse::<i32>()?;
    let slug = models::gen_course_slug(&department, number, title, hours);
    match schema::courses::table
        .select(schema::courses::id)
        .filter(schema::courses::slug.eq(slug))
        .first(conn)
    {
        Ok(id) => Ok(id),
        Err(err) => Err(boxed::Box::new(err)),
    }
}

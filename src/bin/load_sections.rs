use class_util_server_db::{connect, models, schema};

use std::env::args;
use std::fs::read_dir;
use std::path::Path;

use bigdecimal::BigDecimal;
use csv::{ReaderBuilder, Trim};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

const SEMESTERS: &[&str] = &["winter", "spring", "summer1", "summer2", "fall"];

#[derive(Serialize, Deserialize)]
struct Row {
    department: String,
    course_number_1: i32,
    course_title: String,
    faculty: String,
    course_ei: i32,
    credit_hours: i32,
    qca: BigDecimal,
    number: i32,
    #[serde(rename = "As")]
    a_percent: BigDecimal,
    #[serde(rename = "Bs")]
    b_percent: BigDecimal,
    #[serde(rename = "Cs")]
    c_percent: BigDecimal,
    #[serde(rename = "Ds")]
    d_percent: BigDecimal,
    #[serde(rename = "Fs")]
    f_percent: BigDecimal,
    #[serde(rename = "Textbox10")]
    withdrawals: i32,
}

fn main() {
    let connection = connect();

    let arg = args()
        .nth(1)
        .expect("Provide the path to the folder of CSVs as an argument");

    assert_empty(&connection);
    create_semesters(&connection);

    let path = Path::new(&arg);
    assert!(path.is_dir());
    for entry in read_dir(path).unwrap() {
        handle_csv(&connection, &entry.unwrap().path());
    }
}

fn assert_empty(conn: &PgConnection) {
    let should_blow_up: bool =
        diesel::select(diesel::dsl::exists(schema::semesters::table.limit(1)))
            .get_result(conn)
            .unwrap();
    assert!(!should_blow_up, "The DB isn't empty");
}

fn create_semesters(conn: &PgConnection) {
    for (i, name) in SEMESTERS.iter().enumerate() {
        diesel::insert_into(schema::semesters::table)
            .values(&models::NewSemester {
                name,
                ordering: (i + 1) as i32,
            })
            .execute(conn)
            .expect("Error saving new semester");
    }
}

fn handle_csv(conn: &PgConnection, path: &Path) {
    assert!(!path.is_dir() && path.extension().unwrap() == "csv");

    let term_id = create_term(conn, path);

    println!("reading file: {}", path.to_str().unwrap());

    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(path)
        .unwrap();
    for result in reader.deserialize() {
        let row: Row = match result {
            Ok(r) => r,
            Err(msg) => panic!("Problem reading line: {:#?}", msg),
        };

        let section = models::NewSection {
            crn: row.course_ei,
            gpa: &row.qca,
            a_percent: &row.a_percent,
            b_percent: &row.b_percent,
            c_percent: &row.c_percent,
            d_percent: &row.d_percent,
            f_percent: &row.f_percent,
            withdrawals: row.withdrawals,
            class_size: row.number,
            instructor_id: get_or_create_instructor(conn, &row.faculty),
            term_id,
            course_id: get_or_create_course(conn, &row),
        };

        diesel::insert_into(schema::sections::table)
            .values(&section)
            .execute(conn)
            .expect("Error saving new section");
    }
}

fn create_term(conn: &PgConnection, path: &Path) -> i32 {
    use schema::semesters::dsl::*;

    let parts = path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .split('_')
        .collect::<Vec<&str>>();

    let year = parts[0].parse::<i32>().unwrap();
    let semester_name = parts[1];

    let semester_id = semesters
        .select(id)
        .filter(name.eq(semester_name))
        .first(conn)
        .unwrap();

    let term: models::Term = diesel::insert_into(schema::terms::table)
        .values(&models::NewTerm { year, semester_id })
        .get_result(conn)
        .expect("Error saving new term");
    term.id
}

fn get_or_create_instructor(conn: &PgConnection, name: &str) -> i32 {
    let existing_instructor = schema::instructors::table
        .select(schema::instructors::id)
        .filter(schema::instructors::name.eq(name))
        .first(conn);
    match existing_instructor {
        Ok(existing_id) => existing_id,
        Err(_) => {
            let instructor: models::Instructor = diesel::insert_into(schema::instructors::table)
                .values(&models::NewInstructor { name })
                .get_result(conn)
                .expect("Error saving new instructor");
            instructor.id
        }
    }
}

fn get_or_create_course(conn: &PgConnection, row: &Row) -> i32 {
    let slug = &models::gen_course_slug(
        &row.department,
        row.course_number_1,
        &row.course_title,
        row.credit_hours,
    );

    let existing_course = schema::courses::table
        .select(schema::courses::id)
        .filter(schema::courses::slug.eq(slug))
        .first(conn);
    match existing_course {
        Ok(existing_id) => existing_id,
        Err(_) => {
            let course: models::Course = diesel::insert_into(schema::courses::table)
                .values(&models::NewCourse {
                    department: &row.department,
                    number: row.course_number_1,
                    title: &row.course_title,
                    hours: row.credit_hours,
                    slug,
                })
                .get_result(conn)
                .expect("Error saving new course");
            course.id
        }
    }
}

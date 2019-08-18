use super::schema::*;
use bigdecimal::BigDecimal;
use slug::slugify;

#[derive(Identifiable, Queryable)]
pub struct Course {
    pub id: i32,
    pub department: String,
    pub number: i32,
    pub title: String,
    pub hours: i32,
    pub slug: String,
}

#[derive(Insertable)]
#[table_name = "courses"]
pub struct NewCourse<'a> {
    pub department: &'a str,
    pub number: i32,
    pub title: &'a str,
    pub hours: i32,
    pub slug: &'a str,
}

pub fn gen_course_slug(department: &str, number: i32, title: &str, hours: i32) -> String {
    format!(
        "{}_{}_{}_{}",
        slugify(department),
        number,
        slugify(title),
        hours
    )
}

#[derive(Identifiable, Queryable)]
pub struct Instructor {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "instructors"]
pub struct NewInstructor<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable)]
pub struct Semester {
    pub id: i32,
    pub name: String,
    pub ordering: i32,
}

#[derive(Insertable)]
#[table_name = "semesters"]
pub struct NewSemester<'a> {
    pub name: &'a str,
    pub ordering: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Semester)]
pub struct Term {
    pub id: i32,
    pub year: i32,
    pub semester_id: i32,
}

#[derive(Insertable)]
#[table_name = "terms"]
pub struct NewTerm {
    pub year: i32,
    pub semester_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Instructor)]
#[belongs_to(Term)]
#[belongs_to(Course)]
pub struct Section {
    pub id: i32,
    pub crn: i32,
    pub gpa: BigDecimal,
    pub a_percent: BigDecimal,
    pub b_percent: BigDecimal,
    pub c_percent: BigDecimal,
    pub d_percent: BigDecimal,
    pub f_percent: BigDecimal,
    pub withdrawals: i32,
    pub class_size: i32,
    pub instructor_id: i32,
    pub term_id: i32,
    pub course_id: i32,
}

#[derive(Insertable)]
#[table_name = "sections"]
pub struct NewSection<'a> {
    pub crn: i32,
    pub gpa: &'a BigDecimal,
    pub a_percent: &'a BigDecimal,
    pub b_percent: &'a BigDecimal,
    pub c_percent: &'a BigDecimal,
    pub d_percent: &'a BigDecimal,
    pub f_percent: &'a BigDecimal,
    pub withdrawals: i32,
    pub class_size: i32,
    pub instructor_id: i32,
    pub term_id: i32,
    pub course_id: i32,
}

#[derive(Identifiable, Queryable)]
pub struct Pathway {
    pub id: i32,
    pub name: i32,
}

#[derive(Insertable)]
#[table_name = "pathways"]
pub struct NewPathway<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[primary_key(pathway_id, course_id)]
#[belongs_to(Pathway)]
#[belongs_to(Course)]
pub struct PathwayCourse {
    pub pathway_id: i32,
    pub course_id: i32,
}

#[derive(Insertable)]
#[table_name = "pathway_courses"]
pub struct NewPathwayCourse {
    pub pathway_id: i32,
    pub course_id: i32,
}

use crate::schema::*;

// use bigdecimal::BigDecimal;

use diesel::helper_types;
use diesel::prelude::*;
use diesel::dsl::avg; // should replace with sql_function! of round


pub type RichCourse = (
    courses::id,
    courses::department,
    courses::number,
    courses::title,
    courses::hours,
    courses::slug,
    // helper_types::avg<sections::columns::gpa>,
    // investigate sql_function
    // schema::section::a_percent,
    // schema::section::b_percent,
    // schema::section::c_percent,
    // schema::section::d_percent,
    // schema::section::f_percent,
    // schema::section::withdrawal_percent,
);

// type SectionsStats = (
//     helper_types::avg<sections::gpa>,
//     investigate sql_function
//     schema::section::a_percent,
//     schema::section::b_percent,
//     schema::section::c_percent,
//     schema::section::d_percent,
//     schema::section::f_percent,
//     schema::section::withdrawal_percent,
// );

// SELECT
//   courses.department,
//   courses.number,
//   courses.title,
//   courses.hours,
//   courses.slug,
//   courses.id,
//   ROUND(SUM(sections.gpa * sections.class_size) / SUM(sections.class_size), 2) AS gpa,
//   ROUND(SUM(sections.a_percent * sections.class_size) / SUM(sections.class_size), 1) AS a_percent,
//   ROUND(SUM(sections.b_percent * sections.class_size) / SUM(sections.class_size), 1) AS b_percent,
//   ROUND(SUM(sections.c_percent * sections.class_size) / SUM(sections.class_size), 1) AS c_percent,
//   ROUND(SUM(sections.d_percent * sections.class_size) / SUM(sections.class_size), 1) AS d_percent,
//   ROUND(SUM(sections.f_percent * sections.class_size) / SUM(sections.class_size), 1) AS f_percent,
//   ROUND(100.0 * SUM(sections.withdrawals) / SUM(sections.class_size), 1) AS withdrawal_percent
// FROM
//   courses JOIN sections
//     ON courses.id = sections.course_id
// GROUP BY
//   courses.id

pub fn all_courses() -> helper_types::Select<courses::table, RichCourse> {
    courses::table.select((
        courses::id,
        courses::department,
        courses::number,
        courses::title,
        courses::hours,
        courses::slug,
        // avg(sections::gpa) // put combine with aggregate_sections
    ))
}

pub fn aggregate_sections() -> helper_types::Select<sections::table, helper_types::avg<sections::gpa>> {
    sections::table.select(avg(sections::gpa))
}


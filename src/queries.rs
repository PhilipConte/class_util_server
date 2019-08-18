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
table! {
    courses (id) {
        id -> Int4,
        department -> Varchar,
        number -> Int4,
        title -> Varchar,
        hours -> Int4,
        slug -> Varchar,
    }
}

table! {
    instructors (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    pathway_courses (pathway_id, course_id) {
        pathway_id -> Int4,
        course_id -> Int4,
    }
}

table! {
    pathways (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    sections (id) {
        id -> Int4,
        crn -> Int4,
        gpa -> Numeric,
        a_percent -> Numeric,
        b_percent -> Numeric,
        c_percent -> Numeric,
        d_percent -> Numeric,
        f_percent -> Numeric,
        withdrawals -> Int4,
        class_size -> Int4,
        instructor_id -> Int4,
        term_id -> Int4,
        course_id -> Int4,
    }
}

table! {
    semesters (id) {
        id -> Int4,
        name -> Varchar,
        ordering -> Int4,
    }
}

table! {
    terms (id) {
        id -> Int4,
        year -> Int4,
        semester_id -> Int4,
    }
}

joinable!(pathway_courses -> courses (course_id));
joinable!(pathway_courses -> pathways (pathway_id));
joinable!(sections -> courses (course_id));
joinable!(sections -> instructors (instructor_id));
joinable!(sections -> terms (term_id));
joinable!(terms -> semesters (semester_id));

allow_tables_to_appear_in_same_query!(
    courses,
    instructors,
    pathway_courses,
    pathways,
    sections,
    semesters,
    terms,
);

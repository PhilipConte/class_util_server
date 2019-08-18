CREATE TABLE "sections" (
    "id" SERIAL PRIMARY KEY,
    "crn" integer NOT NULL CHECK ("crn" >= 0),
    "gpa" numeric (3, 2) NOT NULL,
    "a_percent" numeric (4, 1) NOT NULL,
    "b_percent" numeric (4, 1) NOT NULL,
    "c_percent" numeric (4, 1) NOT NULL,
    "d_percent" numeric (4, 1) NOT NULL,
    "f_percent" numeric (4, 1) NOT NULL,
    "withdrawals" integer NOT NULL CHECK ("withdrawals" >= 0),
    "class_size" integer NOT NULL CHECK ("class_size" >= 0),
    "instructor_id" integer NOT NULL REFERENCES "instructors" DEFERRABLE INITIALLY DEFERRED,
    "term_id" integer NOT NULL REFERENCES "terms" DEFERRABLE INITIALLY DEFERRED,
    "course_id" integer NOT NULL REFERENCES "courses" DEFERRABLE INITIALLY DEFERRED
);

CREATE INDEX "section_instructor_id" ON "sections" ("instructor_id");
CREATE INDEX "section_term_id" ON "sections" ("term_id");
CREATE INDEX "section_course_id" ON "sections" ("course_id");

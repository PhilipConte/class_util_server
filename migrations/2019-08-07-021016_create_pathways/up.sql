CREATE TABLE "pathways" (
    "id" SERIAL PRIMARY KEY,
    "name" varchar(20) UNIQUE NOT NULL
);

CREATE TABLE "pathway_courses" (
    "pathway_id" integer NOT NULL REFERENCES "pathways" DEFERRABLE INITIALLY DEFERRED,
    "course_id" integer NOT NULL REFERENCES "courses" DEFERRABLE INITIALLY DEFERRED,
    PRIMARY KEY ("pathway_id", "course_id")
);

CREATE INDEX "pathway_courses_pathway_id" ON "pathway_courses" ("pathway_id");
CREATE INDEX "pathway_courses_course_id" ON "pathway_courses" ("course_id");

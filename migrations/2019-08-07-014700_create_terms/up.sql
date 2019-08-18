CREATE TABLE "terms" (
    "id" SERIAL PRIMARY KEY,
    "year" integer NOT NULL CHECK ("year" >= 0),
    "semester_id" integer NOT NULL REFERENCES "semesters" DEFERRABLE INITIALLY DEFERRED
);

CREATE INDEX "term_semester_id" ON "terms" ("semester_id");

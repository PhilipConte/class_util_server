CREATE TABLE "courses" (
    "id" SERIAL PRIMARY KEY,
    "department" varchar(8) NOT NULL,
    "number" integer NOT NULL CHECK ("number" >= 0),
    "title" varchar(200) NOT NULL,
    "hours" integer NOT NULL CHECK ("hours" >= 0),
    "slug" varchar(200) NOT NULL UNIQUE,
    unique ("department", "number", "title", "hours")
);

CREATE INDEX "course_slug" ON "courses" ("slug");

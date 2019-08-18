CREATE TABLE "semesters" (
    "id" SERIAL PRIMARY KEY,
    "name" varchar(20) UNIQUE NOT NULL,
    "ordering" integer UNIQUE NOT NULL CHECK ("ordering" >= 0)
);

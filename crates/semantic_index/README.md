
WIP: Sample SQL Queries
/*

create table "files" (
"id" INTEGER PRIMARY KEY,
"path" VARCHAR,
"sha1" VARCHAR,
);

create table symbols (
"file_id" INTEGER REFERENCES("files", "id") ON CASCADE DELETE,
"offset" INTEGER,
"embedding" VECTOR,
);

insert into "files" ("path", "sha1") values ("src/main.rs", "sha1") return id;
insert into symbols (
"file_id",
"start",
"end",
"embedding"
) values (
(id,),
(id,),
(id,),
(id,),
)


*/

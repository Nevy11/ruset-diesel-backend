-- Your SQL goes here
drop TABLE studentInfo;

CREATE TABLE studentInfo(
    reg_no VARCHAR PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    GRADE VARCHAR,
    marks INT
)
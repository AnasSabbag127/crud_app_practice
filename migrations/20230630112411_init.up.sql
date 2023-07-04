-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS users(
        id Uuid PRIMARY KEY NOT NULL DEFAULT(uuid_generate_v4()),
        user_name VARCHAR(40)
    );

CREATE TABLE
    IF NOT EXISTS tasks(
        id Uuid PRIMARY KEY NOT NULL DEFAULT(uuid_generate_v4()),
        task_name VARCHAR(30),
        task_state VARCHAR(30) DEFAULT 'NotStarted',
        user_id Uuid,  
        CONSTRAINT fk_task
        FOREIGN KEY(user_id)
	    REFERENCES users(id)
    );

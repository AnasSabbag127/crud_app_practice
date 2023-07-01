-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS users(
        id UUID PRIMARY KEY NOT NULL DEFAULT(uuid_generate_v4()),
        user_name VARCHAR(40)
    );

CREATE TABLE
    IF NOT EXISTS tasks(
        id UUID PRIMARY KEY NOT NULL DEFAULT(uuid_generate_v4()),
        task_name VARCHAR(30),
        task_state VARCHAR(30) DEFAULT 'NOTSTARTED',
        user_id UUID,  
        CONSTRAINT fk_task
        FOREIGN KEY(user_id)
	    REFERENCES users(id)
    );

--    pub user_uuid:String,
--    pub task_uuid:String,
--    pub task_type:String,
--    pub state:TaskState,
--    pub source_file:String,
--    pub result_file:Option<String>
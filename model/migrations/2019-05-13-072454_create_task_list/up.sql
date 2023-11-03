CREATE TABLE task_list (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	project_id int NOT NULL
)

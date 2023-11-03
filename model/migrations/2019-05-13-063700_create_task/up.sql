CREATE TABLE task (
	id SERIAL PRIMARY KEY,
	project_id INT NOT NULL references project(id),
	task_list_id INT,
	title VARCHAR NOT NULL,
	completed BOOLEAN NOT NULL DEFAULT 'f'
)

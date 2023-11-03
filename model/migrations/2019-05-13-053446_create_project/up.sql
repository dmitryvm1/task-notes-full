CREATE TABLE project (
	id SERIAL PRIMARY KEY,
	owner_id INT NOT NULL references app_user(id),
	title VARCHAR NOT NULL
)

-- Your SQL goes here
ALTER TABLE task ADD CONSTRAINT project_c FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE;
ALTER TABLE project ADD CONSTRAINT owner_c FOREIGN KEY (owner_id) REFERENCES app_user(id) ON DELETE CASCADE;
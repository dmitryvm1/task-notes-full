-- This file should undo anything in `up.sql`
ALTER TABLE task DROP CONSTRAINT project_c;
ALTER TABLE project DROP CONSTRAINT owner_c;
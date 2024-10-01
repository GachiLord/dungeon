CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    login varchar(36) NOT NULL,
    name varchar(120) NOT NULL,
    password varchar(97) NOT NULL,
    class smallint NOT NULL,
    CHECK (class >= 0 AND class <=3),
    is_admin boolean NOT NULL,
    tags text[] DEFAULT '{}'
);

CREATE UNIQUE INDEX IF NOT EXISTS id_idx ON users(id);


CREATE TABLE IF NOT EXISTS tasks (
  id SERIAL PRIMARY KEY,
  complexity smallint NOT NULL,
  CHECK (complexity >= 0 AND complexity <=3),
  expected_time real NOT NULL,
  CHECK (expected_time > 0),
  tags text[] NOT NULL,
  description varchar(1000) NOT NULL, 
  assigned_to INT DEFAULT NULL,       
  CONSTRAINT fk_users
    FOREIGN KEY(assigned_to) 
	    REFERENCES users(id)
	    ON DELETE SET NULL
);


CREATE UNIQUE INDEX IF NOT EXISTS id_idx ON tasks(id);


CREATE TABLE IF NOT EXISTS completed_tasks (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  CONSTRAINT fk_users
    FOREIGN KEY(user_id) 
	    REFERENCES users(id)
	    ON DELETE SET NULL,
  task_id INT NOT NULL,
  CONSTRAINT fk_tasks
    FOREIGN KEY(task_id) 
	    REFERENCES tasks(id)
	    ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS task_id_idx ON completed_tasks(task_id);
CREATE INDEX IF NOT EXISTS user_id_idx ON completed_tasks(user_id);


CREATE TABLE IF NOT EXISTS invite_tokens (
	  id SERIAL PRIMARY KEY,
	  token VARCHAR (500) NOT NULL,
    is_expired bool NOT NULL
);

CREATE INDEX IF NOT EXISTS token_idx ON invite_tokens USING HASH (token);

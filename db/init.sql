CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    login varchar(36) NOT NULL,
    name varchar(120) NOT NULL,
    password varchar(97) NOT NULL,
    class smallint NOT NULL,
    CHECK (class >= 0 AND class <=3),
    is_admin boolean NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS id_idx ON users(id);


CREATE TABLE IF NOT EXISTS tasks (
  id SERIAL PRIMARY KEY,
  complexity smallint NOT NULL,
  CHECK (complexity >= 0 AND complexity <=3),
  expected_time real NOT NULL,
  CHECK (expected_time > 0),
  tags text[] NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP
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
CREATE UNIQUE INDEX IF NOT EXISTS user_id_idx ON completed_tasks(user_id);


CREATE TABLE IF NOT EXISTS expired_jwts (
	  id SERIAL PRIMARY KEY,
	  jwt_data VARCHAR (500) NOT NULL,
    expire_date DATE NOT NULL DEFAULT CURRENT_DATE
);

CREATE INDEX IF NOT EXISTS jwt_data_idx ON expired_jwts USING HASH (jwt_data);


CREATE TABLE IF NOT EXISTS expired_invite_tokens (
	  id SERIAL PRIMARY KEY,
	  token VARCHAR (500) NOT NULL,
    expire_date DATE NOT NULL DEFAULT CURRENT_DATE
);

CREATE INDEX IF NOT EXISTS jwt_data_idx ON expired_jwts USING HASH (jwt_data);


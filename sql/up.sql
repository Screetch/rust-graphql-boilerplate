-- Your SQL goes here
CREATE TABLE teams (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);
    
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  team_id INT NOT NULL,
  FOREIGN KEY (team_id) REFERENCES teams(id)
);
    
INSERT INTO teams(id, name) VALUES (1, 'Bayside');
INSERT INTO users(name, team_id) VALUES ('Zack', 1);
INSERT INTO users(name, team_id) VALUES ('Screetch', 1);
INSERT INTO users(name, team_id) VALUES ('Slater', 1);
INSERT INTO users(name, team_id) VALUES ('Kelly', 1);
INSERT INTO users(name, team_id) VALUES ('Jessie', 1);
INSERT INTO users(name, team_id) VALUES ('Lisa', 1);

INSERT INTO teams(id, name) VALUES (2, 'Others');
INSERT INTO users(name, team_id) VALUES ('Mr. Belding', 2);
INSERT INTO users(name, team_id) VALUES ('Max', 2);
INSERT INTO users(name, team_id) VALUES ('Tori', 2);
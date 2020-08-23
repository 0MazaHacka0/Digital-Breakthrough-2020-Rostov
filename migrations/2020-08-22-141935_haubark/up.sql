-- Your SQL goes here

CREATE TABLE regions (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE companys (
    id SERIAL PRIMARY KEY,
    password_hash TEXT NOT NULL,
    phone TEXT NOT NULL,
    region_id INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE SET DEFAULT
);

CREATE TABLE houms (
    id SERIAL PRIMARY KEY,
    company_id INTEGER NOT NULL DEFAULT 0,
    description TEXT,
    FOREIGN KEY (company_id) REFERENCES companys(id) ON DELETE SET DEFAULT
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    password_hash TEXT NOT NULL,
    phone TEXT NOT NULL,
    region_id INTEGER NOT NULL DEFAULT 0,
    home_id INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE SET DEFAULT,
    FOREIGN KEY (home_id) REFERENCES houms(id) ON DELETE SET DEFAULT
);

INSERT INTO routes(publication, name, route, description) VALUES
    (1, 'api', '/api/v1/', 'active routes for apis (recomended)');

INSERT INTO regions(id, name) VALUES
    (1, 'Ростовская область');

INSERT INTO companys(id, password_hash, phone, region_id) VALUES
    (1, '1312312321', '89785645563', 1);

INSERT INTO houms(id, company_id) VALUES
    (1, 1);
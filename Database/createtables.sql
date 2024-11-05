CREATE TABLE ASSET(
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                sysname TEXT NOT NULL,
                model TEXT NOT NULL,
                type TEXT NOT NULL,
                manufacturer TEXT NOT NULL,
                ip TEXT NOT NULL,
                purchasedate DATE NOT NULL,
                note TEXT NOT NULL,
                Employee INTEGER NOT NULL
)
CREATE TABLE Department(
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        employee TEXT NOT NULL
)
CREATE TABLE Employee(
        id INTEGER PRIMARY KEY,
        firstname TEXT NOT NULL,
        secondname TEXT NOT NULL,
        email TEXT NOT NULL,
        department TEXT NOT NULL,
        assets TEXT NOT NULL
)

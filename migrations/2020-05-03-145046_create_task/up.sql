CREATE TABLE IF NOT EXISTS task (
    id INTEGER NOT NULL,
    created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    taskname TEXT NOT NULL UNIQUE,
    notes TEXT,
    duration INTEGER NOT NULL DEFAULT 0,
    duedate DATETIME,
    done BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY(id DESC)
);
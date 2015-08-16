DROP DATABASE IF EXISTS bb;
GRANT ALL ON bb.* TO bb;

CREATE DATABASE bb;

USE bb;

CREATE TABLE boards (
    board_name CHAR(8),
    board_desc VARCHAR(256) NOT NULL,

    PRIMARY KEY (board_name)
);

CREATE TABLE threads (
    thread_id INT AUTO_INCREMENT,
    thread_title VARCHAR(64) NOT NULL,
    thread_board_name CHAR(8),

    PRIMARY KEY (thread_id),
    FOREIGN KEY (thread_board_name)
        REFERENCES boards(board_name)
        ON DELETE CASCADE
);

CREATE TABLE posts (
    post_id INT AUTO_INCREMENT,
    post_text TEXT NOT NULL,
    post_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    post_thread_id INT NOT NULL,

    PRIMARY KEY (post_id),
    FOREIGN KEY (post_thread_id)
        REFERENCES threads(thread_id)
        ON DELETE CASCADE
);

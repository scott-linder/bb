USE bb;

INSERT INTO boards (board_name, board_desc) VALUES ('gen', 'general');
INSERT INTO boards (board_name, board_desc) VALUES ('comp', 'computing');

INSERT INTO threads (thread_id, thread_title, thread_board_name) VALUES (1, 'welcome', 'gen');
INSERT INTO threads (thread_id, thread_title, thread_board_name) VALUES (2, 'rules', 'gen');

INSERT INTO posts (post_id, post_text, post_thread_id) VALUES (1, 'welcome to the bulletin boards', 1);
INSERT INTO posts (post_id, post_text, post_thread_id) VALUES (2, 'enjoy your stay', 1);

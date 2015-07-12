USE bb;

INSERT INTO boards (board_id, board_name, board_desc) VALUES (1, 'gen', 'general');
INSERT INTO boards (board_id, board_name, board_desc) VALUES (2, 'comp', 'computing');

INSERT INTO threads (thread_id, thread_title, thread_board_id) VALUES (1, 'welcome', 1);
INSERT INTO threads (thread_id, thread_title, thread_board_id) VALUES (2, 'rules', 1);

INSERT INTO posts (post_id, post_text, post_thread_id) VALUES (1, 'welcome to the bulletin boards', 1);
INSERT INTO posts (post_id, post_text, post_thread_id) VALUES (2, 'enjoy your stay', 1);

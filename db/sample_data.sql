-- Insert users
INSERT INTO users (username, passwd, nationality, race, sexual_orientation) VALUES
('user1', 'password1', 'USA', 'Caucasian', 'heterosexual'),
('user2', 'password2', 'Canada', 'Asian', 'homosexual'),
('user3', 'password3', 'UK', 'Hispanic', 'bisexual'),
('user4', 'password4', 'Australia', 'African', 'homosexual'),
('user5', 'password5', 'India', 'Mixed', 'heterosexual');


-- Insert pictures for each user (3 pics per user)
insert into pictures (owner_id, path_to_pic) values
(1, '/images/user1_pic1.jpg'),
(1, '/images/user1_pic2.jpg'),
(1, '/images/user1_pic3.jpg'),
(2, '/images/user2_pic1.jpg'),
(2, '/images/user2_pic2.jpg'),
(2, '/images/user2_pic3.jpg'),
(3, '/images/user3_pic1.jpg'),
(3, '/images/user3_pic2.jpg'),
(3, '/images/user3_pic3.jpg'),
(4, '/images/user4_pic1.jpg'),
(4, '/images/user4_pic2.jpg'),
(4, '/images/user4_pic3.jpg'),
(5, '/images/user5_pic1.jpg'),
(5, '/images/user5_pic2.jpg'),
(5, '/images/user5_pic3.jpg');

-- Insert ratings (20 ratings, random between users)
insert into ratings (rater_id, rated_id, mark) values
(1, 2, 5),
(1, 3, 4),
(1, 4, 3),
(1, 5, 2),
(2, 1, 4),
(2, 3, 3),
(2, 4, 5),
(2, 5, 4),
(3, 1, 3),
(3, 2, 4),
(3, 4, 2),
(3, 5, 5),
(4, 1, 5),
(4, 2, 3),
(4, 3, 4),
(4, 5, 2),
(5, 1, 2),
(5, 2, 4),
(5, 3, 5),
(5, 4, 3);

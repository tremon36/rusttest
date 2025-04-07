
-- Select all ratings of a user, including the data of the ratings

select *
from ratings r
join users u1 on r.rater_id = u1.id
where r.rated_id = 5;

-- Select all ratings performed by a user, including data of the rated profiles

select *
from ratings r
join users u2 on r.rated_id = u2.id
where r.rater_id = 5;

-- Get amout of ratings performed by user

select count(*) as totalRatedProfilesByUser
from ratings r
join users u2 on r.rated_id = u2.id
where r.rater_id = 5;

-- Get amount of people that rated the user

select count(*) as totalProfilesThatRatedUser
from ratings r
join users u1 on r.rater_id = u1.id
where r.rated_id = 5;

-- Get all data including pfps of a profile

select * 
from users u1
join pictures p1 on p1.owner_id = u1.id
where u1.id = 5;
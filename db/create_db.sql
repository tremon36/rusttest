create table users (
    id bigint unsigned primary key auto_increment,
    username varchar(50) unique not null,
    sexual_orientation varchar(20) not null,
    passwd varchar(50) not null,
    created_at timestamp default now(),
    nationality varchar(50),
    race varchar(10)
);


create table pictures (
    id bigint unsigned primary key auto_increment,
    owner_id bigint unsigned not null,
    path_to_pic varchar(100) not null,
    foreign key(owner_id) references users(id) on delete cascade
);

create table ratings (
    id bigint unsigned primary key auto_increment,
    rater_id bigint unsigned,
    rated_id bigint unsigned,
    mark integer,
    foreign key (rater_id) references users(id) on delete set null,
    foreign key (rated_id) references users(id) on delete set null
);


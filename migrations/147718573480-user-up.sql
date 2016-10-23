-- direction: up
-- backref: 147718563260
-- ref: 147718573480


create table `recog_api`.`user` (
  id int(11) unsigned auto_increment not null primary key,
  name varchar(100) not null default 'stranger',
  email varchar(150) not null,
  password varchar(100) not null
) engine=InnoDB default charset=utf8mb4 collate=utf8mb4_unicode_ci;

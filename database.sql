drop table if exists todo_item;
drop table if exists todo_list;

create table todo_list (
	id serial primary key,
	title varchar(256) not null
);

create table todo_item (
	id serial primary key,
	title varchar(256) not null,
	checked boolean not null default false,
	list_id integer not null,
	foreign key (list_id) references todo_list(id)
);
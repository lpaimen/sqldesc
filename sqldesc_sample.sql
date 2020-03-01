
--* Table with no properties
create table foobar();

--* Sophisticated table to store barbars
create table barbar(
    --* eg Eric
    name varchar(50) not null,
    --* nickname - if not set, use name
    nick varchar(50),
    --* pretty vague, separated with commas
    location varchar(120)
);

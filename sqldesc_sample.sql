
--* Table with no properties
create table foobar();

--* Sophisticated table to store barbars
--* Note: This is extra well commented
create table barbar(
    --* eg Eric
    name varchar(50) not null,
    --* nickname - if not set, use name
    nick varchar(50),
    --* pretty vague, separated with commas
    location varchar(120)
);

select * from foo;

create table no_comment(
    /**
     * long
     * doc
     * line
     */
    id integer primary key
);

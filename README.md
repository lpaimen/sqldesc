# SQLDESC: SQL description comment parser

Implement SQL description comments.

So if you have [sample.dsql](sample.sql) containing:
```
/**
 * List of cities in Finland
 */
CREATE TABLE fi_cities(
    /** Name as written in finnish */
    name VARCHAR(100) NOT NULL,
    /**
     * Alternate/nick names
     * Separated with comma, for example: Nääsville,Tammerfors
     */
    alternate_names TEXT,
    --* Latitude
    lat DOUBLE NULL,
    --* Longitude
    lng DOUBLE NULL,
);
```

You can generate [PostgreSQL comments](https://www.postgresql.org/docs/current/sql-comment.html) with:
```
$ cargo run sample.sql
COMMENT ON TABLE fi_cities IS 'List of cities in Finland';
COMMENT ON COLUMN fi_cities.name IS 'Name as written in finnish';
COMMENT ON COLUMN fi_cities.alternate_names IS 'Alternate/nick names
Separated with comma, for example: Nääsville,Tammerfors';
COMMENT ON COLUMN fi_cities.lat IS 'Latitude';
COMMENT ON COLUMN fi_cities.lng IS 'Longitude';
```

Or store them to table for later processing with other tools:
```
$ cargo run sample.sql --output sql
INSERT INTO sqldesc(type, target, description) VALUES('table', 'fi_cities', 'List of cities in Finland');
INSERT INTO sqldesc(type, target, description) VALUES('column', 'fi_cities.name', 'Name as written in finnish');
INSERT INTO sqldesc(type, target, description) VALUES('column', 'fi_cities.alternate_names', 'Alternate/nick names
Separated with comma, for example: Nääsville,Tammerfors');
INSERT INTO sqldesc(type, target, description) VALUES('column', 'fi_cities.lat', 'Latitude');
INSERT INTO sqldesc(type, target, description) VALUES('column', 'fi_cities.lng', 'Longitude');

```

## Getting started

Install Rust and cargo. Then:

```
cargo run sample.sql
```

## Big thanks

Most of the work is actually done by great [sqlparser-rs](https://github.com/andygrove/sqlparser-rs) library!

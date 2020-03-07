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
-- Add migration script here
ALTER TABLE favorite_movies 
ADD imdb_id TEXT NULL,
ADD plot TEXT NULL,
ADD poster TEXT NULL;
-- Add migration script here
CREATE TABLE favorite_movies(
	id SERIAL PRIMARY KEY,
	user_id SERIAL,
	movie_name TEXT NOT NULL
)
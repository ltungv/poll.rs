CREATE TABLE ballots (
	id SERIAL NOT NULL PRIMARY KEY,
	uuid UUID NOT NULL UNIQUE
);
CREATE UNIQUE INDEX unique_ballot ON ballots(uuid);

CREATE TABLE items (
	id SERIAL NOT NULL PRIMARY KEY,
	title TEXT NOT NULL,
	content TEXT NOT NULL,
	done BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE rankings (
	id SERIAL NOT NULL PRIMARY KEY,
	ballot_id INTEGER NOT NULL,
	item_id INTEGER NOT NULL,
	ord INTEGER NOT NULL,

	FOREIGN KEY (ballot_id) REFERENCES ballots(id),
	FOREIGN KEY (item_id) REFERENCES items(id)
);
CREATE INDEX ordering_by_ballot_item ON rankings(ballot_id ASC, item_id ASC);
CREATE UNIQUE INDEX unique_ranking ON rankings(ballot_id, item_id);

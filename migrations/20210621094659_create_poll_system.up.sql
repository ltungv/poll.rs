CREATE TABLE ballots (
	id INTEGER NOT NULL PRIMARY KEY,
	uuid TEXT NOT NULL UNIQUE
);
CREATE UNIQUE INDEX uniq_ballot ON ballots(uuid);

CREATE TABLE items (
	id INTEGER NOT NULL PRIMARY KEY,
	title TEXT NOT NULL,
	content TEXT NOT NULL,
	done BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE rankings (
	id INTEGER NOT NULL PRIMARY KEY,
	ballot_id INTEGER NOT NULL,
	item_id INTEGER NOT NULL,
	ord INTEGER NOT NULL,

	FOREIGN KEY (ballot_id) REFERENCES ballots(id)
	FOREIGN KEY (item_id) REFERENCES items(id)
);
CREATE INDEX ballot_item_one_to_one ON rankings(ballot_id ASC, item_id ASC);
CREATE UNIQUE INDEX uniq_rankings ON rankings(ballot_id, item_id);

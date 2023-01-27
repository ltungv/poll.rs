CREATE TABLE ballots (
	id INTEGER NOT NULL AUTO_INCREMENT PRIMARY KEY,
	uuid BINARY(16) NOT NULL UNIQUE
);
CREATE UNIQUE INDEX unique_ballot ON ballots(uuid);

CREATE TABLE items (
	id INTEGER NOT NULL AUTO_INCREMENT PRIMARY KEY,
	title TEXT NOT NULL,
	content TEXT NOT NULL,
	done BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE rankings (
	id INTEGER NOT NULL AUTO_INCREMENT PRIMARY KEY,
	ballot_id INTEGER NOT NULL,
	item_id INTEGER NOT NULL,
	ord INTEGER NOT NULL,

	FOREIGN KEY (ballot_id) REFERENCES ballots(id),
	FOREIGN KEY (item_id) REFERENCES items(id)
);
CREATE INDEX ordering_by_ballot_item ON rankings(ballot_id ASC, item_id ASC);
CREATE UNIQUE INDEX unique_ranking ON rankings(ballot_id, item_id);

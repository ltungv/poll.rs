# Poll.rs

A simple web-application for ranking options based on users' votes. The app 
can take a list of options and chooses the most preferred option using the
[instant-runoff voting] system, a type of [ranked preferential] voting counting
method.

[instant-runoff voting]: https://en.wikipedia.org/wiki/Instant-runoff_voting
[ranked preferential]: https://en.wikipedia.org/wiki/Ranked_voting

# Features

## Test drive

- No need for serious security/authentication.
- Users are allowed to change their ranking.
	- Use some kind of unique ID and storage system to identify the user the 
	next time they access the poll
		- Session?
		- Browser storage?

### Some initial state

```
insert into items(title, content) values
	("Title 1", "This is not very interesting"),
	("Title 2", "This is somewhat inteteresting"),
	("Title 3", "This is very interesting");
insert into ballots(uuid) values
	("testing-value-1"),
	("testing-value-2"),
	("testing-value-3"),
	("testing-value-4"),
	("testing-value-5");
insert into rankings(ballot_id,item_id,ord) values
	(1,1,0),
	(1,2,1),
	(1,3,2);
insert into rankings(ballot_id,item_id,ord) values
	(2,1,1),
	(2,2,0),
	(2,3,2);
insert into rankings(ballot_id,item_id,ord) values
	(3,1,1),
	(3,2,2),
	(3,3,0);
insert into rankings(ballot_id,item_id,ord) values
	(4,3,0),
	(4,1,2),
	(4,2,1);
insert into rankings(ballot_id,item_id,ord) values
	(5,1,1),
	(5,2,2),
	(5,3,0);
```


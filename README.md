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

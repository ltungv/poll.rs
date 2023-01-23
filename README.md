# Poll.rs

A simple web-application for electing the best option using the [instant-runoff] voting system.

![register and vote example]

[instant-runoff]: https://en.wikipedia.org/wiki/Instant-runoff_voting
[register and vote example]: docs/register_and_vote.gif

# Features

No need for serious security and users are allowed to change their rankings.
+ Each user is assigned an UUID after they register.
+ The UUID can later be used to access existing rankings.
+ Users' account has no password.
+ Users' session is kept in using cookie.

# Tech stack

+ Server: [actix-web]
+ Database: [sqlx]
+ Template: [sailfish]
+ Observability: [tracing], [opentelemetry]

[actix-web]: https://github.com/actix/actix-web
[sqlx]: https://github.com/launchbadge/sqlx
[sailfish]: https://github.com/launchbadge/sqlx
[tracing]: https://github.com/tokio-rs/tracing
[opentelemetry]: https://github.com/open-telemetry/opentelemetry-rust

# Some initial state

```
insert into items(title, content) values
	('Ada Lovelace', 'Augusta Ada King, Countess of Lovelace was an English mathematician and writer.'),
	('Alan Turing', 'Alan Mathison Turing OBE FRS was an English mathematician, computer scientist, logician, cryptanalyst, philosopher, and theoretical biologist.');

insert into ballots(uuid) values
	(gen_random_uuid()),
	(gen_random_uuid());

insert into rankings(ballot_id,item_id,ord) values
	(1,1,0),
	(1,2,1),
	(2,1,1),
	(2,2,0);
```

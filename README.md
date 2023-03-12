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
+ Users' session is kept in cookie.

# Tech stack

+ Server: [actix-web]
+ Database: [sqlx], [mysql]
+ Template: [sailfish]
+ Observability: [tracing], [opentelemetry]

[actix-web]: https://github.com/actix/actix-web
[mysql]: https://www.mysql.com
[sqlx]: https://github.com/launchbadge/sqlx
[sailfish]: https://github.com/launchbadge/sqlx
[tracing]: https://github.com/tokio-rs/tracing
[opentelemetry]: https://github.com/open-telemetry/opentelemetry-rust

# Some initial state

```
insert into items(title, content) values
	('Ada Lovelace', 'Augusta Ada King, Countess of Lovelace was an English mathematician and writer.'),
	('Alan Turing', 'Alan Mathison Turing OBE FRS was an English mathematician, computer scientist, logician, cryptanalyst, philosopher, and theoretical biologist.');
```

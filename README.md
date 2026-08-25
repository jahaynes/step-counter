To build & run:

    Start the database (podman_pg.sh script included)

    Start the app: cargo run

    Visit the swagger page at http://127.0.0.1:3000/swagger-ui/

    There is a rudimentary system-tests.sh script for testing the whole app.

Notes:

* I had started building an in-memory version of the storage layer, so that the web and/or service layer could be testing independently,however there really wasn't enough logic in any web/service layers to justify it (it's mostly SQL statements), so I switched back to the postgres-only version.  However, the web routes are all set up to handle any StepCounterService, not just the PostgresStepCounterService, so it wouldn't be too hard to switch back.

* For the same reason as above (i.e. the whole task was to persist step counts), I wrote system tests that hit everything from the outside (system-tests.sh), rather than how I usually work (which is to focus more on unit tests).

Discussion:

* It accepts multiple step count submissions for the same user on the same day, and keeps them both.  However it's timestamp-based, if the time-stamps match exactly, it's assumed the user is intending to correct the old figure, so it overwrites it.

* Negative step counts are rejected.

* Queries on missing or excluded data are treated as 0.

* There's no auth or concept of a known or unknown user.  Step counts can be submitted for any userid, 

* Since I don't have time to worry about timezone conversions, the system only accepts ISO 8601 timestamps, e.g.: 2026-08-25T07:11:46.241Z.  So the correct information is stored, and timezone conversion could be added later somewhere in the view.

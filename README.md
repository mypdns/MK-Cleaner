[![My Privacy DNS](https://www.mypdns.org/images/logo.png)](https://www.mypdns.org/)

## Sponsor

[![ko-fi](https://www.mypdns.org/fileproxy/?name=sp_kofi_mypdns)]([DONATION.md](https://kb.mypdns.org/articles/MTX-A-3/DONATION))
[![liberapay](https://www.mypdns.org/fileproxy/?name=sp_receives_mypdns)](https://liberapay.com/MyPDNS/donate)
[![goal](https://www.mypdns.org/fileproxy/?name=sp_goal_mypdns)](https://liberapay.com/MyPDNS/donate)

# MK Cleaner

Mk Cleaner is a little program that can operate with the API of a [Misskey][MK]
server. It should be able to do 3 things:

1. Suspend an Account based on internal(local) ID
2. Delete all the users posts
3. Delete all the users files
4. (If you are really bored) Cron the job to find these accounts by tags and do
   it by itself (Needs Some kind of manual approval system)

This is because I'm running and trying to maintain https://matrix.rocks/, and
this is part is sucking the life out of me, doing this manually.

The first commit to this project comes from : and is based on the Rust gradle
module [misskey-rs][MKRS], the gradle on [docs.rs][DRS], however, the project
kind a looks like it is no longer maintained, as last commit (as time of writing
this), was May 7, 2023

This may be the reason to the error I receive when running the code against the
server version `2024.10.0` while the last mention version on the project is
`v12.63.0`

## The error
The error seems to be related to how the json response is handled, and I don't
know if there is a workaround for this.

```rust
MISSKEY_API_URL="https://matrix.rocks/api/" MISSKEY_TOKEN="API_KEY" ./mk-cleaner $UID
Doing $UID
Failed to get user $UID notes: JSON error: invalid type: map, expected a sequence at line 1 column 825
users suspended: 1, not suspended: 0
notes deleted: 0, not deleted: 1
```

## Other docs

* Contributing: <./Contributing.md>
* Code of Conduct: <./CODE_OF_CONDUCT.md>
* License: <a href="../LICENSE" alt="BSD 3-Clause License" title="BSD 3-Clause License">BSD 3-Clause License</a>

[DRS]: https://docs.rs/misskey/latest/misskey/
[MK]: https://github.com/misskey-dev/misskey
[MKRS]: https://github.com/coord-e/misskey-rs

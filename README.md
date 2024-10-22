[![My Privacy DNS](https://www.mypdns.org/images/logo.png)](https://www.mypdns.org/)

[![ko-fi](https://www.mypdns.org/fileproxy/?name=sp_kofi_mypdns)]([DONATION.md](https://kb.mypdns.org/articles/MTX-A-3/DONATION))
[![liberapay](https://www.mypdns.org/fileproxy/?name=sp_receives_mypdns)](https://liberapay.com/MyPDNS/donate)
[![goal](https://www.mypdns.org/fileproxy/?name=sp_goal_mypdns)](https://liberapay.com/MyPDNS/donate)

# MK Cleaner

<!-- TOC -->
* [MK Cleaner](#mk-cleaner)
  * [Running MK Cleaner](#running-mk-cleaner)
  * [The error](#the-error)
  * [Other docs](#other-docs)
  * [History](#history)
<!-- TOC -->

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

## Running MK Cleaner
First, you should clone this repository to make it easy to update.

```shell
https://github.com/mypdns/MK-Cleaner.git mk-ckleaner
```

> [!NOTE]  
> This is subject to being altered once the program is fully
> functional, and released as stable.
> We, should change the way we read in the environment variables from a config
> file, and the rust program itself asks for the userID. 

> [!IMPORTANT]
> Next you need to configure the `mk-cleaner.sh` by copying the
> `mk-cleaner.sh.example` to `mk-cleaner.sh` and set the values of

1. MISSKEY_API_URL, This is the full url to your API path, such as `https://example.org/api/`
2. MISSKEY_TOKEN, This is your private API_key from your installation URI, such as `https://example.org/settings/api`

Now you should be able to run `mk-cleaner.sh` like 

```shell
mk-cleaner.sh userID
```

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

I've opened this issue https://github.com/coord-e/misskey-rs/issues/73

## Other docs

* Contributing: <a href="../master/Contributing.md">Contributing.md</a>
* Code of Conduct: <a href="../master/CODE_OF_CONDUCT.md">CODE_OF_CONDUCT.md</a>
* License: <a href="../master/LICENSE" title="BSD 3-Clause License">BSD 3-Clause License</a>

## History
This project started the 19th October 2024, 14:51 (CEST) with [this message] on
https://matrix.to/#/#rust-off-topic:xiretza.xyz

```json
{
  "content": {
    "body": "Hey, anyone of you who know how to code, that having a silent and even maybe a bored day?\n\nThen I would like to ask if one of you might like to give me a helping hand, by making a little program that can operate with the API of a Misskey server. It should be able to do 3 things\n\n1. Suspend an Account based on internal(local) ID\n2. Delete all the users posts \n3. Delete all the users files\n4. (If you are really bored) Cron the job to find these accounts by tags and do it by it self (Needs Some kind of manual approval system)\n\nThis is because I'm running and trying to maintain https://matrix.rocks/, and this is part is sucking the live out of me, doing this manually.",
    "format": "org.matrix.custom.html",
    "formatted_body": "<p>Hey, anyone of you who know how to code, that having a silent and even maybe a bored day?</p>\n<p>Then I would like to ask if one of you might like to give me a helping hand, by making a little program that can operate with the API of a Misskey server. It should be able to do 3 things</p>\n<ol>\n<li>Suspend an Account based on internal(local) ID</li>\n<li>Delete all the users posts</li>\n<li>Delete all the users files</li>\n<li>(If you are really bored) Cron the job to find these accounts by tags and do it by it self (Needs Some kind of manual approval system)</li>\n</ol>\n<p>This is because I'm running and trying to maintain https://matrix.rocks/, and this is part is sucking the live out of me, doing this manually.</p>\n",
    "m.mentions": {},
    "msgtype": "m.text"
  },
  "origin_server_ts": 1729342275336,
  "sender": "@spirillen:matrix.org",
  "type": "m.room.message",
  "unsigned": {
    "membership": "join",
    "age": 79859907,
    "transaction_id": "m1729342274901.2"
  },
  "event_id": "$_codlYa9cAgjTh0W3xKuZ9oWbWDHRl-Z1IAJkWTPkH0",
  "room_id": "!yqjgpOiwXStfzObcpo:typ3.tech"
}
```

Some hours later [Kim Minh] replied with this code, you find in the [first commit]

[DRS]: https://docs.rs/misskey/latest/misskey/
[MK]: https://github.com/misskey-dev/misskey
[MKRS]: https://github.com/coord-e/misskey-rs
[RPG]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3c63ed0325a458fbe12b654e3799ba87
[this message]: https://matrix.to/#/!yqjgpOiwXStfzObcpo:typ3.tech/$_codlYa9cAgjTh0W3xKuZ9oWbWDHRl-Z1IAJkWTPkH0?via=computer.surgery&via=matrix.org&via=mozilla.org
[Kim Minh]: https://matrix.to/#/@kaplan:matrix.org
[first commit]: https://github.com/mypdns/MK-Cleaner/commit/61c0af0d09cf87a51ca476abf6ceafb9a76e6012
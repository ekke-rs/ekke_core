# ekke_core
Main server component for ekke


# Toolset

  - actor model: actix
  - logging: slog
  - cmd line params: clap
  - serialization: serde
  - thread sync: parkin_lot, futures-locks or crossbeam, evmap (lockfree hashmap), swym (transactions)
  - hashmap: hashbrown
  - async: io tokio, stdfutures
  - websockets: tokio-tungstenite
  - unix socket: tokio-uds
  - profiling: criterion?
  - fuzzing: cargo-fuzz, afl
  - visually recognisable hashes: snowhash?
  - unique ids: for now 128bit number from rand::Rng
  - process tracking: tokio_process

## Spare tools (not used yet)

  - cargo readme Write readme in lib.rs or main.rs and have examples verified by rustdoc, have the readme be exactly like the api docs intro.
  - inert: allow using non-sync stuff multithreaded (routes?)
  - pretty assertions (colored diffs for assert_eq)
  - rust-dominator (alternative zero-cost DOM implementation)
  - sysinfo (for getting information about the system, disks, memory usages, process list, network, ...)
  - log_derive (automatically generate log code for fn returning a result)
  - typetag (serialize trait objects)
  - hubcaps (github api rust bindings, does not do labels, nor graphql) -> in ruby: gqli.rb
    for github, see: https://github.com/octokit/graphql-schema/blob/master/schema.graphql
    graphql_client in rust
  - https://bors.tech/ for integration testing on pull requests
  - Crate compact -> used in kay to send actor messages over network boundaries
  - methods for getting a global log pointer without mutex: https://unhandledexpression.com/general/2017/08/23/adventures-in-logging.html
  - a global slog!: https://stackoverflow.com/questions/47342036/why-doesnt-a-lazy-static-sloglogger-print-until-a-non-static-logger-is-used

# TODO

- clean up handler for RegisterApplication (error handling)
- fix hyper example in async book
- use tokio process to run code when child dies
- implement debug for every public type
- write derive macros for actix::Actor and actix::MessageResponse
- clap should move back to individual apps

- http server
  - which one? hyper for now
  - use websockets? disposition: yes

- use pipes instead of uds for added security. On windows there is a tokio named pipes, on linux tokio_file_unix

- warn unused crates doesn't work
- panic logging does not work
- Never panic, but do a gracefull shutdown finishing existing requests but stop taking new ones.
- unit tests
- better docs, and figure out how to cross reference.
- fuzzing and serious auditing: https://medium.com/@shnatsel/auditing-popular-rust-crates-how-a-one-line-unsafe-has-nearly-ruined-everything-fab2d837ebb1
- measuring performance/optimizing, check Criterion crate
- investigate the possibility of using actix streamhandler for dispatcher
- investigate actix service actors, notably for dispatcher, so we can get it's address from the registry, also for error handling actor, etc. Use supervised.



- frontend dev
  - yew? react?

- db
  which one? graph?


# Design notes

## UI

 - check https://developer.apple.com/design/human-interface-guidelines/macos/overview/themes/
         https://docs.microsoft.com/en-us/windows/desktop/uxguide/guidelines

## Tag based search
-------------------

Search is elimination! Show hints to the user of what they can type to eliminate stuff with as little typing as possible.
Use bold and color to show the user where the letters they have already typed show up in the top results and show them
what letters they might type to cut it down asap.

As soon as search bar takes focus:
- Seed the elimination by offering broad categories:
  - filesystem -> y
  - start app  -> p
  - configure  -> u
  - open/recent -> n
  - logs        -> lo
  - media (music/videos/pictures) -> d
  - internet    -> in
  - ...

- next level (filesystem example):
  -

## Non-linear text
Just as tag based file browser is about having more ways to get to something and to group things than just directory hierarchy, we can also navicate text (guides, docs, tutorials) in a non-linear fashion. With breadcrumb trails, maybe with allowing to choose starting points (im a dev, im a user, I have intermediate exp with the topics), and end goals (I would like to get this done).

### Visualisation
- Nice breadcrumbs view: https://www.howtographql.com/

# Project management

## Issue and pull request labels

Type:
  - problem
  - improvement
  - feature
  - question
  - feedback

Nature:
Nature ⦔ Broken
Nature ⦔ Security
Nature ⦔ Performance
Nature ⦔ Usability
Nature ⦔ Cosmetics


Priority:
Priority ⦔ Critical
Priority ⦔ High
Priority ⦔ Normal
Priority ⦔ Low
Priority ⦔ Perfectionist


Est. Work:
Est. Work ⦔ 1h
Est. Work ⦔ 3h
Est. Work ⦔ 1d
Est. Work ⦔ 2d
Est. Work ⦔ 3d
Est. Work ⦔ 4d
Est. Work ⦔ 1w
Est. Work ⦔ 2w
Est. Work ⦔ 3w
Est. Work ⦔ 1m
Est. Work ⦔ 1m+


Platform:
Platform ⦔ Linux
Platform ⦔ Mac
Platform ⦔ Windows
Platform ⦔ Bsd
Platform ⦔ Android


Difficulty:
Difficulty ⦔ Research
Difficulty ⦔ Software Design
Difficulty ⦔ Impl Hard
Difficulty ⦔ Impl Intermediate
Difficulty ⦔ Impl Easy
Difficulty ⦔ Mindless Chore

Affects:
⦔ One Crate
⦔ Several Crate

# ekke_core
Main server component for ekke

# TODO

- check zeromq guide for pitfalls when handrolling messaging systems
- warn unused crates doesn't work
- panic logging does not work
- read about lifetimes!
- Never panic, but do a gracefull shutdown finishing existing requests but stop taking new ones.
  Make certain addr globally accessible with a singleton, like the Ekke actor which can handle shutdown.
- create request_void macro
- unit tests
- configuration?
- better docs, and figure out how to cross reference.
- fuzzing and serious auditing: https://medium.com/@shnatsel/auditing-popular-rust-crates-how-a-one-line-unsafe-has-nearly-ruined-everything-fab2d837ebb1
- measuring performance/optimizing, check Criterion crate

- http server
  - which one?
  - use websockets?

- frontend dev
  - yew? react?

- db
  which one? graph?


Tag based search
----------------

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

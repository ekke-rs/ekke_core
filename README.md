# ekke_core
Main server component for ekke


# Toolset

  - actor model: actix
  - logging: slog
  - cmd line params: clap
  - serialization: serde
  - thread sync: parkin_lot, futures-locks or crossbeam
  - hashmap: hashbrown
  - async: io tokio, stdfutures
  - websockets: tokio-tungstenite
  - unix socket: tokio-uds
  - profiling: criterion?
  - fuzzing: cargo-fuzz, afl
  - visually recognisable hashes: snowhash?
  - unique ids: for now 128bit number from rand::Rng
  - process tracking: tokio_process


# TODO

- http server
  - which one? actix-web for now...  fast, async and probably it's secure enough. Should be reaudited though. Fuzz-tested.
  - use websockets? disposition: yes

- configuration?
- only bind to one socket for all peer apps? peer authentication?
- warn unused crates doesn't work
- panic logging does not work
- read about lifetimes!
- Never panic, but do a gracefull shutdown finishing existing requests but stop taking new ones.
  Make certain addr globally accessible with a singleton, like the Ekke actor which can handle shutdown.
- unit tests
- better docs, and figure out how to cross reference.
- fuzzing and serious auditing: https://medium.com/@shnatsel/auditing-popular-rust-crates-how-a-one-line-unsafe-has-nearly-ruined-everything-fab2d837ebb1
- measuring performance/optimizing, check Criterion crate
- investigate the possibility of using actix streamhandler for dispatcher
- investigate actix service actors, notably for dispatcher, so we can get it's address from the registry, also for error handling actor, for main application actor etc. Use supervised.



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

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
  - for http traffic use content adressable ressources and cache-control: immutable (https://code.fb.com/web/this-browser-tweak-saved-60-of-requests-to-facebook/)
  - derive_more for extra derives (display, add, ...)
  - wasm: wasm-bindgen, wasm-bindgen-futures


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
  - templating, look at yarte?
  - validation: validator crate
  - parallelization: rayon
  - web animations: Tweek/Tween
  - error handling, look at: https://epage.github.io/blog/2018/03/redefining-failure/
  - CSS look into SUIT vs BEM vs ???
  - web ui: google reference impls: https://googlechromelabs.github.io/ui-element-samples/
  - catflap and cargo watch for autoreloading
  - tokio-trace potentially better logging than slog!
  - grpc: maybe better than hand rolled ipc? also for logging btw
  - genome browser in wasm: http://2020.ensembl.org/app/browser/GRCh38_demo/ENSG00000139618?region=13:32196775-32543698
  - https://docs.rs/newtype_derive/0.1.6/newtype_derive/ lets you create transparent new type with deref...
  - lagom: microservices with actors, maybe look for inspiration: https://www.lagomframework.com/

# TODO

- clean up handler for RegisterApplication (error handling)
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

trigger focus searchbar by single control or alt key?

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

## Sub commands

Filter already returned results by a subsearch, or issue a command without changing context, ctrl+enter to open a second search bar whilst keeping the first one...


## commands as tags

the way to talk to the programs is by issuing commands. Command suggestions have high priority while returning search results and shall be shown above other suggestions. A number of general commands shall be common to all programs integrated in ekke and it shall be listed in documentation:
- login ? -> not sure actually, some apps might not have users
- logout ? -> not sure actually, some apps might not have users
- `name` things -> eg. a sudoku game you played, you can give it a name
- `notes` add notes to things
- `offline` make stuff available offline
- `identity`
- `hide` to liberate screen space

Commands have their own  suggestions. Once a command is selected, it suggests further info it needs to execute.

### standard interactions
There should be commands that are generic over user interfaces. Eg. UI elements can implement traits and traits can be mapped to commands that can be send with the input bar (need a better name, candy bar?). Eg, plenty of things might implement Filter trait. A table with rows, a list with elements, a text with sentences... A Copy trait, files, clipboard, ...


## Non-linear text
Just as tag based file browser is about having more ways to get to something and to group things than just directory hierarchy, we can also navicate text (guides, docs, tutorials) in a non-linear fashion. With breadcrumb trails, maybe with allowing to choose starting points (im a dev, im a user, I have intermediate exp with the topics), and end goals (I would like to get this done).

### Visualisation
- Nice breadcrumbs view: https://www.howtographql.com/
- snowhash

## Abstract out over services
While thinking on what might be a successful model for recreating things that exist already, it's by having the best interface, but support all. Play go online, fine, seamlessly combine your accounts on kgs, ogs, igs, ... local play against leela, analysis by leela, ... all in one interface.
chat: combine whatsapp, tox, matrix, irc, jabber, signal. Just let you setup accounts for all of these services


## Security & Privacy

For sending messages to one another, applications need to know each others address in order to communicate. It's a bit like sandstorm, but we haven't though about sandboxing yet. In any case a good user interface should exist for the user to define which apps can communicate to which. a visualisation network would be nice.

Applications could reuse the graphical component for identities. Eg, an email/chat client could have a different db for each identity, and could keep information from leaking between accounts (the thing that thunderbird fails to do) and then reuse the nice graphical component for visualizing and letting the user edit permissions of interaction. Another way would be to instantiate one subprocess per identity, but merge the user interface somehow.

### Firewall

Possibility to put all apps in network namespaces and only allow internet connection if configured in firewall app?


### Logviewer ideas

Everything that has a timeline should implement zoom

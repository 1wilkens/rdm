# rdm [![Build Status](https://travis-ci.org/1wilkens/rdm.svg)](https://travis-ci.org/1wilkens/rdm)
A toy display manager written in Rust. Inspired by Slim it aims to provide a simple and elegant login screen.

THIS IS A WIP AND HAS MAJOR ISSUES RELATED TO SECURITY!! USE AT YOUR OWN RISK!

## Current Problems/Questions
- X authorization
    - xauth cookie is generated but not copied to `~/.Xauthority`
    - -> Investigate further
- Session setup / PAM usage
    - Session setup works(tm) but I am still not sure whether all "best practices" are followed
    - -> Review and validate `pam-sys`/`pam-auth` crates
- `systemd-logind` support
    - Investigate what this actually means
        - Do we need to link against `libsystemd.so` and call APIs from there?
    - Implement or mark it as done
- `dbus` support
    - Is this required (maybe for systemd integration)?
    - If so what methods do we need to support?
    - (multi-)`seat`-API
- Architecture
    - Stay with simple client or use server-client to encapsulate login process
    - Likely related to multiseat support
    - Split into more crates (likely not justified)

## Goals
- [ ] Support common standards
    - [x] Linux PAM (in review)
    - [ ] `systemd-logind` (?)
    - [ ] `dbus` (?)
- [ ] Basic theming support

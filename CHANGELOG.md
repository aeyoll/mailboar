# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v0.3.2 - 2024-07-30
#### Build system
- **(docker)** update static asset location - (6c7de82) - *aeyoll*
- **(docker)** update alpine image - (2284d57) - *aeyoll*
- allow to publish backend and frontend crates - (62a627a) - aeyoll
#### Documentation
- bump required rust version - (01159fd) - aeyoll

- - -

## v0.3.1 - 2024-07-29
#### Build system
- **(deps)** update Rust dependencies - (02a961d) - *aeyoll*
- **(deps)** upgrade JavaScript dependencies - (73145d4) - *aeyoll*
- specify NODE_ENV when running build scripts - (a3d56b2) - aeyoll
- allow to inspect rspack builds with RSDOCTOR=true - (ebce69d) - aeyoll
- disable sass warnings for dependencies - (5b51d12) - aeyoll
#### Performance Improvements
- prevent page reload when going back to the index after going to a message page - (7dd850f) - aeyoll

- - -

## v0.3.0 - 2024-07-21
#### Build system
- **(deps)** upgrade JavaScript dependencies - (14a2520) - *aeyoll*
- **(deps)** upgrade Rust dependencies - (1633fa4) - *aeyoll*
- **(docker)** update Dockerfile for cross-compiling - (f9660b1) - *aeyoll*
#### Features
- add keep-alive to the sse route - (2c3083a) - aeyoll
#### Refactoring
- migrate to rspack (#6) - (3acd5e0) - aeyoll
- backport tiny-mailcatcher in the project and rewrite it with axum (#5) - (169d67d) - aeyoll
- improve html message display - (d12f038) - aeyoll

- - -

## v0.2.0 - 2023-02-19
#### Bug Fixes
- update message display when only the source is available - (a283806) - aeyoll
- add an html content-type header to the "main" response - (5f1325c) - aeyoll
- remove a typo - (8b447e3) - aeyoll
- update routing to match tiny-mailcatcher's - (30099da) - aeyoll
- correct the navbar home link - (db34568) - aeyoll
- improve message definition display - (f470256) - aeyoll
- add a body to the html message if its not present - (4fef831) - aeyoll
- display the html iframe only once - (1ca86ac) - aeyoll
- add an offset to the frame height - (3545770) - aeyoll
#### Build system
- **(deps)** upgrade Rust dependencies - (22b99cd) - *aeyoll*
- **(deps)** upgrade JavaScript dependencies - (83ad316) - *aeyoll*
- **(deps)** bump nanoid from 3.1.30 to 3.3.4 - (a372a73) - dependabot[bot]
- **(deps)** bump follow-redirects from 1.14.5 to 1.15.0 - (41d9321) - dependabot[bot]
- **(deps)** bump minimist from 1.2.5 to 1.2.6 - (d0c068e) - dependabot[bot]
- upgrade all js dependencies - (134baed) - aeyoll
- update deps - (10ae013) - aeyoll
- add a Docker image - (a764bed) - aeyoll
- auto-install husky hooks when running "yarn" - (5a1995e) - aeyoll
- add PurgeCSS configuration - (5e17bb7) - aeyoll
- disable google fonts - (e52bb62) - aeyoll
- update Imagemin plugin configuration - (baee11a) - aeyoll
- disable the vue/max-attributes-per-line rule - (f51fe2f) - aeyoll
- add vuejs eslint config - (45de968) - aeyoll
#### Continuous Integration
- add github workflows - (44f270c) - aeyoll
#### Documentation
- improve installation instructions - (03e20d8) - aeyoll
- create LICENCE file - (5f8d9e6) - aeyoll
- add README - (515d4ec) - aeyoll
#### Features
- allow to specify the api url from command line arguments - (971a133) - aeyoll
- display images in the html format - (8be1cbc) - aeyoll
- display the html format first - (55b69a3) - aeyoll
- add gzip headers to static assets - (28494fc) - aeyoll
- improve plain message style - (ad07519) - aeyoll
- improve message tab style - (83f0d7d) - aeyoll
- improve url format - (5cc4fd5) - aeyoll
- use vuex to store and read the api address - (a016294) - aeyoll
- improve message list display - (aee53d3) - aeyoll
- add a button to delete all messages - (3a728c0) - aeyoll
- add a button to delete a message - (fea5b2c) - aeyoll
- add titles to every page - (69e6592) - aeyoll
- display the attachments - (51334d9) - aeyoll
- go to the message detail by clicking the whole line - (55d3bf1) - aeyoll
- sort index list by created date (latest messages first) - (e98ad75) - aeyoll
- set the html iframe height depending on its content - (cced13c) - aeyoll
- add html email preview - (c16abac) - aeyoll
- display the source of an email - (4a23335) - aeyoll
- use a mixin to compute an email's informations - (d034eeb) - aeyoll
- display the received date in a relative format - (92fcd6e) - aeyoll
- improve email display in the homepage - (bdd1d36) - aeyoll
- display a message when there is no email to show - (b38593c) - aeyoll
- add a basic message page - (6dfc1e6) - aeyoll
- add the mail listing page - (6fc15d9) - aeyoll
- add the frontend webserver with warp - (0ee3f52) - aeyoll
#### Miscellaneous Chores
- add cog configuration - (c51ee08) - aeyoll
- bump version - (7b79cfb) - aeyoll
- bump version - (ebf70cb) - aeyoll
- update cargo deps - (001b743) - aeyoll
- bump version - (48506fd) - aeyoll
- bump version - (7517e39) - aeyoll
- update rust deps - (62e4d9d) - aeyoll
- prepare Cargo.toml for publication - (374eba0) - aeyoll
- update javascript deps - (9256dec) - aeyoll
- update tiny-mailcatcher - (f8096db) - aeyoll
- upgrade node deps - (c0b6cd5) - aeyoll
- remove the main nav - (75769eb) - aeyoll
- code styling - (d666179) - aeyoll
- run eslint with the new configuration - (a5d8c9f) - aeyoll
- import tinymail-catcher - (e7d9f63) - aeyoll
- add commitlint configuration - (53cc4e7) - aeyoll
#### Refactoring
- rewrite everything with axum (#4) - (81082f7) - aeyoll
- move the attachment to a separate component - (c66e997) - aeyoll
- simplify mail definition list - (49e2840) - aeyoll
- remove deprecated filters - (402627a) - aeyoll
- rename the function which handle iframe communication - (20d2563) - aeyoll
- split email definition to a separate file - (f767e22) - aeyoll
#### Style
- capitalize the message formats - (b17404e) - aeyoll
- add stripes to the main table - (9914972) - aeyoll

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).

# CloudUp

[![Rust](https://github.com/jyrama/cloudup/actions/workflows/rust.yml/badge.svg)](https://github.com/jyrama/cloudup/actions/workflows/rust.yml)

Stuff for working with UpCloud APIs. You know, this company: [https://upcloud.com/](https://upcloud.com/).

The cloudup library can has some [reqwest](https://github.com/seanmonstar/reqwest) based API thingamajigs, originally written for ad hoc purposes during some late Wednesday evening during a certain pandemic.
Nevertheless the library shall see some more or less major rework under the hood in the future.

The Uppermine sub-project contains an example of the libs usage. Set some env variables and it run it-
it will prepare you an UpCloud virtual server with Java pre-installed and its firewall configured.
Just add your `minecraft.jar`.

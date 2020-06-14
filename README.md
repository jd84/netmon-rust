# netmon-rust

netmon-rust is the rust binding for netmon. netmon itself is a very simple network library for windows written in C++ to get some deep insights. The [winapi][1] project struggles with the maintances and future development is no guranteed.
Some network stuff ist currently no supported by the [winapi][1] project. netmon links direct to the winapi and is desined to integrate with ease into rust. This library is no desined to run in other environments.

At the moment its much easier to keep the stuff that is hard to port to rust in C++.

[1]: https://crates.io/crates/winapi
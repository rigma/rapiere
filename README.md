# rapiere

A pure Rust implementation of Google's [AIP 160][aip160].

This crate is still under construction and is not considered *stable*. Usage
in production environment *is not advised*.

## Motivation

Google's [AIP 160][aip160] goal is to provide common structured syntax for both
technical and non-technical audience that can be used to express *filters*.

A *filter* is a set of constraints that are used to exclude rows from a data table,
or resources from a RESTful API endpoint for instance. Usually, when meeting such
needs, a developer would develop specific needs *its* API, regardless of evolving
requirements or inter-operability between APIs. In that regard, [AIP 160][aip160]
is defining syntax that can be interpreted by different services in the same
fashion.

This project is an attempt to port this specification into Rust ecosystem.

## License

This project is distributed under the terms of both the MIT license and the Apache
License (Version 2.0).

See [LICENSE-APACHE][apache] and [MIT-License][mit] for details.


[aip160]: https://google.aip.dev/160
[apache]: LICENSE-APACHE
[mit]: LICENSE-MIT

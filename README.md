# package-deps-list

This lists all dependencies located in a Crates.lock file in a `name-version` format
as a small helper utility for creating Gentoo ebuilds that build rust projects.

## Usage

```sh
package-deps-list ./Cargo.lock
```

Note that the Cargo.lock will contain the project itself as a package in the list, it will likely need to be removed.

# Codesync

Codesync helps you manage your mass of code repositories across computers.
It does this by setting up your development tree and checking that the
repositories are correctly synced up.

## Usage

List all your repositories, along with their state.
```
$ codesync list
```

You can sync all repositories from a particular user or organization.
Based on the language type of each repository and your configuration it will try to figure out where it should be stored.
```
$ codesync from github.com/goulash
```

You can sync all your local repositories. This will traverse your code root, taking your configuration into account,
and check that each repository has no uncommited changes and fast-forward pulling.
```
$ codesync local
```

If you have a completely new setup and are lacking a configuration, you can kickstart your development tree with:
```
$ codesync init
```

## Configuration

Your configuration file may look like this:
```yaml
root: "~/lang"
accounts:
  - "github.com/cassava"
  - "github.com/goulash"
  - "gitlab.com/cassava"
  - "gitlab.com/benmorgan"
  - "bitbucket.org/cassava"
  - "bitbucket.org/benmorgan"
languages:
  go: {
    root: "$GOPATH"
    }
  rust: {
    root: "~/lang/rust"
    }
repositories:
  - local: "rust/codesync"
    remote: "github.com:cassava/codesync.git"
  - local: "rust/vanity"
    remote: "github.com:cassava/vanity.git"
```


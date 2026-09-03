# ExDateUtil

**Description**
Rrule parsing and utils based on rust nif

- https://docs.rs/rrule/latest/rrule/index.html
- https://github.com/fmeringdal/rust-rrule

Only does this for now: ExDateUtil.Rrule.next("DTSTART;TZID=Etc/UTC:20191220T020000\nRRULE:FREQ=MONTHLY;BYMONTHDAY=28,29,30,31;BYSETPOS=-1", 10)
## Installation

Published to both hex.pm and the private `flickswitch` hex repository. Public
consumers add:

```elixir
def deps do
  [
    {:exdateutil, "~> 1.1"}
  ]
end
```

Flickswitch projects take it from the private repo instead. Register the
repository once per machine — reads need the auth token, ask SRE (this is the
read token, not the publish token):

```sh
curl -sS https://hex.flickswitch.cloud/repos/flickswitch/public_key -o /tmp/flickswitch_hex_public_key.pem
mix hex.repo add flickswitch https://hex.flickswitch.cloud/repos/flickswitch \
  --public-key /tmp/flickswitch_hex_public_key.pem --auth-key <read-token>
```

```elixir
def deps do
  [
    {:exdateutil, "~> 1.1", repo: "flickswitch"}
  ]
end
```

Docs are on [HexDocs](https://hexdocs.pm/exdateutil). The private repo serves
packages only, no docs.

## Releasing

Push a `v<version>` tag matching `@version` in `mix.exs`. The release workflow
builds the precompiled NIFs for all six targets, attaches them to the GitHub
release, rebuilds the checksum manifest from those artifacts, then publishes to
hex.pm and to the private repo.

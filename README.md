# Raxva Relay

Raxva Relay is a nostr relay.

## Features

- **Fast Performance**: [docs/PERFORMANCE.md](docs/PERFORMANCE.md)
- **Personal Relay Support**: Works as your personal relay or an open relay: [docs/PERSONAL_RELAY.md](docs/PERSONAL_RELAY.md)
- **Blossom Server**: Can serve as a blossom server

## Documentation

- [DEPLOYING](docs/DEPLOYING.md) — Deployment guide
- [CONFIG](docs/CONFIG.md) — Configuration guide
- [BEHAVIOR](docs/BEHAVIOR.md) — Relay behavior
- [TOOLS](docs/TOOLS.md) — Command line tools
- [MANAGEMENT](docs/MANAGEMENT.md) — Management API

## Limitations

Raxva Relay does not have any provisions for charging users.

Raxva Relay does not have any provisions for synchronizing events with other relays outside of the nostr protocol.

## Comparison with Other Relays

### strfry

[strfry](https://github.com/hoytech/strfry) is a more mature relay with additional features:

**Advantages:**
- Synchronizing events with other relays efficiently (negentropy)
- Zero-downtime restarts
- Websocket permessage-deflate
- Plugins for event sifting

**Raxva Relay Advantages:**
- Probably faster and more efficient
- Personal relay rules by default
- Extensive IP banning and rate limiting for abuse protection
- Support for NIP-42 (AUTH), NIP-59 (GiftWrap), NIP-65 (Relay Lists), PR 1030 and PR 1325
- Moderation command-line tool and moderation API (PR 1325)
- Blossom server support

### Other Relays

- [nostream](https://github.com/Cameri/nostream)
- [nostr-rs-relay](https://git.sr.ht/~gheartsfield/nostr-rs-relay)
- [khatru](https://github.com/fiatjaf/khatru)

## Git Branches

Use the `latest` branch for production deployments.

⚠️ **Do not use the `master` branch.** Master is updated with breaking changes that are unstable and may require configuration updates. Upgrade instructions will not be announced until official release.

## About

**Raxva Relay** is a fork of [Chorus](https://github.com/mikedilger/chorus), a high-performance nostr relay by Michael Dilger.

### License

This project is licensed under the [MIT License](LICENSE.txt). See LICENSE.txt for details.

### Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

### Original Project

Raxva Relay is built upon the excellent work of the Chorus project. We appreciate the original contributions that made this relay possible.

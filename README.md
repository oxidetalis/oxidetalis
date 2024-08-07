<div align="center">

<img src="https://otmp.4rs.nl/otmp_logo.png" alt="OTMP Logo" width="100"
height="100">

# Oxidetalis

An open-source [OxideTalis Messaging Protocol][`OTMP`] (OTMP) homeserver
implementation written in Rust.

[![Forgejo CI Status](https://git.4rs.nl/oxidetalis/oxidetalis/badges/workflows/ci.yml/badge.svg)](https://git.4rs.nl/oxidetalis/oxidetalis)
[![Forgejo CD Status](https://git.4rs.nl/oxidetalis/oxidetalis/badges/workflows/cd.yml/badge.svg)](https://git.4rs.nl/oxidetalis/oxidetalis)

[![agplv3-or-later](https://www.gnu.org/graphics/agplv3-88x31.png)](https://www.gnu.org/licenses/agpl-3.0.html)

</div>

> **Warning**
>
> The project is still in very early development and is not ready for
> any kind of use yet, the protocol is not finalized and the server is not
> feature complete, also not all protocol features are implemented yet.

## About
[`OTMP`] is a decentralized, end-to-end encrypted chat protocol designed to
prioritize privacy. Unlike existing chat protocols, [`OTMP`] operates without a
central server. When you send a message, it connects directly to your friend's
server, ensuring that your messages are not stored centrally. Only you and your
friend can read the messages, even your servers cannot access their content.

## Key Features
- **Decentralized**: No central server, messages are sent directly to the recipient server by you.
- **End-to-End Encryption**: Messages are encrypted on the client and decrypted
  on the client.
- **Self-Hosted**: You can host your own server and have full control over your
  data.
- **Privacy-Respecting**: No tracking, no ads, no data mining, no
  email/usernames/passwords required.
- **Secure**: Messages are encrypted and signed, and the [protocol is designed to
  be secure][`OTMP`].
- **Lightweight**: Simple protocol, easy to implement, easy to use. No bloat.

## Protocol non-goals
- Group chats
- Voice/video calls

## How to authenticate without usernames and passwords
[`OTMP`] uses a different authroization mechanism than most chat protocols.
Instead of using usernames and passwords, [`OTMP`] uses public/private key pairs
to authenticate users. When you create an account, you generate a key pair on
your device, and the public key is sent to the server. When you sent a request
to the server, you sign the request with shared secret key between you and the
server. This way, the server can verify that the request is coming from you and
authroize the request.

## E2EE, how does it work?
[`OTMP`] key pairs are used for more than just authroization, they are also used
for end-to-end encryption. When you send a message to a friend, you encrypt the
message with shared secret key, the shared secret key is generated by
diffie-hellman key exchange using your private key and your friend's public key.
This way, only you and your friend can read the message, even the server can't
read it.

## Running the server

> **Note**
>
> You must update `OXIDETALIS_CONFIG` in the `docker-compose.yml` file to point
> to the correct configuration file. And you must update the configuration file.

To run the server, you need to have docker and docker-compose installed on your
system. You can run the server by running the following command:
```sh
docker-compose up -d
```

## Contributing
For information on how to contribute to the project, please see the
[CONTRIBUTING.md](./CONTRIBUTING.md) file. You can see the list of contributors in the [CONTRIBUTORS.md](./CONTRIBUTORS.md) file.

## Repository mirrors
- [Codeberg](https://codeberg.org/awiteb/oxidetalis)
- [GitHub](https://github.com/oxidetalis/oxidetalis)

## Licenses
The project is split into multiple crates, each with its own license:
* [`crates/oxidetalis`]: Homeserver implementation, licensed under the GNU
      AGPLv3.
* [`crates/oxidetalis_config`]: Configuration library, licensed under the MIT
      license.
* [`crates/oxidetalis_core`]: The core library, licensed under the MIT license.
* [`crates/migrations`]: Database migrations, licensed under the MIT license.
* [`crates/entities`]: Database entities, licensed under the MIT license.

[`OTMP`]: https://otmp.4rs.nl
[`crates/oxidetalis`]: ./crates/oxidetalis
[`crates/oxidetalis_config`]: ./crates/oxidetalis_config
[`crates/oxidetalis_core`]: ./crates/oxidetalis_core
[`crates/migrations`]: ./crates/migrations
[`crates/entities`]: ./crates/entities

# Collaboration in Zaply

Zaply supports realtime collaboration: shared projects, collaborative
editing, calls, and shared notes. There is one important difference from
most editors:

**Zaply does not run a collaboration service, and there are no Zaply
accounts.** Collaboration only works against a collaboration server
that you (or your team, or your company) host yourselves. The server is
a separate, optional component of this repository. It is never bundled
with the editor, and the editor never sends anything to a Zaply-run
server, because there is none.

If you do not need collaboration, you do not need any of this. The
editor is fully functional on its own.

## How it fits together

- The **editor** connects to a server address you configure.
- The **collaboration server** (the `collab` package in `crates/collab`)
  is a single Rust binary backed by a PostgreSQL database. It serves
  both the realtime collaboration protocol and a small HTTP API,
  including a `/healthz` liveness endpoint.
- Voice calls additionally use a LiveKit server that you host; the
  collaboration server is configured with its address and keys.

## Point the editor at your server

In your Zaply settings file (`Settings: Open Settings File`, or
`~/.config/zaply/settings.json` on Linux), set the top-level
`server_url` to your server's address:

```json
{
  "server_url": "https://collab.example.com"
}
```

The `ZAPLY_SERVER_URL` environment variable overrides the setting.
Restart Zaply after changing it. The collaboration panel then offers a
Connect button for your server instead of explaining this setup.

Authentication is handled by the server deployment (the reference
deployment uses GitHub OAuth credentials that you register for your own
server). There is no Zaply account to create.

## Run the server

### With Docker

The repository ships `Dockerfile-collab`, which builds the server, and
a `compose.yml` that wires it up with PostgreSQL, an S3-compatible blob
store (MinIO), and LiveKit for local development:

```sh
docker compose up
```

### From source

Requirements: a Rust toolchain, PostgreSQL, and the system packages
installed by `script/linux` on Debian/Ubuntu.

```sh
cargo build --release --package collab --bin collab
```

Configure the server through environment variables (a `.env.toml` file
next to the binary also works; see `crates/collab/.env.toml` for the
development defaults):

| Variable | Required | Purpose |
| --- | --- | --- |
| `DATABASE_URL` | yes | PostgreSQL connection string |
| `DATABASE_MAX_CONNECTIONS` | yes | Connection pool size |
| `HTTP_PORT` | yes | Port for the HTTP API and health check |
| `ZAPLY_ENVIRONMENT` | yes | `development`, `staging`, or `production` |
| `LIVEKIT_SERVER`, `LIVEKIT_KEY`, `LIVEKIT_SECRET` | for calls | Your LiveKit deployment |
| `BLOB_STORE_*` | for the API | S3-compatible storage for the extension API |

Then run it:

```sh
./target/release/collab serve all
```

`serve api` runs only the HTTP API, `serve collab` only the realtime
protocol, and `serve all` runs both in one process, which is what most
self-hosters want. Check `http://localhost:8080/healthz` (or your
configured port) to confirm the server is up. Database migrations run
automatically when the server connects.

For production, put the server behind HTTPS (the editor expects a
`https://` address for anything except localhost), run PostgreSQL with
backups, and keep the server version in step with the editor versions
your team uses.

## Status and limits

The server builds in CI on every change to it, and the CI job starts it
against PostgreSQL and verifies the health check. Multi-user behavior
(simultaneous editors in one project, calls, and channel features) is
exercised by the project's own test suite but is not yet verified
end-to-end with the Zaply editor builds published from this
repository. Treat self-hosted collaboration as available but young, and
keep your own backups of anything important.

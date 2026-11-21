# Zaply collaboration server

This crate is the optional, self-hosted collaboration server for
Zaply. It is not bundled with the editor, and the Zaply project runs
no hosted collaboration service: anyone who wants multi-user editing
builds and runs this server themselves, and Zaply connects to the
address set as the server URL in its settings. See COLLABORATION.md
in the repository root for the setup guide.

Continuous integration builds this binary on every change to the
server crates and smoke-tests it against PostgreSQL: the server must
start and answer its /healthz endpoint (workflow:
build-zaply-collab.yml).

# Local Development

## Database setup

Before you can run the collab server locally, you'll need to set up a zaply Postgres database. Follow the steps sequentially:

1. Ensure you have postgres installed. If not, install with `brew install postgresql@15`.
2. Follow the steps on Brew's formula and verify your `$PATH` contains `/opt/homebrew/opt/postgresql@15/bin`.
3. If you hadn't done it before, create the `postgres` user with `createuser -s postgres`.
4. You are now ready to run the `bootstrap` script:

```sh
script/bootstrap
```

This script will set up the `zaply` Postgres database, and populate it with some users. It requires internet access, because it fetches some users from the GitHub API.

The script will create several _admin_ users, who you'll sign in as by default when developing locally. The GitHub logins for the default users are specified in the `seed.default.json` file.

To use a different set of admin users, create `crates/collab/seed.json`.

```json
{
  "admins": ["yourgithubhere"],
  "channels": ["zaply"]
}
```

## Testing collaborative features locally

In one terminal, run Zaply's collaboration server and the livekit dev server:

```sh
foreman start
```

In a second terminal, run two or more instances of Zaply.

```sh
script/zaply-local -2
```

This script starts one to four instances of Zaply, depending on the `-2`, `-3` or `-4` flags. Each instance will be connected to the local `collab` server, signed in as a different user from `seed.json` or `seed.default.json`.

# Deployment

We run two instances of collab:

- Staging (https://example.com)
- Production (https://example.com)

Both of these run on the Kubernetes cluster hosted in Digital Ocean.

Deployment is triggered by pushing to the `collab-staging` (or `collab-production`) tag in GitHub. The best way to do this is:

- `./script/deploy-collab staging`
- `./script/deploy-collab production`

You can tell what is currently deployed with `./script/what-is-deployed`.

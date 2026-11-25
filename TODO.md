# TODO

- [ ] Populate tell.system table (distro, pkg_manager, arch)
- [ ] Implement tell.pkgs(pkgs: string | string[])
- [ ] Implement tell.cfg() and tell.install_cfgs()
- [ ] Implement tell.task() with cmd and unless support
- [ ] Implement CLI commands (apply, plan / --dry-run, sync, edit)
- [ ] Implement git support to grab configuration repos from github/git and move them to `~/.config`
- [ ] state.json to check what has been removed since the last time tell ran

## Commands

`tell apply` (default): Executes the plan (.lua)
`tell plan` (or tell apply --dry-run): Prints what would happen (+ install: ripgrep, ~ link: ~/.config/nvim, - remove: htop) without touching the system.
`tell sync`: Pulls updates (git pull) on all cached tell.cfg repositories and re-applies any changes.
`tell edit`: Opens $EDITOR ~/.config/tell/init.lua and optionally runs tell apply on exit.

## Lua Function Definitions

`tell.system`
Table populated automatically by tell before evaluating your config:

- `tell.system.distro`: Detected distro ID from /etc/os-release (e.g., "arch", "fedora", "ubuntu").
- `tell.system.pkg_manager`: Detected or overridden package manager ("pacman", "dnf", "apt", etc.).
- `tell.system.arch`: CPU architecture (e.g., "x86_64", "aarch64").

`tell.pkgs( pkgs[]: string[] | pkg: string )`
Installs a single package or list of packages using the detected/configured distro package manager (pacman, dnf, apt, etc.).

- Can be called multiple times across modular Lua files—all packages are deduplicated and installed in a single batch.
- Safely ignores nil or false entries in tables so inline conditionals (like is_laptop and "tlp") work cleanly.

`tell.cfg(opts: table): tell.cfg`
Creates a configuration object (tell.cfg) representing a dotfile/config repository:

- `id`: (string, optional): Unique identifier for caching the repo. Defaults to the repository/directory name derived from source.
- `source`: (string, required): Git repository URL or source path.
- `root_path`: (string, optional): Directory in which the config exists in the repository (default `./`)
- `target`: (string, required): Target destination path (e.g., ~/.config/...).
- `method`: (string, optional): `"symlink"` (default) or `"copy"`
- `post_install`: (function, optional): Lua callback executed after the config is linked/copied.

`tell.install_cfgs(cfgs[]: tell.cfg[])`
Queues one or more tell.cfg objects to be cloned/pulled and linked to their target paths.

`tell.task(opts: table)`
Queues a custom shell command or Lua callback to run as part of the execution plan:

- `name`: (string, required): Human-readable label shown in plan output and logs.
- `cmd`: (string | function, required): Shell command string (executed via sh -c) or Lua callback to run when the task executes.
- `unless`: (string | function, optional): Idempotency guard. If a shell command exits with status 0 (or a Lua function returns true), the task is skipped.
- `sudo`: (boolean, optional): Run cmd with elevated privileges (default false).

# Package managers

- [ ] Add pacman support
- [ ] Add dnf support
- [ ] Add apt support

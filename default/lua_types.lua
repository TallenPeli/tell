-- ==============================
-- EDIT `init.lua` TO GET STARTED
-- ==============================

---@meta

---System information populated automatically by `tell` before evaluating your config.
---You can override `pkg_manager` at the top of `init.lua` if needed.
---@class TellSystem
---@field distro_id string Detected distro id from `/etc/os-release` (e.g. `"arch"`, `"fedora"`, `"ubuntu"`).
---@field pkg_manager string Detected or overridden package manager (`"pacman"`, `"dnf"`, `"apt"`, etc.).
---@field arch string CPU architecture (e.g. `"x86_64"`, `"aarch64"`).

---Options passed to `tell.cfg()`.
---@class TellCfgOpts
---@field source string Git repository URL or local directory path.
---@field target string Destination path on your system (e.g. `"~/.config/nvim"`).
---@field id? string Unique identifier for caching the repo. Defaults to the repo/directory name derived from `source`.
---@field root_path? string Subdirectory inside the repository to link. Defaults to `"./"`.
---@field method? "symlink" | "copy" Installation method. Defaults to `"symlink"`.
---@field post_install? fun() Optional Lua callback executed after the config is linked or copied.

---A validated configuration object returned by `tell.cfg()`.
---@class TellCfg : TellCfgOpts
---@field id string
---@field root_path string
---@field method "symlink" | "copy"

---Options passed to `tell.task()`.
---@class TellTaskOpts
---@field name string Human-readable label shown in plan output and logs.
---@field cmd string | fun() Shell command (run via `sh -c`) or Lua callback to execute.
---@field unless? string | fun(): boolean Idempotency guard. Skips the task if the shell command exits `0` or the function returns `true`.
---@field sudo? boolean Run `cmd` with elevated privileges (`sudo`). Defaults to `false`.

---Declarative system and dotfile manager API.
---@class Tell
---@field system TellSystem
tell = {}

---Queues a single package to be installed using the active distro package manager.
---@overload fun(pkg: string)
---Queues a list of packages to be deduplicated and installed in a single batch.
---Safely ignores `nil` or `false` entries so inline conditionals (e.g. `is_laptop and "tlp"`) work cleanly.
---@param pkgs (string | boolean | nil)[]
function tell.pkgs(pkgs) end

---Creates and validates a configuration object representing a dotfile/config repository.
---@param opts TellCfgOpts
---@return TellCfg
function tell.cfg(opts) end

---Queues a single `TellCfg` object to be cloned/pulled and linked to its target path.
---@overload fun(cfg: TellCfg)
---Queues multiple `TellCfg` objects to be cloned/pulled and linked to their target paths.
---@param cfgs (TellCfg | boolean | nil)[]
function tell.install_cfgs(cfgs) end

---Queues a custom shell command or Lua callback to run as part of the execution plan.
---@param opts TellTaskOpts
function tell.task(opts) end

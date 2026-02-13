<h1>
  <picture>
    <img src="./public/bonk.png" alt="❄" width="90" height="90">
  </picture>
  bonk
</h1>

> NixOS workflow multitool — Bonk is a simple wrapper around `nh`, `nix`, and `nix-store` so you don't have to remember all the flags.

## Why bonk?

- **Less typing** - `bonk s` instead of the whole rebuild command
- **Smart defaults** - Detects your hostname, finds your flake and remembers your preferences
- **Remote builds** - Offload compilation to a build server with a single flag
- **Store management** - GC, optimize, repair, and nuke commands in one place
- **Shell completions** - Fish, Bash, and Zsh supported out of the box

## Quick Bonks

```bash
# Switch to your NixOS config
bonk switch

# Update all flake inputs
bonk update

# Try a package without installing
bonk try cowsay -- cowsay "moo"

# Clean up the nix store
bonk store gc
```

## Commands

### switch (alias: s) / boot

Both commands build your NixOS configuration and accept the same flags. The difference is activation:

- **`switch`** - Activate immediately. Wraps `nh os switch`.
- **`boot`** - Add boot entry without switching. Wraps `nh os boot`.

```bash
bonk switch                       # Build and activate now
bonk s                            # Same thing, shorter
bonk boot                         # Build, but only activate on next boot
bonk s -H rune                    # Build a specific host config
bonk s -TH zebes                  # Build config for zebes and deploy via SSH
bonk s -H zebes --target-host root@192.168.1.50  # Deploy to a different SSH address
bonk s -B buildserver             # Offload build to remote host
bonk s --local                    # Force local build (ignore BONK_BUILD_HOST)
bonk s -t                         # Enable --show-trace for debugging
bonk s -s https://cache.example.com -k "key:AAAA..."  # Use extra cache
bonk s -n                         # Dry run - show what would be built
```

Options:

- `-H, --host <HOST>` - Select which NixOS flake configuration to build (defaults to current hostname)
- `-T, --target` - Also deploy to the -H host via SSH. Combine as `-TH <host>` to select a config and deploy in one shot.
- `--target-host <HOST>` - Deploy to a specific SSH target when it differs from -H (e.g. root@192.168.1.50)
- `-B, --build-host <HOST>` - Build on a remote host instead of locally
- `--local` - Force local build, ignoring BONK_BUILD_HOST
- `-t, --trace` - Enable --show-trace for debugging
- `-s, --substituter <URL>` - Extra binary cache URL
- `-k, --key <KEY>` - Trusted public key for the cache
- `-n, --dry-run` - Show what would be built without building

### build (alias: b)

Build packages into the Nix store. Wraps `nix build`.

```bash
bonk build                        # Build default package from current flake
bonk b                            # Same thing, shorter
bonk b .#mypackage                # Build specific package
bonk b -H buildserver             # Offload build to remote host
bonk b --local                    # Force local build (ignore BONK_BUILD_HOST)
bonk b --no-link                  # Build without creating result symlink
bonk b -o my-result               # Custom output symlink path
bonk b -t                         # Enable --show-trace
bonk b -n                         # Dry run
```

Options:

- `<TARGET>` - Package or flake output to build (default package if empty)
- `-H, --build-host <HOST>` - Build on a remote host (overrides BONK_BUILD_HOST)
- `-l, --local` - Force local build, ignoring BONK_BUILD_HOST
- `--no-link` - Don't create the result symlink
- `-o, --out-link <PATH>` - Output path for the result symlink
- `-t, --trace` - Enable --show-trace for debugging
- `-n, --dry-run` - Show what would be built without building

### update (alias: u)

Update flake inputs. Wraps `nix flake update`.

```bash
bonk update                       # Update all inputs
bonk u                            # Same thing, shorter
bonk u nixpkgs                    # Update only nixpkgs
bonk u nixpkgs home-manager       # Update multiple inputs
bonk u --commit                   # Commit the lock file changes
```

Options:

- `<INPUTS>` - Specific inputs to update (all if empty)
- `-c, --commit` - Commit the lock file changes

### try

Create a temporary shell with packages. Wraps `nix shell`.

```bash
bonk try ripgrep                  # Open shell with ripgrep available
bonk try ripgrep fd bat           # Multiple packages
bonk try cowsay -- cowsay moo     # Run command directly
bonk try python3 --pure           # Pure shell (no inherited environment)
bonk try python3 --pure -- python -c "print('hi')"
```

Options:

- `<PACKAGES>` - Packages to make available (required)
- `[COMMAND]` - Command to run after `--` (interactive shell if empty)
- `--pure` - Use a pure shell with no inherited environment

### store

Nix store management subcommands.

#### store gc (alias: clean)

Garbage collect old generations.

```bash
bonk store gc                     # Collect garbage, keep 3 generations
bonk store clean                  # Same thing
bonk store gc -o 7d               # Delete generations older than 7 days
bonk store gc -o 2w               # Delete generations older than 2 weeks
bonk store gc -k 5                # Keep at least 5 generations
bonk store gc -n                  # Dry run
```

Options:

- `-o, --older-than <DURATION>` - Delete generations older than this (e.g., 7d, 2w, 1m)
- `-k, --keep <N>` - Keep at least this many generations (default: 3)
- `-n, --dry-run` - Show what would be deleted without deleting

#### store optimize

Deduplicate the Nix store via hard-linking. Wraps `nix store optimise`.

```bash
bonk store optimize               # Optimize the store
bonk store optimize -n            # Show potential savings without optimizing
```

Options:

- `-n, --dry-run` - Show potential savings without optimizing

#### store repair

Verify and repair store integrity. Wraps `nix store verify` and `nix store repair`.

```bash
bonk store repair                 # Verify and repair all paths
bonk store repair /nix/store/...  # Repair specific paths
bonk store repair --check-only    # Only verify, don't repair
```

Options:

- `<PATHS>` - Specific store paths to repair (all if empty)
- `-c, --check-only` - Only verify, don't repair

#### store nuke

Aggressive full cleanup. Removes all old generations and performs deep GC. Automatically rebuilds boot entries via `nh os boot` after cleanup to keep the system bootable.

```bash
bonk store nuke                   # Full cleanup + rebuild boot entries (will prompt for confirmation)
bonk store nuke -y                # Skip confirmation
bonk store nuke -r                # Also remove result symlinks in current directory
bonk store nuke --no-rebuild      # Skip automatic boot entry rebuild (Bad Idea)
```

Options:

- `-y, --yes` - Skip confirmation prompt
- `-r, --remove-results` - Also remove result symlinks in current directory
- `--no-rebuild` - Skip automatic boot entry rebuild after cleanup

#### store info

Show store statistics.

```bash
bonk store info                   # Show basic stats
bonk store info -d                # Show detailed breakdown
```

Options:

- `-d, --detailed` - Show detailed breakdown

## Global Options

These apply to all commands:

- `-p, --flake-path <PATH>` - Override the flake path
- `-v, --verbose` - Enable verbose output

## Environment Variables

Configure bonk's defaults with environment variables:

| Variable          | Purpose                                           | Example              |
| ----------------- | ------------------------------------------------- | -------------------- |
| `BONK_FLAKE_PATH` | Default flake path                                | `/home/user/nixos`   |
| `BONK_BUILD_HOST` | Default remote build host                         | `buildserver`        |
| `BONK_EXTRA_ARGS` | Extra args passed to nh/nix (colon-separated)     | `--impure:--verbose` |
| `FLAKE`           | Fallback flake path (if BONK_FLAKE_PATH is unset) | `/home/user/nixos`   |

## Installation

### Flake

Add bonk to your flake inputs:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    bonk = {
      url = "github:tophc7/bonk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, bonk, ... }: {
    # Use the overlay to add bonk-nix to pkgs
    nixosConfigurations.yourhostname = nixpkgs.lib.nixosSystem {
      modules = [
        { nixpkgs.overlays = [ bonk.overlays.default ]; }
        # Now you can use pkgs.bonk-nix
      ];
    };
  };
}
```

> **Note:** The package is named `bonk-nix` to avoid conflicts with the `bonk` package in nixpkgs (which is a completely different thing).

### NixOS / Home Manager Module

Both modules work the same way. Add the appropriate module to your config:

- NixOS: `bonk.nixosModules.default`
- Home Manager: `bonk.homeManagerModules.default`

Then configure:

```nix
{
  programs.bonk = {
    enable = true;
    flakePath = /home/user/nixos;  # Your flake location
    buildHost = null;              # null = local builds, or "buildserver"
    extraArgs = [ "--impure" ];    # Extra args passed to nh/nix
  };
}
```

### Try Without Installing

```bash
nix shell github:tophc7/bonk

# Then use it
bonk try fastfetch -- fastfetch
```

## Shell Completions

Bonk includes shell completions for Fish, Bash, and Zsh. When installed via the flake/modules, completions are automatically set up.

If you're building from source:

```bash
# Completions are generated at build time to target/release/build/bonk-*/out/completions/
cargo build --release

# Fish
cp target/release/build/bonk-*/out/completions/bonk.fish ~/.config/fish/completions/

# Bash
cp target/release/build/bonk-*/out/completions/bonk.bash /etc/bash_completion.d/

# Zsh
cp target/release/build/bonk-*/out/completions/_bonk ~/.zsh/completions/
```

## Dependencies

Bonk wraps these tools (they need to be in your PATH):

- `nh` - Used by switch/boot and gc commands
- `nix` - Used by build, update, try, and store commands

The NixOS and Home Manager modules automatically include `nh` as a dependency.

## License

GPL-3.0

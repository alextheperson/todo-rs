# todo-rs

`todo-rs` is a CLI todo list written in Rust.

## Features
**Human Readable** - All of your todo items are stored in `.todo` files in a fully human readable and manually editable format.

**Scoped** - `todo-rs` will search up though your file system to find `.todo` files. This lets you have both user-wide and project-specific todo lists.

**Priority** - It will sort your items based on what priorty you give them.

## Installation
### Flake (NixOS + Home Manager)
First, add `todo-rs` to your flake inputs:
```nix

{
  inputs = {
    # ...
    todo-rs = "github:alextheperson/todo-rs";
    inputs.nixpkgs.follows = "nixpkgs";
    # ...
  }
  # ...
}
```

Then you need to pass it to your HM config, likely with something like this:
```nix

  # /etc/nixos/flake.nix
  # ...
  outputs = inputs@{ self, nixpkgs, home-manager, todo-rs, ... }: {
    # ...
    home-manager.extraSpecialArgs = { inherit todo-rs; };
  }
```

and

```nix
{ config, pkgs, todo-rs, ... }:
```

Finally, you need to add it to you package list:
```nix

home.packages = with pkgs; [
  # ...
] ++ [
  todo-rs.packages.aarch64-linux.default
]
```

## Usage
### Basic Usage
To use `todo-rs`, use the `todo` command. For example, `$ todo list` will list all of you todo items from the current directory and all of its parent directories (up to your home directory).

To create a new todo list in the current directory, use `$ todo new`.

You can add an item with `$ todo add "#todo list" "new item"`.

Mark items as complete or toggle their completion with the `todo complete "item name"` or `todo toggle "item name"` commands respectively.

### Advanced usage
There isn't much room for advanced usage yet, but here are some extra things that you might want to know.

Generally, the names of todo lists are prefixed with a `#` (`#todo list`), you can use this syntax in commands, though currently none of them react differently, apart from removing the prefix before parsing the name.

`todo-rs` supports nested items. If you want to point a command to a nested item, just use slashes (eg `item/sub item/really nested`).

## `.todo` File Syntax
The syntax of the `.todo` files is very simple. The first line is a # followed by the name of the list, then followed by a newline:

```
# Todo List

{{items}}
```

For the actual items. they use `- [ ]` and `- [x]` to represent their completion status, like in some flavors of Markdown. After that they (optionally) have a priority number and/or date, delimited by backslashes. Finally, nested items are represented with indentation. An example file might look like this:

```
# Example Todo List

- [ ] \2\6/13/2026\ Replant the garden
- [ ] \5\ Fix the broken thing
 - [ ] Research seeds
- [x] \4\ Mount the shelf
```

This will render out as:

```
╭ #  Example Todo List (/path/to/list/.todo)
│
├ □ 5 Fix the broken thing
├ ▣ 4 Mount the shelf
├ □ 2 (6/13/2026) Replant the garden
│ ├ □ 0 Research seeds
```

## Roadmap
Things that are going to be added (I don't know in what order):
- Date-based priority bumps.
- Removing items
- Pruning lists
- Putting completed items at the bottom
- TUI editor
- Editing items
- Getting the next item
- Downwards (global) searches

# todo-rs

`todo-rs` is a CLI todo list written in Rust.

## Features
**Human Readable** - All of your todo items are stored in `.todo` files in a fully human readable and manually editable format.

**Scoped** - `todo-rs` will search up though your file system to find `.todo` files. This lets you have both user-wide and project-specific todo lists.

**Priority** - It will sort your items based on what priority you give them. Additionally, you can give items dates, which increases their priority as the date approaches.

**Archives** - Once you complete an item, you can also archive it, which hides it in the list, but keeps it around in case you ever want to look back at what you have accomplished.

**Date Parsing** - You can give it dates like `'tomorrow'`, `'next month'`, or `'january'`, and it should understand what you mean.

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
To use `todo-rs`, use the `todo` command. For example, `$ todo list` will list all of your todo items from the current directory and all of its parent directories (up to your home directory). You can also pass in the `-d` (down) flag to make in go the other way.

To create a new todo list in the current directory, use `$ todo init`.

You can add an item with `$ todo add "#todo list" "new item"`.

Mark items as complete or toggle their completion with the `$ todo complete "item name"` or `$ todo toggle "item name"` commands respectively.

### Advanced usage
`todo-rs` supports nested items. If you want to point a command to a nested item, just use slashes (e.g. `#list/item/sub item/really nested`). If you leave out the name of the list, then it will try to use a list in your current directory.

You can also specify the format that you want the list to output in. For example, I have a Waybar that displays my todo list. In order to get the output into the Pango format that Waybar needs, I use `$ todo list ~ -d --format pango`

## `.todo` File Syntax
The syntax of the `.todo` files is very simple. The first line is a # followed by the name of the list, then optional metadata, followed by a newline:

```
# Todo List
# date 15/dec/2027
# priority 3
# archived

- [ ] One Fish
- [ ] Two Fish
 - [x] Red Fish
- [a] Blue Fish
```

For the actual items. they use `- [ ]` and `- [x]` to represent their completion status, and `- [a]` is an archived item. After that they (optionally) have a priority number and/or date, delimited by backslashes. Finally, nested items are represented with indentation. An example file might look like this:

```
# Example Todo List

- [ ] \2\6/13/2026\ Replant the garden
- [ ] \5\ Fix the broken thing
 - [ ] Research seeds
- [x] \4\ Mount the shelf
```

This will render out as:

```
╭ # Example Todo List (/home/alex/Documents)
│
├ □ 5 Fix the broken thing
│ ╰ □ 0 Research seeds
├ □ 2 (13-Jun-2026) Replant the garden
╰ ▣ 4 Mount the shelf
```

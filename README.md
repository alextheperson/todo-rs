# todo-rs

A CLI Todo List

> □ Task 1
> ▣ ~Task 2~
> ├▣ ~Task 3~
> ╰□ Task 4

## Commands
`todo new` - Create a new todo list in the current directory

`todo add` - Add an item to the active todo list

## File
It creates a file called `.todo`. Inside, the syntax is basically markdown. Here is an example file.

```
# Test Todo

- [ ] - 4 -- Item 1
- [ ] - 3 - 12/13/2025 - Item 2
- [x] - 6 -- Checked item
 - [x] --- Checked sub-item 1
 - [x] --- Checked sub-item 2
  - [x] --- Checked sub-sub-item
 - [x] --- Checked sub-item 3
- [ ] --- Item 3
- [ ] --- Item 4
```

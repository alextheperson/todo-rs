# todo-rs

A CLI Todo List
```
□ Task 1
▣ Task 2
├ ▣ Task 3
╰ □ Task 4
```

## Commands
`todo new` - Create a new todo list in the current directory

`todo add` - Add an item to the active todo list

## File
It creates a file called `.todo`. Inside, the syntax is basically markdown. Here is an example file.

```
# Test Todo

- [ ] \4\\ Item 1
- [ ] \3\15/10/2025\ Item 2
- [x] \6\\ Checked item
 - [x] \0\\ Checked sub-item 1
 - [x] \0\\ Checked sub-item 2
  - [x] \-10\\ Checked sub-sub-item
 - [x] \0\\ Checked sub-item 3
- [ ] \7\24/12/2027\ Item 3
- [ ] \0\\ Item 4
```

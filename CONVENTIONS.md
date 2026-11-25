# Documentation conventions

How every exercise, milestone and project in this repo is documented, so
the repo stays consistent. These conventions follow the roadmap's rules:
every piece of work ships a proof, and every project ships a postmortem.

> Drafted with Claude, then adapted by me.

## 1. Principles

- **Truthful.** Write only what is true and what I can prove. No
  overclaiming ("mastered", "production-ready").
- **Short.** A page a stranger can read in two minutes beats a long one.
- **Same shape every time.** Same file names, same headings, same order.
- **Own words.** Notes explain things the way I understand them.
- **Proof over claims.** "It works" doesn't count without evidence.

## 2. What each piece of work ships

| Level     | Examples              | Files                                                                                            |
| --------- | --------------------- | ------------------------------------------------------------------------------------------------ |
| Exercise  | `e01-unit-converter`  | `README.md`, `NOTES.md`, tests / proof script                                                    |
| Milestone | `m1-...`              | `README.md`, tests, a short `POSTMORTEM.md`                                                      |
| Project   | `p1-hardware-fetcher` | `README.md`, `DESIGN.md`, `PROOF.md`, `POSTMORTEM.md`, plus `UNSAFE.md` if there is any `unsafe` |

## 3. Folder layout

```
rust-systems-crucible/
├── README.md            # overview + progress table
├── CONVENTIONS.md       # this file
├── .gitignore           # target/
├── e01-unit-converter/
│   ├── Cargo.toml
│   ├── src/
│   ├── tests/           # proof script(s)
│   ├── README.md
│   └── NOTES.md
└── p1-hardware-fetcher/
    ├── Cargo.toml
    ├── src/
    ├── tests/
    ├── README.md
    ├── DESIGN.md
    ├── PROOF.md
    └── POSTMORTEM.md
```

Rules:

- Folder names: lowercase, hyphens, start with the roadmap ID
  (`e01-...`, `m1-...`, `p1-...`).
- File names: `README.md`, `NOTES.md`, `DESIGN.md`, `PROOF.md`,
  `UNSAFE.md`, `POSTMORTEM.md`, all uppercase, `.md` extension.
- One Cargo package per folder. Create it from the repo root with
  `cargo new <folder-name>`.

## 4. Templates

### README.md (every folder)

Answers: what is this, how do I run it, how do I know it works.

````markdown
# E01: Unit converter

One or two sentences: what the program does.

## Usage

```
convert <number> <from_unit> <to_unit>
```

Examples with expected output.

## Behaviour

What it supports. Choices I made (for example, 1 KiB = 1024 bytes).

## Errors

| Situation | Exit code | Example |
| --------- | --------- | ------- |

## Build, run, test

Exact commands.

## Status

Version, and what is deliberately missing (for example, "v0.1.0 uses only
Book chapters 1-3; v0.2.0 will use `match`").
````

### NOTES.md (exercises)

Short bullets of concepts I learned, each in my own words and checked
against the docs or the compiler. One idea per bullet.

```markdown
# E01: Notes

- **Concept:** one or two sentences, my words.
- **Mistake I made:** what went wrong and what fixed it.
```

### DESIGN.md (projects)

```markdown
# P1: Design

## Goal

## Diagram

(ASCII or an image: the data flow or the components)

## Key decisions

Decision, alternatives considered, why this one.

## Phase exit test

My answers to the roadmap's questions for this project.
```

### PROOF.md (projects; a short section in README for exercises)

Evidence, not opinion. Say how to reproduce each claim.

```markdown
# P1: Proof

| Claim                                  | How I checked it   | Result |
| -------------------------------------- | ------------------ | ------ |
| Matches the reference tool on N inputs | `tests/proof.sh`   | pass   |
| Fails cleanly on bad input             | failure-path tests | pass   |

## How to reproduce

Exact commands.

## Known gaps

What I did not verify.
```

### UNSAFE.md (only if the project has `unsafe`)

For every `unsafe` block: where it is, what invariant must hold, and why
it holds there. The block itself also carries a `// SAFETY:` comment.

### POSTMORTEM.md (every project and milestone)

The roadmap asks for exactly four things:

```markdown
# P1: Postmortem

## What the system actually did

(from strace, /proc, man pages, traces)

## What my abstraction hid

## What failed

What broke, and how I found out.

## One thing I would redesign
```

## 5. Writing style

- English, present tense, plain sentences. Spell-check before committing.
- Code, commands, file names and exit codes in backticks.
- Fenced code blocks for multi-line commands.
- Use the same exit-code style everywhere: `exit code 1`.
- Numbered lists only for ordered steps. Indent list items under their
  parent so they render correctly.
- Link to sources (Rust Book chapter, TLPI chapter, man page) instead of
  copying text from them.

## 6. Progress table and versions

The top-level `README.md` has one row per piece of work.

- Status values: `planned`, `in progress`, `done`.
- Add a row when I start, not before.
- Add the video link once the video is published.
- When an exercise is rewritten with later knowledge, keep the old version
  and mark it with a Git tag (`e01-v0.1.0`), then add the new one
  (`e01-v0.2.0`).

## 7. Commits

- One logical change per commit, short present-tense message:
  `Add E01 unit converter`, `Add E01 proof script`, `Mark E01 done`.
- Commit author is `Elonith` with the GitHub noreply address.
- Never commit `target/`, secrets, or build output.

## 8. Definition of done

A piece of work is done only when all of these are true:

- [ ] `cargo build` works.
- [ ] `cargo clippy -- -D warnings` is clean.
- [ ] `cargo fmt --check` is clean.
- [ ] Happy-path and failure-path tests pass.
- [ ] README, NOTES (or the project dossier) are written and spell-checked.
- [ ] Progress table updated.
- [ ] Committed and pushed.

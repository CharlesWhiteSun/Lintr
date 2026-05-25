# Spec: Lintr Library API

## ADDED Requirements

### Requirement: Source Linting API

Lintr SHALL expose a public library API named `lint(source, config)` that analyzes Python source text with the enabled rule set.

#### Scenario: Valid Python source is linted

- **GIVEN** valid Python source and a default config
- **WHEN** the caller invokes `lint(source, config)`
- **THEN** Lintr parses the source, runs enabled rules, and returns diagnostics for detected violations

#### Scenario: Disabled rule is skipped

- **GIVEN** valid Python source that would trigger a rule and config that disables that rule code
- **WHEN** the caller invokes `lint(source, config)`
- **THEN** Lintr does not return diagnostics for the disabled rule

### Requirement: File Linting API

Lintr SHALL expose a public library API named `lint_file(path, config)` that reads a Python file and applies the same analysis contract as `lint(source, config)`.

#### Scenario: Existing file is linted

- **GIVEN** a readable Python file and a config
- **WHEN** the caller invokes `lint_file(path, config)`
- **THEN** Lintr reads the file and returns the same diagnostics that `lint(source, config)` would produce for that file content

#### Scenario: Missing file returns IO error

- **GIVEN** a path that cannot be read
- **WHEN** the caller invokes `lint_file(path, config)`
- **THEN** Lintr returns a typed IO error and does not run parser or rules

### Requirement: Config Filtering

Lintr MUST apply config before executing rules, including rule code filtering, category filtering, and per-rule settings.

#### Scenario: Rule code filtering disables one rule

- **GIVEN** source that can trigger multiple rules and config that disables one rule code
- **WHEN** Lintr runs analysis
- **THEN** diagnostics for enabled rules remain and diagnostics for the disabled rule are absent

#### Scenario: Rule setting changes behavior

- **GIVEN** source with a line length between the default limit and a custom `max_line_length`
- **WHEN** Lintr runs with config that sets the custom limit
- **THEN** `E001` evaluates the custom limit instead of the default

### Requirement: Diagnostic Ordering

Lintr MUST return diagnostics in a stable order independent of registry insertion details.

#### Scenario: Multiple diagnostics are sorted deterministically

- **GIVEN** source that triggers diagnostics at multiple ranges
- **WHEN** Lintr returns diagnostics
- **THEN** diagnostics are sorted by start offset, then end offset, then rule code

### Requirement: Parse Error Behavior

Lintr MUST return a typed parse error when Python source cannot be parsed, and MUST NOT execute lint rules for that source.

#### Scenario: Invalid Python source fails before rules execute

- **GIVEN** syntactically invalid Python source
- **WHEN** the caller invokes `lint(source, config)`
- **THEN** Lintr returns a typed parse error
- **THEN** Lintr does not return partial rule diagnostics for that source

# Spec: Lintr Development Process

## ADDED Requirements

### Requirement: TDD Implementation Flow

Every Lintr behavior change MUST follow a Red-Green-Refactor loop unless the task explicitly documents why automated tests are not feasible.

#### Scenario: New rule starts with failing tests

- **GIVEN** a new lint rule is being implemented
- **WHEN** development starts
- **THEN** the first code change defines failing positive, negative, and boundary tests for that rule
- **THEN** implementation only proceeds after the expected failure is observed or documented

#### Scenario: Public API behavior starts with integration tests

- **GIVEN** a change to `lint()` or `lint_file()` behavior
- **WHEN** development starts
- **THEN** an integration test under the root library crate defines the expected behavior before implementation

### Requirement: SOLID Crate Boundaries

Lintr MUST preserve SOLID-oriented crate boundaries and avoid dependency cycles.

#### Scenario: Core crate remains dependency-light

- **GIVEN** a change to `lintr-core`
- **WHEN** dependencies are reviewed
- **THEN** `lintr-core` does not depend on `lintr-parser`, `lintr-config`, `lintr-rules`, or `lintr`

#### Scenario: Rules are substitutable through a trait

- **GIVEN** multiple built-in rules
- **WHEN** the engine runs enabled rules
- **THEN** each rule is invoked through `Rule<PythonLintContext>` or an equivalent abstraction rather than concrete rule types

### Requirement: Mandatory Verification Commands

Lintr changes MUST run workspace formatting, linting, testing, and build verification before completion unless a blocker is documented.

#### Scenario: Workspace verification succeeds

- **GIVEN** code or behavior was changed
- **WHEN** the task is ready to finish
- **THEN** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and `cargo build --workspace` have been executed or their blocker is documented

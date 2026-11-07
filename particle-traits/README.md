# Crate particle-traits

Core traits for the particle component system.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![crates.io](https://img.shields.io/crates/v/particle-traits.svg)](https://crates.io/crates/particle-traits)
[![docs.rs](https://docs.rs/particle-traits/badge.svg)](https://docs.rs/particle-traits)

Part of the [Particle project](https://rust-particle.github.io).

Note that this crate is *by default* `no_std` and has

## Content

| Module          | Types                                                                                                                                                                                                                                                                 |
|-----------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| builders        | `Builder`, `InfallibleBuilder`, `HasBuilder`, `HasInfallibleBuilder`                                                                                                                                                                                                  |
| commands        | `CommandOnce`, `CommandOnceWith`, `Command`, `CommandWith`, `CommandMut`, `CommandMutWith`                                                                                                                                                                            |
| properties      | `Singleton`, `Opaque`,                                                                                                                                                                                                                                                |
| + classifiers   | `Classifier`, `HasClassifier`, `HasClassifierMut`, `HasClassifiers`, `HasClassifiersMut`                                                                                                                                                                              |
| + configuration | `HasConfiguration`, `HasConfigurationMut`, `HasOptionalConfiguration`, `HasOptionalConfigurationMut`, `ConfigurationBuilderFragment`                                                                                                                                  |
| + descriptions  | `Description`, `HasDescription`, `HasDescriptionMut`, `HasOptionalDescription`, `HasOptionalDescriptionMut`, `DescriptionBuilderFragment`                                                                                                                             |
| + errors        | `ErrorCode`, `HasErrorCode`, `HasErrorCodeMut`, `Severity`, `HasSeverity`, `HasSeverityMut`, `StructuredError`, `ErrorCodeBuilderFragment`, `SeverityBuilderFragment`                                                                                                 |
| + icons         | `TextIcon`, `HasTextIcon`, `HasTextIconMut`, `TextIconBuilderFragment`                                                                                                                                                                                                |
| + identifiers   | `Identifier`, `IdGenerator`, `HasIdentifier`, `HasIdentifierMut`, `HasParentIdentifier`, `HasParentIdentifierMut`, `HasChildIdentifiers`, `HasChildIdentifiersMut`, `IdentifierBuilderFragment`, `ParentIdentifierBuilderFragment`, `ChildIdentifiersBuilderFragment` |
| + labels        | `DisplayLabel`, `HasDisplayLabel`, `HasDisplayLabelMut`, `HasOptionalDisplayLabel`, `HasOptionalDisplayLabelMut`, `HasDisplayLabels`, `HasDisplayLabelsMut`, `DisplayLabelBuilderFragment`                                                                            |
| + names         | `Name`, `HasStaticName`, `HasName`, `HasNameMut`, `HasOptionalName`, `HasOptionalNameMut`, `NameBuilderFragment`                                                                                                                                                      |
| + priorities    | `Priority`, `HasPriority`, `HasPriorityMut`, `PriorityBuilderFragment`                                                                                                                                                                                                |
| + scopes        | `Scope`, `HasScope`, `HasScopeMut`, `HasOptionalScope`, `HasOptionalScopeMut`, `ScopeBuilderFragment`                                                                                                                                                                 |
| + states        | `State`, `StateSet`, `StateKind`, `HasState`, `HasStateMut`, `HasOptionalState`, `HasOptionalStateMut`, `HasPotentialStates`, `StateBuilder`, `StateBuilderFragment`                                                                                                  |
| + tags          | `Tag`, `HasTags`, `HasTagsMut`, `TagBuilderFragment`                                                                                                                                                                                                                  |
| + values        | `Value`, `HasValue`, `HasValueMut`, `HasOptionalValue`, `HasOptionalValueMut`, `ValueBuilderFragment`                                                                                                                                                                 |
| strings         | `AsStr`, `CheckedFromStr`, `InfallibleFromStr`, `UncheckedNewStr`, `StringLike`                                                                                                                                                                                       |
| validation      | `Validator`, `RuleGroup`, `Rule`, `RuleFilter`, `Action`, `TargetRules`, `HasValidation`, `HasFastValidation`, `HasValidator`, `HasValidatorMut`, `ValidatorBuilderFragment`                                                                                          |
| wrappers        | `AsInner`, `AsInnerMut`, `IntoInner`, `FromInner`, `Untrusted`, `Trusted`                                                                                                                                                                                             |

## Patterns

TBD

### Single-Value Properties

### Multi-Value Properties

### Builders & Fragments

## Licenses

See the repository files
[LICENSE-Apache](https://github.com/johnstonskj/rust-particle/blob/main/LICENSE-Apache),
and [LICENSE-MIT](https://github.com/johnstonskj/rust-particle/blob/main/LICENSE-MIT).

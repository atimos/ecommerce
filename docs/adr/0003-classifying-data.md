# Classifiyng data

* Status: approved
* Date: 2021-02-11

## Context and Problem Statement

The system needs to have a way to know what information is private and public,
to decide if it can send it out via api, store it as clear text in database or log it
## Considered Options

* wrapper types
* serde field attributes
* custom marco that optionally implements serdes serialize deserialize
* multiple types
* multiple types, macro for access methods
* trait with different access methods

## Decision Outcome

Chosen option: "trait with different access methods", because it can handle all types and structs containing other structs

### Positive Consequences

Will enable classification of all values in a straight forward way

### Negative Consequences

If the output step does not call the convert methods it will send out the data unmasked

## Pros and Cons of the Options

### wrapper types

use wrapper types to specify the class,
a trait containing public, secret, confidential, personal methods and implement them on all types

    struct User {
        ssn: Confidential<String>,
        password: Secret<String>,
        age: u8,
    }

* Good, easy to see follow
* Bad, need to derive or implement traits that the inner value has
* Bad, hard to specify how to mask while serializing
* Bad, easy to miss a wrapper somewhere, for example if ssn is used in other structs

### serde field attributes

    #[derive(Serialize, Deserialize)]
    struct User {
        #[serde(with = "confidential")]
        ssn: String,
        #[serde(with = "password")]
        password: String,
        age: u8,
    }

* Good, already exists
* Bad, requires serde for every storage, logging, api
* Bad, can be easy to miss, it doesnt give any context to why it exists
* Bad, hard to compose
* Bad, hard specify what level should be used while serializing

### custom marco that optionally implements serialize deserialize

Implement Serialize and Deserialize if specified otherwise it will only add the Classify methods

    #[derive(Classify, Serialize, Deserialize)]
    #[Classify(DefaultKind(public), Serialize, Deserialize)]
    struct User {
        #[Kind(pii)]
        ssn: String,
        #[Kind(password)]
        password: String,
        age: u8,
    }

* Good, easy to follow
* Good, easy to compose, just add more kinds
* Good, works together with serde
* Bad, every serialization framework used needs to be implemented in the macro
* Bad, hard to specify what level should be used while serializing

### multiple types

Use different types depending on where its used.
Do not implement any serialization on internal types and
implement the From trait to convert between them.

    struct User {
        name: String,
        ssn: String,
        password: String,
        age: u8,
    }

    mod logging {
        struct User {
            name: String,
            age: u8,
        }

        impl From<super::User> for User {
            fn from(source: super::User) -> Self {
                Self {
                    name: source.name,
                    age: source.age,
                }
            }
        }
    }

optionally using macro for conversion:

    mod logging {
        #[derive(ConvertFrom)]
        #[ConvertFrom(super::User)]
        struct User {
            name: String,
            age: u8,
        }
    }

* Good, easy to see what is used where
* Good, easy to implement
* Meh, needs implementation of From trait for everything,
  could be mitigated by using a macro for the implementation
* Bad, it could be easy to miss and
  use an internal type where an interface one should be used
* Bad, no classification is specified, everything is ad hoc
* Bad, have to remember to update all types and From implementations if core type is updated

### multiple types, macro for access methods

Use different types depending on where its used.
Implement Classify on internal type, do not implement serialization
implement the From trait to convert between them by using accessors created by the macro,
like convert_ssn_public, convert_ssn_secret, convert_ssn_pii that will mask the data depending on Kind.
Could possibly create a conversion macro that generates the From implementation.

    #[derive(Classify)]
    #[DefaultKind(public)]
    struct User {
        name: String,
        #[Kind(pii)]
        ssn: String,
        #[Kind(password)]
        password: String,
        age: u8,
    }

    mod logging {
        struct User {
            name: String,
            ssn: String,
            password: String,
            age: u8,
        }

        impl From<super::User> for User {
            fn from(source: super::User) -> Self {
                Self {
                    name: source.convert_name_public(),
                    ssn: source.convert_ssn_public(),
                    password: source.convert_password_public(),
                    age: source.convert_age_public(),
                }
            }
        }
    }

optionally using macro for conversion:

    mod logging {
        #[derive(ConvertFrom)]
        #[ConvertFrom(super::User, public)]
        struct User {
            name: String,
            ssn: String,
            password: String,
            age: u8,
        }
    }

* Good, easy to see what is used where
* Good, easy to implement
* Meh, implementations for From trait could accidentally use the wrong class,
  could be made less error prone by using a macro for the implementation
* Meh, have to remember to update all types and From implementations if core type is updated,
  could be helped by compiler error if using a macro for From implementations
* Meh, needs implementation of From trait for everything,
  could be mitigated by using a macro for the implementation
* Bad, easy to use the wrong type in the wrong place
* Bad, handling recursively if a type contains a type that implements Classify is not possible

### trait with different access methods

a trait containing public, secret, pii methods and implement them on all types
a derive macro for implementing the trait on structs with optional masking function

    trait Classify {
        type Public = Self
        type Personal = Self
        type Confidential = Self
        type Secret = Self

        public(self) -> Self::Public;
        personal(self) -> Self::Personal;
        confidential(self) -> Self::Confidential;
        secret(self) -> Self::Secret;
    }

    #[derive(Classify)]
    #[Class(public)]
    struct User {
        name: String,
        ssn: Ssn,
        password: Password,
        age: u8,
    }

    #[derive(Classify)]
    #[Class(pii, mask_ssn)]
    struct Ssn(String);

    #[derive(Classify)]
    #[Class(secret)]
    struct Password(String);

    fn mask_ssn(ssn: Ssn) -> Ssn {
        SSN(&ssn[0..12])
    }

* Good, easy to see what is used where
* Good, easy to implement
* Good, classification is specified on type
* Good, handles the recursive problem in "multiple types, macro for access methods"
* Meh, needs to be implemented for everything, should not be handled by blank implementation,
  if we miss implementing the trait on important types, they would be handled as public class.
* Meh, using different associated types could be annoying,
  because every type containing a type with different associated types
  has to have different versions for public, pii and secret.
* Bad, easy to forget to call one of the methods before serialization to api or other places

<!-- markdownlint-disable-file MD013 -->

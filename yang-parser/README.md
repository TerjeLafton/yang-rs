# YANG RS

A Rust-based YANG parser implemented using [pest](https://pest.rs/).

## Project Overview

YANG RS is an experimental implementation of a YANG parser in Rust. This project aims to create a general-purpose YANG parser that can serve as a foundation for other utilities and libraries for working with YANG and gNMI from Rust.

The parser implements the [RFC 7950](https://tools.ietf.org/html/rfc7950) specification.

### Potential Applications

Examples of tools that could be built on top of this parser include:
- YANG model explorers
- Libraries for generating code from YANG models

### Scope Limitations

This parser is not intended for validating YANG models. It operates under the assumption that the input is well-formed and follows the YANG grammar. Elements like `range`, which have clear definitions in the specification with rules about syntax and semantics, are simply parsed as strings.

While someone could implement a validator using the parser's output, it might be more efficient to enhance the parser's robustness in such cases.

The parser is primarily designed to work with models from the [YangModels/yang GitHub repository](https://github.com/YangModels/yang), which contains complete models already in use on devices.

## Implementation Approach

The initial implementation strategy is to create an intermediate representation of YANG models using Rust types, which can then be used for further processing.

### Example

Consider this simple YANG module:

```yang
module test-parser {
  namespace "urn:example:test-parser";
  prefix "test";

  organization "Test Organization";
  contact "test@example.com";
  description "A test YANG module with various constructs for parser testing";

  container system {
    description "System configuration";

    leaf hostname {
      type string {
        length "1..64";
        pattern '[a-zA-Z0-9\-\.]+';
      }
      mandatory true;
      description "The hostname of the system";
    }
  }
}
```

The parser would transform this into Rust code structured like:

```rust
Type {
    name: "string",
    length: Some("1..64"),
    pattern: Some("[a-zA-Z0-9\\-\\.]+"),
}

Leaf {
    name: "hostname",
    type_info: Type { ... }, // Reference to the Type above
    mandatory: true,
    description: Some("The hostname of the system"),
}

Container {
    name: "system",
    description: Some("System configuration"),
    leafs: {
        "hostname": Leaf { ... }, // Reference to the Leaf above
    },
}

Module {
    name: "test-parser",
    namespace: "urn:example:test-parser",
    prefix: "test",
    organization: Some("Test Organization"),
    contact: Some("test@example.com"),
    description: Some("A test YANG module with various constructs for parser testing"),
    containers: {
        "system": Container { ... }, // Reference to the Container above
    },
}
```

### Further Processing

This intermediate representation can be further processed to create user-friendly Rust structs. For example, the `System` container with a single leaf could be transformed into:

```rust
struct System {
    hostname: String,
}
```

Such abstractions would allow users to easily create instances of YANG modules in their Rust code.

## Complete example
<details>
<summary>This is a full example showing an example YANG module and how it looks after the initial pest parsing. </summary>

This module is a test YANG module with various constructs for parser testing.
```yang
module test-parser {
  namespace "urn:example:test-parser";
  prefix "test";

  organization "Test Organization";
  contact "test@example.com";
  description "A test YANG module with various constructs for parser testing";

  revision "2025-04-02" {
    description "Initial revision";
    reference "None";
  }

  // Import statements
  import ietf-yang-types {
    prefix "yang";
  }

  // Type definitions
  typedef percent {
    type uint8 {
      range "0..100";
    }
    description "Percentage value";
    units "percent";
  }

  typedef connection-status {
    type enumeration {
      enum "idle" {
        value 0;
        description "No connection";
      }
      enum "connecting" {
        value 1;
        description "Connection in progress";
      }
      enum "connected" {
        value 2;
        description "Connection established";
      }
      enum "disconnecting" {
        value 3;
        description "Disconnection in progress";
      }
    }
    description "Connection status values";
  }

  // Features
  feature advanced-metrics {
    description "Support for advanced metrics collection";
  }

  feature high-availability {
    description "High availability support";
  }

  // Identity definitions
  identity authentication-method {
    description "Base identity for authentication methods";
  }

  identity password-based {
    base authentication-method;
    description "Password-based authentication";
  }

  identity certificate-based {
    base authentication-method;
    description "Certificate-based authentication";
  }

  identity token-based {
    base authentication-method;
    description "Token-based authentication";
  }

  // Container with configuration data
  container system {
    description "System configuration";

    leaf hostname {
      type string {
        length "1..64";
        pattern '[a-zA-Z0-9\-\.]+';
      }
      mandatory true;
      description "The hostname of the system";
    }

    leaf domain {
      type string;
      default "example.com";
      description "The domain name";
    }

    leaf-list ntp-server {
      type string;
      ordered-by user;
      max-elements 5;
      description "List of NTP servers";
    }

    leaf enable-metrics {
      type boolean;
      default false;
      description "Whether to enable metrics collection";
    }

    leaf created-at {
      type yang:date-and-time;
      config false;
      description "When the system was created";
    }

    container location {
      presence "Indicates location configuration is present";
      description "Physical location of the system";

      leaf building {
        type string;
        description "Building name or number";
      }

      leaf floor {
        type int8 {
          range "-2..50";
        }
        description "Floor number";
      }

      leaf room {
        type string;
        description "Room identifier";
      }
    }

    container cpu {
      description "CPU information";

      leaf model {
        type string;
        config false;
        description "CPU model";
      }

      leaf cores {
        type uint8;
        config false;
        description "Number of CPU cores";
      }

      leaf utilization {
        if-feature "advanced-metrics";
        type percent;
        config false;
        description "Current CPU utilization";
      }
    }

    list user {
      key "username";
      unique "user-id";
      description "User accounts on the system";

      leaf username {
        type string {
          length "3..32";
          pattern '[a-zA-Z0-9_\-]+';
        }
        description "User login name";
      }

      leaf user-id {
        type uint16 {
          range "1000..65535";
        }
        mandatory true;
        description "Numeric user identifier";
      }

      leaf full-name {
        type string;
        description "User's full name";
      }

      leaf password {
        type string;
        description "User's password (hashed)";
      }

      leaf-list group {
        type string;
        description "Groups the user belongs to";
      }

      leaf auth-method {
        type identityref {
          base authentication-method;
        }
        default "password-based";
        description "Authentication method for this user";
      }
    }
  }

  // RPCs
  rpc restart-system {
    description "Restart the entire system";

    input {
      leaf delay {
        type uint16 {
          range "0..3600";
        }
        units "seconds";
        default 0;
        description "Delay before restart";
      }

      leaf force {
        type boolean;
        default false;
        description "Force restart without confirmation";
      }
    }

    output {
      leaf status {
        type enumeration {
          enum "success";
          enum "failure";
        }
        mandatory true;
        description "Result of operation";
      }

      leaf message {
        type string;
        description "Informational message about the result";
      }
    }
  }

  rpc generate-report {
    if-feature "advanced-metrics";
    description "Generate a system report";

    input {
      leaf format {
        type enumeration {
          enum "text";
          enum "xml";
          enum "json";
        }
        default "text";
        description "Output format of the report";
      }

      leaf-list sections {
        type enumeration {
          enum "system";
          enum "users";
          enum "network";
          enum "storage";
        }
        description "Sections to include in the report";
      }
    }

    output {
      leaf report-id {
        type string;
        mandatory true;
        description "Unique identifier for the generated report";
      }
    }
  }

  // Notifications
  notification system-restart {
    description "Indicates the system is restarting";

    leaf reason {
      type string;
      description "The reason for the restart";
    }

    leaf time {
      type yang:date-and-time;
      mandatory true;
      description "The time the restart occurred";
    }
  }

  notification resource-threshold-exceeded {
    if-feature "advanced-metrics";
    description "Indicates a resource threshold has been exceeded";

    leaf resource {
      type enumeration {
        enum "cpu";
        enum "memory";
        enum "disk";
        enum "network";
      }
      mandatory true;
      description "The resource that exceeded its threshold";
    }

    leaf current-value {
      type percent;
      mandatory true;
      description "Current resource utilization";
    }

    leaf threshold {
      type percent;
      mandatory true;
      description "Threshold that was exceeded";
    }
  }

  // Augmentation
  augment "/system" {
    if-feature "high-availability";

    container ha-config {
      description "High availability configuration";

      leaf mode {
        type enumeration {
          enum "active-active";
          enum "active-passive";
        }
        default "active-passive";
        description "HA operation mode";
      }

      leaf priority {
        type uint8 {
          range "1..255";
        }
        default 100;
        description "Priority in the HA cluster";
      }

      leaf-list peer {
        type string;
        min-elements 1;
        max-elements 10;
        description "HA peer addresses";
      }
    }
  }

  // Groups
  grouping address-fields {
    description "Common address fields";

    leaf street {
      type string;
      description "Street name";
    }

    leaf city {
      type string;
      description "City name";
    }

    leaf state {
      type string;
      description "State or province name";
    }

    leaf postal-code {
      type string;
      description "Postal code";
    }

    leaf country {
      type string;
      description "Country name";
    }
  }

  // Extension
  extension metadata {
    description "Extension to add metadata to definitions";
    argument "name";
  }

  // End of module
}
```

This is the resulting AST used for further parsing:
```
- module
  - string > unquoted_string: "test-parser"
  - namespace > string > double_quoted_string: "\"urn:example:test-parser\""
  - prefix > string > double_quoted_string: "\"test\""
  - organization > string > double_quoted_string: "\"Test Organization\""
  - contact > string > double_quoted_string: "\"test@example.com\""
  - description > string > double_quoted_string: "\"A test YANG module with various constructs for parser testing\""
  - revision
    - string > double_quoted_string: "\"2025-04-02\""
    - description > string > double_quoted_string: "\"Initial revision\""
    - reference > string > double_quoted_string: "\"None\""
  - import
    - string > unquoted_string: "ietf-yang-types"
    - prefix > string > double_quoted_string: "\"yang\""
  - typedef
    - string > unquoted_string: "percent"
    - type
      - string > unquoted_string: "uint8"
      - numberical_restriction > range > string > double_quoted_string: "\"0..100\""
    - description > string > double_quoted_string: "\"Percentage value\""
    - units > string > double_quoted_string: "\"percent\""
  - typedef
    - string > unquoted_string: "connection-status"
    - type
      - string > unquoted_string: "enumeration"
      - enum_specification
        - enum
          - string > double_quoted_string: "\"idle\""
          - value > integer: "0"
          - description > string > double_quoted_string: "\"No connection\""
        - enum
          - string > double_quoted_string: "\"connecting\""
          - value > integer: "1"
          - description > string > double_quoted_string: "\"Connection in progress\""
        - enum
          - string > double_quoted_string: "\"connected\""
          - value > integer: "2"
          - description > string > double_quoted_string: "\"Connection established\""
        - enum
          - string > double_quoted_string: "\"disconnecting\""
          - value > integer: "3"
          - description > string > double_quoted_string: "\"Disconnection in progress\""
    - description > string > double_quoted_string: "\"Connection status values\""
  - feature
    - string > unquoted_string: "advanced-metrics"
    - description > string > double_quoted_string: "\"Support for advanced metrics collection\""
  - feature
    - string > unquoted_string: "high-availability"
    - description > string > double_quoted_string: "\"High availability support\""
  - identity
    - string > unquoted_string: "authentication-method"
    - description > string > double_quoted_string: "\"Base identity for authentication methods\""
  - identity
    - string > unquoted_string: "password-based"
    - base > string > unquoted_string: "authentication-method"
    - description > string > double_quoted_string: "\"Password-based authentication\""
  - identity
    - string > unquoted_string: "certificate-based"
    - base > string > unquoted_string: "authentication-method"
    - description > string > double_quoted_string: "\"Certificate-based authentication\""
  - identity
    - string > unquoted_string: "token-based"
    - base > string > unquoted_string: "authentication-method"
    - description > string > double_quoted_string: "\"Token-based authentication\""
  - container
    - string > unquoted_string: "system"
    - description > string > double_quoted_string: "\"System configuration\""
    - leaf
      - string > unquoted_string: "hostname"
      - type
        - string > unquoted_string: "string"
        - string_restriction
          - length > string > double_quoted_string: "\"1..64\""
          - pattern > string > single_quoted_string: "'[a-zA-Z0-9\\-\\.]+'"
      - mandatory > boolean: "true"
      - description > string > double_quoted_string: "\"The hostname of the system\""
    - leaf
      - string > unquoted_string: "domain"
      - type > string > unquoted_string: "string"
      - default > string > double_quoted_string: "\"example.com\""
      - description > string > double_quoted_string: "\"The domain name\""
    - leaf_list
      - string > unquoted_string: "ntp-server"
      - type > string > unquoted_string: "string"
      - ordered_by > ordered_by_value: "user"
      - max_elements > max_elements_value > integer: "5"
      - description > string > double_quoted_string: "\"List of NTP servers\""
    - leaf
      - string > unquoted_string: "enable-metrics"
      - type > string > unquoted_string: "boolean"
      - default > string > unquoted_string: "false"
      - description > string > double_quoted_string: "\"Whether to enable metrics collection\""
    - leaf
      - string > unquoted_string: "created-at"
      - type > string > unquoted_string: "yang:date-and-time"
      - config > boolean: "false"
      - description > string > double_quoted_string: "\"When the system was created\""
    - container
      - string > unquoted_string: "location"
      - presence > string > double_quoted_string: "\"Indicates location configuration is present\""
      - description > string > double_quoted_string: "\"Physical location of the system\""
      - leaf
        - string > unquoted_string: "building"
        - type > string > unquoted_string: "string"
        - description > string > double_quoted_string: "\"Building name or number\""
      - leaf
        - string > unquoted_string: "floor"
        - type
          - string > unquoted_string: "int8"
          - numberical_restriction > range > string > double_quoted_string: "\"-2..50\""
        - description > string > double_quoted_string: "\"Floor number\""
      - leaf
        - string > unquoted_string: "room"
        - type > string > unquoted_string: "string"
        - description > string > double_quoted_string: "\"Room identifier\""
    - container
      - string > unquoted_string: "cpu"
      - description > string > double_quoted_string: "\"CPU information\""
      - leaf
        - string > unquoted_string: "model"
        - type > string > unquoted_string: "string"
        - config > boolean: "false"
        - description > string > double_quoted_string: "\"CPU model\""
      - leaf
        - string > unquoted_string: "cores"
        - type > string > unquoted_string: "uint8"
        - config > boolean: "false"
        - description > string > double_quoted_string: "\"Number of CPU cores\""
      - leaf
        - string > unquoted_string: "utilization"
        - if_feature > string > double_quoted_string: "\"advanced-metrics\""
        - type > string > unquoted_string: "percent"
        - config > boolean: "false"
        - description > string > double_quoted_string: "\"Current CPU utilization\""
    - list
      - string > unquoted_string: "user"
      - key > string > double_quoted_string: "\"username\""
      - unique > string > double_quoted_string: "\"user-id\""
      - description > string > double_quoted_string: "\"User accounts on the system\""
      - leaf
        - string > unquoted_string: "username"
        - type
          - string > unquoted_string: "string"
          - string_restriction
            - length > string > double_quoted_string: "\"3..32\""
            - pattern > string > single_quoted_string: "'[a-zA-Z0-9_\\-]+'"
        - description > string > double_quoted_string: "\"User login name\""
      - leaf
        - string > unquoted_string: "user-id"
        - type
          - string > unquoted_string: "uint16"
          - numberical_restriction > range > string > double_quoted_string: "\"1000..65535\""
        - mandatory > boolean: "true"
        - description > string > double_quoted_string: "\"Numeric user identifier\""
      - leaf
        - string > unquoted_string: "full-name"
        - type > string > unquoted_string: "string"
        - description > string > double_quoted_string: "\"User's full name\""
      - leaf
        - string > unquoted_string: "password"
        - type > string > unquoted_string: "string"
        - description > string > double_quoted_string: "\"User's password (hashed)\""
      - leaf_list
        - string > unquoted_string: "group"
        - type > string > unquoted_string: "string"
        - description > string > double_quoted_string: "\"Groups the user belongs to\""
      - leaf
        - string > unquoted_string: "auth-method"
        - type
          - string > unquoted_string: "identityref"
          - identityref_specification > base > string > unquoted_string: "authentication-method"
        - default > string > double_quoted_string: "\"password-based\""
        - description > string > double_quoted_string: "\"Authentication method for this user\""
  - rpc
    - string > unquoted_string: "restart-system"
    - description > string > double_quoted_string: "\"Restart the entire system\""
    - input
      - leaf
        - string > unquoted_string: "delay"
        - type
          - string > unquoted_string: "uint16"
          - numberical_restriction > range > string > double_quoted_string: "\"0..3600\""
        - units > string > double_quoted_string: "\"seconds\""
        - default > string > unquoted_string: "0"
        - description > string > double_quoted_string: "\"Delay before restart\""
      - leaf
        - string > unquoted_string: "force"
        - type > string > unquoted_string: "boolean"
        - default > string > unquoted_string: "false"
        - description > string > double_quoted_string: "\"Force restart without confirmation\""
    - output
      - leaf
        - string > unquoted_string: "status"
        - type
          - string > unquoted_string: "enumeration"
          - enum_specification
            - enum > string > double_quoted_string: "\"success\""
            - enum > string > double_quoted_string: "\"failure\""
        - mandatory > boolean: "true"
        - description > string > double_quoted_string: "\"Result of operation\""
      - leaf
        - string > unquoted_string: "message"
        - type > string > unquoted_string: "string"
        - description > string > double_quoted_string: "\"Informational message about the result\""
  - rpc
    - string > unquoted_string: "generate-report"
    - if_feature > string > double_quoted_string: "\"advanced-metrics\""
    - description > string > double_quoted_string: "\"Generate a system report\""
    - input
      - leaf
        - string > unquoted_string: "format"
        - type
          - string > unquoted_string: "enumeration"
          - enum_specification
            - enum > string > double_quoted_string: "\"text\""
            - enum > string > double_quoted_string: "\"xml\""
            - enum > string > double_quoted_string: "\"json\""
        - default > string > double_quoted_string: "\"text\""
        - description > string > double_quoted_string: "\"Output format of the report\""
      - leaf_list
        - string > unquoted_string: "sections"
        - type
          - string > unquoted_string: "enumeration"
          - enum_specification
            - enum > string > double_quoted_string: "\"system\""
            - enum > string > double_quoted_string: "\"users\""
            - enum > string > double_quoted_string: "\"network\""
            - enum > string > double_quoted_string: "\"storage\""
        - description > string > double_quoted_string: "\"Sections to include in the report\""
    - output > leaf
      - string > unquoted_string: "report-id"
      - type > string > unquoted_string: "string"
      - mandatory > boolean: "true"
      - description > string > double_quoted_string: "\"Unique identifier for the generated report\""
  - notification
    - string > unquoted_string: "system-restart"
    - description > string > double_quoted_string: "\"Indicates the system is restarting\""
    - leaf
      - string > unquoted_string: "reason"
      - type > string > unquoted_string: "string"
      - description > string > double_quoted_string: "\"The reason for the restart\""
    - leaf
      - string > unquoted_string: "time"
      - type > string > unquoted_string: "yang:date-and-time"
      - mandatory > boolean: "true"
      - description > string > double_quoted_string: "\"The time the restart occurred\""
  - notification
    - string > unquoted_string: "resource-threshold-exceeded"
    - if_feature > string > double_quoted_string: "\"advanced-metrics\""
    - description > string > double_quoted_string: "\"Indicates a resource threshold has been exceeded\""
    - leaf
      - string > unquoted_string: "resource"
      - type
        - string > unquoted_string: "enumeration"
        - enum_specification
          - enum > string > double_quoted_string: "\"cpu\""
          - enum > string > double_quoted_string: "\"memory\""
          - enum > string > double_quoted_string: "\"disk\""
          - enum > string > double_quoted_string: "\"network\""
      - mandatory > boolean: "true"
      - description > string > double_quoted_string: "\"The resource that exceeded its threshold\""
    - leaf
      - string > unquoted_string: "current-value"
      - type > string > unquoted_string: "percent"
      - mandatory > boolean: "true"
      - description > string > double_quoted_string: "\"Current resource utilization\""
    - leaf
      - string > unquoted_string: "threshold"
      - type > string > unquoted_string: "percent"
      - mandatory > boolean: "true"
      - description > string > double_quoted_string: "\"Threshold that was exceeded\""
  - augment
    - string > double_quoted_string: "\"/system\""
    - if_feature > string > double_quoted_string: "\"high-availability\""
    - container
      - string > unquoted_string: "ha-config"
      - description > string > double_quoted_string: "\"High availability configuration\""
      - leaf
        - string > unquoted_string: "mode"
        - type
          - string > unquoted_string: "enumeration"
          - enum_specification
            - enum > string > double_quoted_string: "\"active-active\""
            - enum > string > double_quoted_string: "\"active-passive\""
        - default > string > double_quoted_string: "\"active-passive\""
        - description > string > double_quoted_string: "\"HA operation mode\""
      - leaf
        - string > unquoted_string: "priority"
        - type
          - string > unquoted_string: "uint8"
          - numberical_restriction > range > string > double_quoted_string: "\"1..255\""
        - default > string > unquoted_string: "100"
        - description > string > double_quoted_string: "\"Priority in the HA cluster\""
      - leaf_list
        - string > unquoted_string: "peer"
        - type > string > unquoted_string: "string"
        - min_elements > integer: "1"
        - max_elements > max_elements_value > integer: "10"
        - description > string > double_quoted_string: "\"HA peer addresses\""
  - grouping
    - string > unquoted_string: "address-fields"
    - description > string > double_quoted_string: "\"Common address fields\""
    - leaf
      - string > unquoted_string: "street"
      - type > string > unquoted_string: "string"
      - description > string > double_quoted_string: "\"Street name\""
    - leaf
      - string > unquoted_string: "city"
      - type > string > unquoted_string: "string"
      - description > string > double_quoted_string: "\"City name\""
    - leaf
      - string > unquoted_string: "state"
      - type > string > unquoted_string: "string"
      - description > string > double_quoted_string: "\"State or province name\""
    - leaf
      - string > unquoted_string: "postal-code"
      - type > string > unquoted_string: "string"
      - description > string > double_quoted_string: "\"Postal code\""
    - leaf
      - string > unquoted_string: "country"
      - type > string > unquoted_string: "string"
      - description > string > double_quoted_string: "\"Country name\""
  - extension
    - string > unquoted_string: "metadata"
    - description > string > double_quoted_string: "\"Extension to add metadata to definitions\""
    - argument > string > double_quoted_string: "\"name\""
- EOI: ""
```
</details>

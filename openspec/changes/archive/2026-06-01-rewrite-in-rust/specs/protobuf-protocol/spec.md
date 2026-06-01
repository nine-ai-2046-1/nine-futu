## ADDED Requirements

### Requirement: Protobuf code generation from .proto files
The system SHALL compile all 77 Futu .proto files at build time using prost-build and generate Rust types for all message definitions.

#### Scenario: Build-time code generation
- **WHEN** the project is built with `cargo build`
- **THEN** all proto files are compiled and Rust types are generated in a `futu_proto` module

#### Scenario: Proto2 syntax support
- **WHEN** the .proto files use proto2 syntax
- **THEN** prost-build correctly generates Rust types with required/optional field support

### Requirement: Request packing
The system SHALL pack API requests into the correct Protobuf binary format with the appropriate proto_id.

#### Scenario: Pack InitConnect request
- **WHEN** the client needs to send an InitConnect request (proto_id=1001)
- **THEN** the request body is serialized to Protobuf binary and paired with proto_id=1001

#### Scenario: Pack Quote request
- **WHEN** the client needs to send a GetSecuritySnapshot request (proto_id=3203)
- **THEN** the request body is serialized to Protobuf binary and paired with proto_id=3203

### Requirement: Response unpacking
The system SHALL unpack API responses by parsing the 48-byte header and deserializing the Protobuf body.

#### Scenario: Unpack successful response
- **WHEN** the client receives a response with ret_type=0
- **THEN** the body is deserialized to the corresponding Rust type and returned

#### Scenario: Unpack error response
- **WHEN** the client receives a response with ret_type != 0
- **THEN** the error code and message are extracted and returned as an error

### Requirement: Serial number management
The system SHALL maintain an auto-incrementing serial number for request-response matching.

#### Scenario: Request-response matching
- **WHEN** multiple requests are in flight
- **THEN** each response is matched to its request using the serial_no from the header

#### Scenario: Serial number overflow
- **WHEN** the serial number reaches u32::MAX
- **THEN** the serial number wraps to 0

### Requirement: Push message detection
The system SHALL identify push messages by checking if the proto_id is in the All_PushId list.

#### Scenario: Push message routing
- **WHEN** a received message has proto_id in [1003, 2208, 2218, 3005, 3007, 3009, 3011, 3013, 3015, 3019]
- **THEN** the message is routed to the push data handler instead of the request-response matching

#### Scenario: Response message routing
- **WHEN** a received message has proto_id not in All_PushId
- **THEN** the message is matched to the pending request using serial_no

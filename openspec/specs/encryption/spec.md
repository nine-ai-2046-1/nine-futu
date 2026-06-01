## ADDED Requirements

### Requirement: AES encryption/decryption
The system SHALL support AES-ECB and AES-CBC encryption modes for packet body encryption.

#### Scenario: Encrypt request body
- **WHEN** encryption is enabled and the client sends a request
- **THEN** the request body is encrypted using the negotiated AES key before sending

#### Scenario: Decrypt response body
- **WHEN** encryption is enabled and the client receives a response
- **THEN** the response body is decrypted using the negotiated AES key before parsing

### Requirement: RSA key exchange
The system SHALL use RSA to exchange the AES encryption key during connection initialization.

#### Scenario: Key exchange
- **WHEN** the client connects with encryption enabled
- **THEN** the client sends an RSA-encrypted AES key to the gateway

#### Scenario: RSA with private key file
- **WHEN** the SysConfig.INIT_RSA_FILE is set
- **THEN** the client uses the private key for RSA decryption

### Requirement: Encryption configuration
The system SHALL allow enabling/disabling encryption per connection via the is_encrypt parameter.

#### Scenario: Encrypted connection
- **WHEN** the client creates a connection with is_encrypt=true
- **THEN** all messages on that connection are encrypted

#### Scenario: Unencrypted connection
- **WHEN** the client creates a connection with is_encrypt=false or None
- **THEN** all messages are sent in plaintext

### Requirement: PacketEncAlgo support
The system SHALL support the following encryption algorithms as specified in the Protocol:
- PacketEncAlgo_FTAES_ECB (0): Futu-modified AES-ECB
- PacketEncAlgo_None (-1): No encryption
- PacketEncAlgo_AES_ECB (1): Standard AES-ECB
- PacketEncAlgo_AES_CBC (2): Standard AES-CBC

#### Scenario: Algorithm selection
- **WHEN** the client negotiates encryption with the gateway
- **THEN** the agreed algorithm is used for all subsequent messages

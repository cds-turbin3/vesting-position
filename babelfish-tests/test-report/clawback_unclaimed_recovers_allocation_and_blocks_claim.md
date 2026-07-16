# clawback_unclaimed_recovers_allocation_and_blocks_claim

**Source:** [`tests/clawback.rs` L103](https://github.com/cds-turbin3/vesting-position/blob/3f946c506628d348826d858340b4ac335a62e967/babelfish-tests/tests/clawback.rs#L103)

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as system
    p0->>p1: ClawbackUnclaimed
    activate p1
    p1->>p2: createAccount
    activate p2
    p2-->>p1: ✓
    deactivate p2
    note over p1: 🚩 custom program error  0x177d
    p1-->>p0: ✗ 69028cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    6vT5(["6vT5…"]):::signer
    BXgR[("BXgR…")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    system["system"]:::program
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| 6vT5
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| CreatorATAMint
    Creator -->|signs| system
    6vT5 -->|signs| system
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    VestingPositions["VestingPositions"]:::program
    6vT5[("6vT5…")]:::state
    token["token"]:::program
    BXgR[("BXgR…")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    system -->|owns| Creator
    VestingPositions -->|owns| 6vT5
    token -->|owns| BXgR
    token -->|owns| CreatorATAMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Creator (69028cu)
└─ VestingPositions::ClawbackUnclaimed ✗ 69028cu
   └─ system::createAccount ✓
```

</details>

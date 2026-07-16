# clawback_unclaimed_succeeds_despite_buyers_zeroed_receipt

**Source:** [`tests/clawback.rs` L163](https://github.com/cds-turbin3/vesting-position/blob/c4f43acad0bb9f007622fb43bc19ddc3610c7688/babelfish-tests/tests/clawback.rs#L163)

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as token
    p0->>p1: ClawbackUnclaimed
    activate p1
    p1->>p2: transferChecked
    activate p2
    p2-->>p1: ✓ 105cu
    deactivate p2
    p1-->>p0: ✓ 92625cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    6vT5[("6vT5…")]:::state
    BXgR[("BXgR…")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    token["token"]:::program
    2rbE(["2rbE…"]):::signer
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| 6vT5
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| CreatorATAMint
    token -->|writes| BXgR
    token -->|writes| CreatorATAMint
    2rbE -->|signs| token
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

Creator (92625cu)
└─ VestingPositions::ClawbackUnclaimed ✓ 92625cu
   └─ token::transferChecked ✓ 105cu
```

</details>

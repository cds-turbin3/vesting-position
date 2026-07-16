# clawback_unclaimed_before_grace_fails

**Source:** [`tests/clawback.rs` L196](https://github.com/cds-turbin3/vesting-position/blob/3f946c506628d348826d858340b4ac335a62e967/babelfish-tests/tests/clawback.rs#L196)

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    p0->>p1: CloseCampaign
    activate p1
    note over p1: 🚩 custom program error  0x1787
    p1-->>p0: ✗ 8052cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    2rbE[("2rbE…")]:::state
    BXgR[("BXgR…")]:::state
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| 2rbE
    VestingPositions -->|writes| BXgR
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    VestingPositions["VestingPositions"]:::program
    2rbE[("2rbE…")]:::state
    token["token"]:::program
    BXgR[("BXgR…")]:::state
    system -->|owns| Creator
    VestingPositions -->|owns| 2rbE
    token -->|owns| BXgR
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Creator (8052cu)
└─ VestingPositions::CloseCampaign ✗ 8052cu
```

</details>

# close_campaign_succeeds_when_vault_empty

**Source:** [`tests/clawback.rs` L222](https://github.com/cds-turbin3/vesting-position/blob/3f946c506628d348826d858340b4ac335a62e967/babelfish-tests/tests/clawback.rs#L222)

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as token
    p0->>p1: CloseCampaign
    activate p1
    p1->>p2: closeAccount
    activate p2
    p2-->>p1: ✓ 118cu
    deactivate p2
    p1-->>p0: ✓ 9878cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    2rbE(["2rbE…"]):::signer
    BXgR[("BXgR…")]:::state
    token["token"]:::program
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| 2rbE
    VestingPositions -->|writes| BXgR
    token -->|writes| BXgR
    token -->|writes| Creator
    2rbE -->|signs| token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    2rbE[("2rbE…")]:::state
    BXgR[("BXgR…")]:::state
    system -->|owns| Creator
    system -->|owns| 2rbE
    system -->|owns| BXgR
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Creator (9878cu)
└─ VestingPositions::CloseCampaign ✓ 9878cu
   └─ token::closeAccount ✓ 118cu
```

</details>

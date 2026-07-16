# scenario_2_two_users_transfer_and_bob_claims_both

**Source:** [`tests/claim.rs` L61](https://github.com/cds-turbin3/vesting-position/blob/3f946c506628d348826d858340b4ac335a62e967/babelfish-tests/tests/claim.rs#L61)

```mermaid
sequenceDiagram
    participant p0 as H87x…
    participant p1 as VestingPositions
    p0->>p1: Claim
    activate p1
    p1-->>p0: ✓ 32629cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    H87x(["H87x…"]):::signer
    Collection[("Collection")]:::state
    BXgR[("BXgR…")]:::state
    7NWR[("7NWR…")]:::state
    3oY2[("3oY2…")]:::state
    6vT5[("6vT5…")]:::state
    H87x -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| 7NWR
    VestingPositions -->|writes| 3oY2
    VestingPositions -->|writes| 6vT5
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    H87x[("H87x…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    BXgR[("BXgR…")]:::state
    7NWR[("7NWR…")]:::state
    3oY2[("3oY2…")]:::state
    VestingPositions["VestingPositions"]:::program
    6vT5[("6vT5…")]:::state
    system -->|owns| H87x
    CoRE -->|owns| Collection
    token -->|owns| BXgR
    token -->|owns| 7NWR
    CoRE -->|owns| 3oY2
    VestingPositions -->|owns| 6vT5
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

H87x… (32629cu)
└─ VestingPositions::Claim ✓ 32629cu
```

</details>

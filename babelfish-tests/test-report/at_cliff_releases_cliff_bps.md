# at_cliff_releases_cliff_bps

**Source:** [`tests/vesting_schedule.rs` L47](https://github.com/cds-turbin3/vesting-position/blob/c4f43acad0bb9f007622fb43bc19ddc3610c7688/babelfish-tests/tests/vesting_schedule.rs#L47)

```mermaid
sequenceDiagram
    participant p0 as 4wQQ…
    participant p1 as Comp…
    participant p2 as VestingPositions
    participant p3 as splAssociatedTokenAccount
    participant p4 as token
    participant p5 as system
    participant p6 as CoRE…
    p0->>p1: ?
    activate p1
    p1-->>p0: ✓
    deactivate p1
    p0->>p2: Claim
    activate p2
    p2->>p3: create
    activate p3
    p3->>p4: getAccountDataSize
    activate p4
    p4-->>p3: ✓ 183cu
    deactivate p4
    p3->>p5: createAccount
    activate p5
    p5-->>p3: ✓
    deactivate p5
    p3->>p4: initializeImmutableOwner
    activate p4
    p4-->>p3: ✓ 38cu
    deactivate p4
    p3->>p4: initializeAccount3
    activate p4
    p4-->>p3: ✓ 235cu
    deactivate p4
    p3-->>p2: ✓ 13416cu
    deactivate p3
    p2->>p5: createAccount
    activate p5
    p5-->>p2: ✓
    deactivate p5
    p2->>p6: CreateV2
    activate p6
    p6->>p5: createAccount
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6-->>p2: ✓ 29646cu
    deactivate p6
    p2->>p4: transferChecked
    activate p4
    p4-->>p2: ✓ 105cu
    deactivate p4
    p2-->>p0: ✓ 163621cu
    deactivate p2
```

```mermaid
flowchart LR
    Comp["Comp…"]:::program
    VestingPositions["VestingPositions"]:::program
    4wQQ(["4wQQ…"]):::signer
    Collection[("Collection")]:::state
    BXgR[("BXgR…")]:::state
    45PB(["45PB…"]):::signer
    4QVs(["4QVs…"]):::signer
    7kmN(["7kmN…"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    CoRE["CoRE…"]:::program
    vGh5(["vGh5…"]):::signer
    2rbE(["2rbE…"]):::signer
    4wQQ -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| 45PB
    VestingPositions -->|writes| 4QVs
    VestingPositions -->|writes| 7kmN
    4wQQ -->|signs| splAssociatedTokenAccount
    splAssociatedTokenAccount -->|writes| 45PB
    4wQQ -->|signs| system
    45PB -->|signs| system
    token -->|writes| 45PB
    7kmN -->|signs| system
    4QVs -->|signs| CoRE
    CoRE -->|writes| Collection
    vGh5 -->|signs| CoRE
    4wQQ -->|signs| CoRE
    4QVs -->|signs| system
    system -->|writes| 4QVs
    token -->|writes| BXgR
    2rbE -->|signs| token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    4wQQ[("4wQQ…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    BXgR[("BXgR…")]:::state
    45PB[("45PB…")]:::state
    4QVs[("4QVs…")]:::state
    VestingPositions["VestingPositions"]:::program
    7kmN[("7kmN…")]:::state
    system -->|owns| 4wQQ
    CoRE -->|owns| Collection
    token -->|owns| BXgR
    token -->|owns| 45PB
    CoRE -->|owns| 4QVs
    VestingPositions -->|owns| 7kmN
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

4wQQ… (163771cu)
├─ Comp…::? ✓
└─ VestingPositions::Claim ✓ 163621cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ system::createAccount ✓
   ├─ CoRE…::CreateV2 ✓ 29646cu
   │  ├─ system::createAccount ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  └─ system::transferSol ✓
   └─ token::transferChecked ✓ 105cu
```

</details>

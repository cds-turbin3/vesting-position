# scenario_8_wrong_asset_subsequent_claim_fails

**Source:** [`tests/claim.rs` L216](https://github.com/cds-turbin3/vesting-position/blob/a73490430ef5c43423e4370c61364fdf961e7484/babelfish-tests/tests/claim.rs#L216)

Over 1 day: 2 moments.

<details>
<summary>Cast</summary>

| name | address |
| --- | --- |
| Collection | 9HbgSnRdBeYKzrjr9DYtcT6oKYrcTVB8NWUoU7JVKuWW |
| Creator | 2ZBYuwtWiRzk7CwiCYTv5MQhQHDEaN4B8xhw4L7L3RY5 |
| Mint | 4Kr8ypueV83MddH54fZXLkKFKRd7eWFcejQ8HtynfJRk |
| VestingPositions | 7DkU9TQhcN87f2djZDd2MjjPZoXLfnZZj8HhybeZswX1 |
| campaign(Collection) | G4GWLHr4aHZoxWra82eRc111wRJ9aDiXsSuk3bWoys2G |
| campaignAta(campaign(Collection), token, Mint) | 3kKwPKo9z6XQWrZxuo75c36vm9ChMgGDAMBV7cFEhjSn |
| creatorAta(Creator, token, Mint) | AvzkuSUEzjhXyboXRyfQcsjzKLBqeP9FeoExRBWkXjdJ |
| updateAuthority(Collection) | CYBwE6G2RjrsFYbwy5pUVVDVL5UR5g5VaRcWBPbzby1p |
| userAta(ErV6…, token, Mint) | A8Eu8FarK5CxvYkBoYeJ9bsoA4mFZFvH9dqgH56TZQnf |
| 1115… | 11157t3sqMV725NVRLrVQbAu98Jjfk1uCKehJnXXQs |
| BPGT… | BPGTo3tYBXeY4XK6hNXkHKXP4DvNd6dgiU66GapJTtFj |
| ErV6… | ErV63ApqLgh1Je5PdiVj6kzwkKJmLjKV41QoN9U4BNag |

</details>

### Timeline

| time | Vault balance | Δ | Creator balance | Δ | Mallory balance | comment |
| --- | --- | --- | --- | --- | --- | --- |
| T0 | 10,000,000,000,000 | +10,000,000,000,000 | 0 |  | — | Initialize (day 0) |
| T1 | 10,000,000,000,000 |  | 0 |  | — | Claim (day 1) |

### T0: Initialize (day 0)

| observation | before | after |
| --- | --- | --- |
| Vault balance | — | 10,000,000,000,000 |
| Creator balance | — | 0 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as system
    participant p3 as splAssociatedTokenAccount
    participant p4 as token
    participant p5 as mplCoreProgram
    p0->>p1: Initialize
    activate p1
    p1->>p2: createAccount
    activate p2
    p2-->>p1: ✓
    deactivate p2
    p1->>p3: create
    activate p3
    p3->>p4: getAccountDataSize
    activate p4
    p4-->>p3: ✓ 183cu
    deactivate p4
    p3->>p2: createAccount
    activate p2
    p2-->>p3: ✓
    deactivate p2
    p3->>p4: initializeImmutableOwner
    activate p4
    p4-->>p3: ✓ 38cu
    deactivate p4
    p3->>p4: initializeAccount3
    activate p4
    p4-->>p3: ✓ 235cu
    deactivate p4
    p3-->>p1: ✓ 18017cu
    deactivate p3
    p1->>p5: CreateCollectionV2
    activate p5
    p5->>p2: createAccount
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5->>p2: transferSol
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5->>p2: transferSol
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5->>p2: transferSol
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5-->>p1: ✓ 19976cu
    deactivate p5
    p1->>p4: transferChecked
    activate p4
    p4-->>p1: ✓ 105cu
    deactivate p4
    p1-->>p0: ✓ 89713cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    Collection(["Collection"]):::signer
    Mint[("Mint")]:::state
    creatorAtaCreatortokenMint[("creatorAta(Creator, token, Mint)")]:::state
    campaignCollection(["campaign(Collection)"]):::signer
    campaignAtacampaignCollectiontokenMint(["campaignAta(campaign(Collection), token, Mint)"]):::signer
    system["system"]:::program
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    mplCoreProgram["mplCoreProgram"]:::program
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> Mint
    VestingPositions --> creatorAtaCreatortokenMint
    VestingPositions --> campaignCollection
    VestingPositions --> campaignAtacampaignCollectiontokenMint
    Creator --> system
    campaignCollection --> system
    Creator --> splAssociatedTokenAccount
    splAssociatedTokenAccount --> campaignAtacampaignCollectiontokenMint
    campaignAtacampaignCollectiontokenMint --> system
    token --> campaignAtacampaignCollectiontokenMint
    Collection --> mplCoreProgram
    Creator --> mplCoreProgram
    Collection --> system
    system --> Collection
    token --> creatorAtaCreatortokenMint
    Creator --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,6,7,8,10,12,13,14,17 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,9,11,15,16 stroke:#b87800,stroke-width:2px
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    mplCoreProgram["mplCoreProgram"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    Mint[("Mint")]:::state
    creatorAtaCreatortokenMint[("creatorAta(Creator, token, Mint)")]:::state
    VestingPositions["VestingPositions"]:::program
    campaignCollection[("campaign(Collection)")]:::state
    campaignAtacampaignCollectiontokenMint[("campaignAta(campaign(Collection), token, Mint)")]:::state
    system -->|owns| Creator
    mplCoreProgram --> Collection
    token --> Mint
    token --> creatorAtaCreatortokenMint
    VestingPositions --> campaignCollection
    token --> campaignAtacampaignCollectiontokenMint
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,1,2,3,4,5 stroke:#456ea1,stroke-width:2px
```

### Tree

```

Creator (89713cu)
└─ VestingPositions::Initialize ✓ 89713cu
   ├─ system::createAccount ✓
   ├─ splAssociatedTokenAccount::create ✓ 18017cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ mplCoreProgram::CreateCollectionV2 ✓ 19976cu
   │  ├─ system::createAccount ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  └─ system::transferSol ✓
   └─ token::transferChecked ✓ 105cu
```

</details>

*1 day pass.*

### T1: Claim (day 1)

- [x] refused: InvalidAsset — InstructionError(0, Custom(6015))

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as ErV6…
    participant p1 as VestingPositions
    participant p2 as splAssociatedTokenAccount
    participant p3 as token
    participant p4 as system
    p0->>p1: Claim
    activate p1
    p1->>p2: create
    activate p2
    p2->>p3: getAccountDataSize
    activate p3
    p3-->>p2: ✓ 183cu
    deactivate p3
    p2->>p4: createAccount
    activate p4
    p4-->>p2: ✓
    deactivate p4
    p2->>p3: initializeImmutableOwner
    activate p3
    p3-->>p2: ✓ 38cu
    deactivate p3
    p2->>p3: initializeAccount3
    activate p3
    p3-->>p2: ✓ 235cu
    deactivate p3
    p2-->>p1: ✓ 13416cu
    deactivate p2
    p1->>p4: createAccount
    activate p4
    p4-->>p1: ✓
    deactivate p4
    note over p1: 🚩 custom program error  0x177f
    p1-->>p0: ✗ 45719cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    ErV6(["ErV6…"]):::signer
    Collection[("Collection")]:::state
    campaignAtacampaignCollectiontokenMint[("campaignAta(campaign(Collection), token, Mint)")]:::state
    userAtaErV6tokenMint(["userAta(ErV6…, token, Mint)"]):::signer
    n1115[("1115…")]:::state
    BPGT(["BPGT…"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    ErV6 -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> campaignAtacampaignCollectiontokenMint
    VestingPositions --> userAtaErV6tokenMint
    VestingPositions --> n1115
    VestingPositions --> BPGT
    ErV6 --> splAssociatedTokenAccount
    splAssociatedTokenAccount --> userAtaErV6tokenMint
    ErV6 --> system
    userAtaErV6tokenMint --> system
    token --> userAtaErV6tokenMint
    BPGT --> system
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,6,8,9,11 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,7,10 stroke:#b87800,stroke-width:2px
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    ErV6[("ErV6…")]:::state
    mplCoreProgram["mplCoreProgram"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtacampaignCollectiontokenMint[("campaignAta(campaign(Collection), token, Mint)")]:::state
    userAtaErV6tokenMint[("userAta(ErV6…, token, Mint)")]:::state
    n1115[("1115…")]:::state
    VestingPositions["VestingPositions"]:::program
    BPGT[("BPGT…")]:::state
    system -->|owns| ErV6
    mplCoreProgram --> Collection
    token --> campaignAtacampaignCollectiontokenMint
    token --> userAtaErV6tokenMint
    system --> n1115
    VestingPositions --> BPGT
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,1,2,3,4,5 stroke:#456ea1,stroke-width:2px
```

### Tree

```

ErV6… (45719cu)
└─ VestingPositions::Claim ✗ 45719cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   └─ system::createAccount ✓
```

</details>

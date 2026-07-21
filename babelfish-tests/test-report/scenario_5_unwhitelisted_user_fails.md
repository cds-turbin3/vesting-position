# scenario_5_unwhitelisted_user_fails

**Source:** [`tests/claim.rs` L157](https://github.com/cds-turbin3/vesting-position/blob/932c23856044d9b06abefcbda648838176349fe4/babelfish-tests/tests/claim.rs#L157)

Over 1 day: 2 moments.

<details>
<summary>Cast</summary>

| name | address |
| --- | --- |
| Collection | 9HbgSnRdBeYKzrjr9DYtcT6oKYrcTVB8NWUoU7JVKuWW |
| Creator | 2ZBYuwtWiRzk7CwiCYTv5MQhQHDEaN4B8xhw4L7L3RY5 |
| Mallory | ErV63ApqLgh1Je5PdiVj6kzwkKJmLjKV41QoN9U4BNag |
| Mallory's position NFT | yP9LAuj4pAEf6XGLa2Dm54zFgdQbDHh4TebTuR4tuyH |
| Mallory's receipt | BPGTo3tYBXeY4XK6hNXkHKXP4DvNd6dgiU66GapJTtFj |
| Mint | 4Kr8ypueV83MddH54fZXLkKFKRd7eWFcejQ8HtynfJRk |
| Vesting campaign | G4GWLHr4aHZoxWra82eRc111wRJ9aDiXsSuk3bWoys2G |
| VestingPositions | 7DkU9TQhcN87f2djZDd2MjjPZoXLfnZZj8HhybeZswX1 |
| campaignAta(Vesting campaign, token, Mint) | 3kKwPKo9z6XQWrZxuo75c36vm9ChMgGDAMBV7cFEhjSn |
| creatorAta(Creator, token, Mint) | AvzkuSUEzjhXyboXRyfQcsjzKLBqeP9FeoExRBWkXjdJ |
| updateAuthority(Collection) | CYBwE6G2RjrsFYbwy5pUVVDVL5UR5g5VaRcWBPbzby1p |

</details>

### Timeline

| time | Vault balance | Δ | Creator balance | Δ | Mallory balance | comment |
| --- | --- | --- | --- | --- | --- | --- |
| T0 | 10,000,000,000,000 | +10,000,000,000,000 | 0 |  | — | Initialize (day 0) |
| T1 | 10,000,000,000,000 |  | 0 |  | — | FirstClaim (day 1) |

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
    Vestingcampaign(["Vesting campaign"]):::signer
    campaignAtaVestingcampaigntokenMint(["campaignAta(Vesting campaign, token, Mint)"]):::signer
    system["system"]:::program
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    mplCoreProgram["mplCoreProgram"]:::program
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> Mint
    VestingPositions --> creatorAtaCreatortokenMint
    VestingPositions --> Vestingcampaign
    VestingPositions --> campaignAtaVestingcampaigntokenMint
    Creator --> system
    Vestingcampaign --> system
    Creator --> splAssociatedTokenAccount
    splAssociatedTokenAccount --> campaignAtaVestingcampaigntokenMint
    campaignAtaVestingcampaigntokenMint --> system
    token --> campaignAtaVestingcampaigntokenMint
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
    Vestingcampaign[("Vesting campaign")]:::state
    campaignAtaVestingcampaigntokenMint[("campaignAta(Vesting campaign, token, Mint)")]:::state
    system -->|owns| Creator
    mplCoreProgram --> Collection
    token --> Mint
    token --> creatorAtaCreatortokenMint
    VestingPositions --> Vestingcampaign
    token --> campaignAtaVestingcampaigntokenMint
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

### T1: FirstClaim (day 1)

- [x] refused: InvalidProofs — InstructionError(0, Custom(6013))

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Mallory
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
    note over p1: 🚩 custom program error  0x177d
    p1-->>p0: ✗ 92417cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Mallory(["Mallory"]):::signer
    Collection[("Collection")]:::state
    campaignAtaVestingcampaigntokenMint[("campaignAta(Vesting campaign, token, Mint)")]:::state
    MalloryATAMint(["Mallory/ATA(Mint)"]):::signer
    MalloryspositionNFT[("Mallory's position NFT")]:::state
    Mallorysreceipt(["Mallory's receipt"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    Mallory -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> campaignAtaVestingcampaigntokenMint
    VestingPositions --> MalloryATAMint
    VestingPositions --> MalloryspositionNFT
    VestingPositions --> Mallorysreceipt
    Mallory --> splAssociatedTokenAccount
    splAssociatedTokenAccount --> MalloryATAMint
    Mallory --> system
    MalloryATAMint --> system
    token --> MalloryATAMint
    Mallorysreceipt --> system
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
    Mallory[("Mallory")]:::state
    mplCoreProgram["mplCoreProgram"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtaVestingcampaigntokenMint[("campaignAta(Vesting campaign, token, Mint)")]:::state
    MalloryATAMint[("Mallory/ATA(Mint)")]:::state
    MalloryspositionNFT[("Mallory's position NFT")]:::state
    VestingPositions["VestingPositions"]:::program
    Mallorysreceipt[("Mallory's receipt")]:::state
    system -->|owns| Mallory
    mplCoreProgram --> Collection
    token --> campaignAtaVestingcampaigntokenMint
    token --> MalloryATAMint
    system --> MalloryspositionNFT
    VestingPositions --> Mallorysreceipt
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,1,2,3,4,5 stroke:#456ea1,stroke-width:2px
```

### Tree

```

Mallory (92417cu)
└─ VestingPositions::Claim ✗ 92417cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   └─ system::createAccount ✓
```

</details>

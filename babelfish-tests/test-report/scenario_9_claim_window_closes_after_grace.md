# scenario_9_claim_window_closes_after_grace

**Source:** [`tests/claim.rs` L237](https://github.com/cds-turbin3/vesting-position/blob/414788a7eaf024c312a712e22c69b20da7f9697d/babelfish-tests/tests/claim.rs#L237)

Over 38 days: 4 moments.

<details>
<summary>Cast</summary>

| name | address |
| --- | --- |
| Alice | 4wQQJM9LNuhinieNAqmHuPCm8LXDTVfhx84P32nAVE9P |
| Alice's position NFT | 6hhAjXPGt41Y6oPH6mATnAqMECdaHrKaAGbE4SFuARXK |
| Alice's receipt | 5yfASCcX25V4fzBgdfixtXgS2JrvpWdwX27JQXpVyuHB |
| Bob | H87xi4CUqrUPXzppV3jotTmre6DyR5pCaMk5bKQQBFTg |
| Bob's position NFT | 5fjLQR7cXnkBzwud4xAzHaxWKbTiQMJyrNe72RXUtieC |
| Bob's receipt | C49GR57waDzt1nFUvxKPDs1HVAKF694inGJeB2KGRPM9 |
| Collection | 9HbgSnRdBeYKzrjr9DYtcT6oKYrcTVB8NWUoU7JVKuWW |
| Creator | 2ZBYuwtWiRzk7CwiCYTv5MQhQHDEaN4B8xhw4L7L3RY5 |
| Mint | 4Kr8ypueV83MddH54fZXLkKFKRd7eWFcejQ8HtynfJRk |
| Vesting campaign | G4GWLHr4aHZoxWra82eRc111wRJ9aDiXsSuk3bWoys2G |
| VestingPositions | 7DkU9TQhcN87f2djZDd2MjjPZoXLfnZZj8HhybeZswX1 |
| campaignAta(Vesting campaign, token, Mint) | 3kKwPKo9z6XQWrZxuo75c36vm9ChMgGDAMBV7cFEhjSn |
| creatorAta(Creator, token, Mint) | AvzkuSUEzjhXyboXRyfQcsjzKLBqeP9FeoExRBWkXjdJ |
| updateAuthority(Collection) | CYBwE6G2RjrsFYbwy5pUVVDVL5UR5g5VaRcWBPbzby1p |
| userAta(Alice, token, Mint) | C7PguAKs34J4WkXRRp762bPFhzhmFBYKdLuHQYBrVmAW |

</details>

### Timeline

| time | Vault balance | Δ | Creator balance | Δ | Alice balance | Δ | Bob balance | comment |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T0 | 10,000,000,000,000 | +10,000,000,000,000 | 0 |  | — |  | — | Initialize (day 0) |
| T1 | 10,000,000,000,000 |  | 0 |  | — |  | — | FirstClaim (day 1) |
| T2 | 9,000,000,000,000 | -1,000,000,000,000 | 0 |  | 1,000,000,000,000 | +1,000,000,000,000 | — | Claim (day 37) |
| T3 | 9,000,000,000,000 |  | 0 |  | 1,000,000,000,000 |  | — | FirstClaim (day 38) |

### Chart

```mermaid
xychart-beta
    x-axis ["d0", "d1", "d37", "d38"]
    line [10000000000000, 10000000000000, 9000000000000, 9000000000000]
    line [0, 0, 0, 0]
    line [0, 0, 1000000000000, 1000000000000]
```

*series: Vault balance, Creator balance, Alice balance*

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

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as computeBudget
    participant p2 as VestingPositions
    participant p3 as splAssociatedTokenAccount
    participant p4 as token
    participant p5 as system
    participant p6 as mplCoreProgram
    p0->>p1: setComputeUnitLimit
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
    p6-->>p2: ✓ 29413cu
    deactivate p6
    p2-->>p0: ✓ 164909cu
    deactivate p2
```

### Authority

```mermaid
flowchart LR
    computeBudget["computeBudget"]:::program
    VestingPositions["VestingPositions"]:::program
    Alice(["Alice"]):::signer
    Collection[("Collection")]:::state
    campaignAtaVestingcampaigntokenMint[("campaignAta(Vesting campaign, token, Mint)")]:::state
    AliceATAMint(["Alice/ATA(Mint)"]):::signer
    AlicespositionNFT(["Alice's position NFT"]):::signer
    Alicesreceipt(["Alice's receipt"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    mplCoreProgram["mplCoreProgram"]:::program
    updateAuthorityCollection(["updateAuthority(Collection)"]):::signer
    Alice -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> campaignAtaVestingcampaigntokenMint
    VestingPositions --> AliceATAMint
    VestingPositions --> AlicespositionNFT
    VestingPositions --> Alicesreceipt
    Alice --> splAssociatedTokenAccount
    splAssociatedTokenAccount --> AliceATAMint
    Alice --> system
    AliceATAMint --> system
    token --> AliceATAMint
    Alicesreceipt --> system
    AlicespositionNFT --> mplCoreProgram
    mplCoreProgram --> Collection
    updateAuthorityCollection --> mplCoreProgram
    Alice --> mplCoreProgram
    AlicespositionNFT --> system
    system --> AlicespositionNFT
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,6,8,9,11,12,14,15,16 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,7,10,13,17 stroke:#b87800,stroke-width:2px
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    Alice[("Alice")]:::state
    mplCoreProgram["mplCoreProgram"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtaVestingcampaigntokenMint[("campaignAta(Vesting campaign, token, Mint)")]:::state
    AliceATAMint[("Alice/ATA(Mint)")]:::state
    AlicespositionNFT[("Alice's position NFT")]:::state
    VestingPositions["VestingPositions"]:::program
    Alicesreceipt[("Alice's receipt")]:::state
    system -->|owns| Alice
    mplCoreProgram --> Collection
    token --> campaignAtaVestingcampaigntokenMint
    token --> AliceATAMint
    mplCoreProgram --> AlicespositionNFT
    VestingPositions --> Alicesreceipt
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,1,2,3,4,5 stroke:#456ea1,stroke-width:2px
```

### Tree

```

Alice (165059cu)
├─ computeBudget::setComputeUnitLimit ✓
└─ VestingPositions::Claim ✓ 164909cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ system::createAccount ✓
   └─ mplCoreProgram::CreateV2 ✓ 29413cu
      ├─ system::createAccount ✓
      ├─ system::transferSol ✓
      ├─ system::transferSol ✓
      ├─ system::transferSol ✓
      └─ system::transferSol ✓
```

</details>

*3196799 seconds pass.*

### T2: Claim (day 37)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 10,000,000,000,000 | 9,000,000,000,000 |
| Alice balance | — | 1,000,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as system
    participant p4 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2->>p3: transferSol
    activate p3
    p3-->>p2: ✓
    deactivate p3
    p2-->>p1: ✓ 22492cu
    deactivate p2
    p1->>p4: transferChecked
    activate p4
    p4-->>p1: ✓ 105cu
    deactivate p4
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 13574cu
    deactivate p2
    p1-->>p0: ✓ 95977cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Alice(["Alice"]):::signer
    Collection[("Collection")]:::state
    campaignAtaVestingcampaigntokenMint[("campaignAta(Vesting campaign, token, Mint)")]:::state
    userAtaAlicetokenMint[("userAta(Alice, token, Mint)")]:::state
    AlicespositionNFT[("Alice's position NFT")]:::state
    Alicesreceipt[("Alice's receipt")]:::state
    mplCoreProgram["mplCoreProgram"]:::program
    updateAuthorityCollection(["updateAuthority(Collection)"]):::signer
    system["system"]:::program
    token["token"]:::program
    Vestingcampaign(["Vesting campaign"]):::signer
    Alice -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> campaignAtaVestingcampaigntokenMint
    VestingPositions --> userAtaAlicetokenMint
    VestingPositions --> AlicespositionNFT
    VestingPositions --> Alicesreceipt
    mplCoreProgram --> AlicespositionNFT
    mplCoreProgram --> Collection
    Alice --> mplCoreProgram
    updateAuthorityCollection --> mplCoreProgram
    Alice --> system
    system --> AlicespositionNFT
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,10,14 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,11,12,13 stroke:#b87800,stroke-width:2px
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    Alice[("Alice")]:::state
    mplCoreProgram["mplCoreProgram"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtaVestingcampaigntokenMint[("campaignAta(Vesting campaign, token, Mint)")]:::state
    userAtaAlicetokenMint[("userAta(Alice, token, Mint)")]:::state
    AlicespositionNFT[("Alice's position NFT")]:::state
    VestingPositions["VestingPositions"]:::program
    Alicesreceipt[("Alice's receipt")]:::state
    system -->|owns| Alice
    mplCoreProgram --> Collection
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    mplCoreProgram --> AlicespositionNFT
    VestingPositions --> Alicesreceipt
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,1,2,3,4,5 stroke:#456ea1,stroke-width:2px
```

### Tree

```

Alice (95977cu)
└─ VestingPositions::Claim ✓ 95977cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 22492cu
   │  └─ system::transferSol ✓
   ├─ token::transferChecked ✓ 105cu
   └─ mplCoreProgram::UpdatePlugin ✓ 13574cu
```

</details>

*1 second pass.*

### T3: FirstClaim (day 38)

- [x] refused: ClaimWindowClosed — InstructionError(0, Custom(6025))

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Bob
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
    note over p1: 🚩 custom program error  0x1789
    p1-->>p0: ✗ 61744cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Bob(["Bob"]):::signer
    Collection[("Collection")]:::state
    campaignAtaVestingcampaigntokenMint[("campaignAta(Vesting campaign, token, Mint)")]:::state
    BobATAMint(["Bob/ATA(Mint)"]):::signer
    BobspositionNFT[("Bob's position NFT")]:::state
    Bobsreceipt(["Bob's receipt"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    Bob -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> campaignAtaVestingcampaigntokenMint
    VestingPositions --> BobATAMint
    VestingPositions --> BobspositionNFT
    VestingPositions --> Bobsreceipt
    Bob --> splAssociatedTokenAccount
    splAssociatedTokenAccount --> BobATAMint
    Bob --> system
    BobATAMint --> system
    token --> BobATAMint
    Bobsreceipt --> system
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
    Bob[("Bob")]:::state
    mplCoreProgram["mplCoreProgram"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtaVestingcampaigntokenMint[("campaignAta(Vesting campaign, token, Mint)")]:::state
    BobATAMint[("Bob/ATA(Mint)")]:::state
    BobspositionNFT[("Bob's position NFT")]:::state
    VestingPositions["VestingPositions"]:::program
    Bobsreceipt[("Bob's receipt")]:::state
    system -->|owns| Bob
    mplCoreProgram --> Collection
    token --> campaignAtaVestingcampaigntokenMint
    token --> BobATAMint
    system --> BobspositionNFT
    VestingPositions --> Bobsreceipt
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,1,2,3,4,5 stroke:#456ea1,stroke-width:2px
```

### Tree

```

Bob (61744cu)
└─ VestingPositions::Claim ✗ 61744cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   └─ system::createAccount ✓
```

</details>

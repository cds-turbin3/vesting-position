# claims_at_linear_checkpoints

**Source:** [`tests/vesting_schedule.rs` L181](https://github.com/cds-turbin3/vesting-position/blob/7c9fb3a609fc786de731fb5d2ea82919f4974b08/babelfish-tests/tests/vesting_schedule.rs#L181)

<details>
<summary>Over 31 days: 16 moments; 1/1 law held.</summary>

Invariants (laws):

- Alice balance is monotonic ✓ across 16 moment(s)

</details>

<details>
<summary>Cast</summary>

| name | address |
| --- | --- |
| Alice | 4wQQJM9LNuhinieNAqmHuPCm8LXDTVfhx84P32nAVE9P |
| Alice's position NFT | 6hhAjXPGt41Y6oPH6mATnAqMECdaHrKaAGbE4SFuARXK |
| Alice's receipt | 5yfASCcX25V4fzBgdfixtXgS2JrvpWdwX27JQXpVyuHB |
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

| time | Vault balance | Δ | Creator balance | Δ | Alice claimable (schedule ceiling) | Δ | Alice balance | Δ | comment |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T0 | 10,000,000,000,000 | +10,000,000,000,000 | 0 |  | — |  | — |  | Initialize (day 0) |
| T1 | 9,900,000,000,000 | -100,000,000,000 | 0 |  | 100,000,000,000 | +100,000,000,000 | 100,000,000,000 | +100,000,000,000 | FirstClaim (day 2) |
| T2 | 9,891,000,000,000 | -9,000,000,000 | 0 |  | 109,000,000,000 | +9,000,000,000 | 109,000,000,000 | +9,000,000,000 | Claim (day 2) |
| T3 | 9,837,000,000,000 | -54,000,000,000 | 0 |  | 163,000,000,000 | +54,000,000,000 | 163,000,000,000 | +54,000,000,000 | Claim (day 4) |
| T4 | 9,783,000,000,000 | -54,000,000,000 | 0 |  | 217,000,000,000 | +54,000,000,000 | 217,000,000,000 | +54,000,000,000 | Claim (day 5) |
| T5 | 9,720,000,000,000 | -63,000,000,000 | 0 |  | 280,000,000,000 | +63,000,000,000 | 280,000,000,000 | +63,000,000,000 | Claim (day 7) |
| T6 | 9,603,000,000,000 | -117,000,000,000 | 0 |  | 397,000,000,000 | +117,000,000,000 | 397,000,000,000 | +117,000,000,000 | Claim (day 11) |
| T7 | 9,495,000,000,000 | -108,000,000,000 | 0 |  | 505,000,000,000 | +108,000,000,000 | 505,000,000,000 | +108,000,000,000 | Claim (day 15) |
| T8 | 9,450,000,000,000 | -45,000,000,000 | 0 |  | 550,000,000,000 | +45,000,000,000 | 550,000,000,000 | +45,000,000,000 | Claim (day 16) |
| T9 | 9,387,000,000,000 | -63,000,000,000 | 0 |  | 613,000,000,000 | +63,000,000,000 | 613,000,000,000 | +63,000,000,000 | Claim (day 18) |
| T10 | 9,297,000,000,000 | -90,000,000,000 | 0 |  | 703,000,000,000 | +90,000,000,000 | 703,000,000,000 | +90,000,000,000 | Claim (day 21) |
| T11 | 9,225,000,000,000 | -72,000,000,000 | 0 |  | 775,000,000,000 | +72,000,000,000 | 775,000,000,000 | +72,000,000,000 | Claim (day 23) |
| T12 | 9,153,000,000,000 | -72,000,000,000 | 0 |  | 847,000,000,000 | +72,000,000,000 | 847,000,000,000 | +72,000,000,000 | Claim (day 26) |
| T13 | 9,081,000,000,000 | -72,000,000,000 | 0 |  | 919,000,000,000 | +72,000,000,000 | 919,000,000,000 | +72,000,000,000 | Claim (day 28) |
| T14 | 9,009,000,000,000 | -72,000,000,000 | 0 |  | 991,000,000,000 | +72,000,000,000 | 991,000,000,000 | +72,000,000,000 | Claim (day 30) |
| T15 | 9,000,000,000,000 | -9,000,000,000 | 0 |  | 1,000,000,000,000 | +9,000,000,000 | 1,000,000,000,000 | +9,000,000,000 | Claim (day 31) |

<details>
<summary>Chart</summary>

```mermaid
%%{init: {"themeVariables": {"xyChart": {"plotColorPalette": "#e0641e, #3987e5, #2ca02c, #9a5ce0, #d9504f"}}}}%%
xychart-beta
    x-axis ["d0", "d2", "d2", "d4", "d5", "d7", "d11", "d15", "d16", "d18", "d21", "d23", "d26", "d28", "d30", "d31"]
    line [10000000000000, 9900000000000, 9891000000000, 9837000000000, 9783000000000, 9720000000000, 9603000000000, 9495000000000, 9450000000000, 9387000000000, 9297000000000, 9225000000000, 9153000000000, 9081000000000, 9009000000000, 9000000000000]
    line [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    line [0, 100000000000, 109000000000, 163000000000, 217000000000, 280000000000, 397000000000, 505000000000, 550000000000, 613000000000, 703000000000, 775000000000, 847000000000, 919000000000, 991000000000, 1000000000000]
    line [0, 100000000000, 109000000000, 163000000000, 217000000000, 280000000000, 397000000000, 505000000000, 550000000000, 613000000000, 703000000000, 775000000000, 847000000000, 919000000000, 991000000000, 1000000000000]
```

*🟠 Vault balance · 🔵 Creator balance · 🟢 Alice claimable (schedule ceiling) · 🟣 Alice balance*

</details>

<details>
<summary>Flows</summary>

```mermaid
%%{init: {"themeVariables": {"xyChart": {"plotColorPalette": "#e0641e, #3987e5, #2ca02c, #9a5ce0, #d9504f"}}}}%%
xychart-beta
    x-axis ["d0", "d2", "d2", "d4", "d5", "d7", "d11", "d15", "d16", "d18", "d21", "d23", "d26", "d28", "d30", "d31"]
    y-axis "change since first sample"
    line [0, -100000000000, -109000000000, -163000000000, -217000000000, -280000000000, -397000000000, -505000000000, -550000000000, -613000000000, -703000000000, -775000000000, -847000000000, -919000000000, -991000000000, -1000000000000]
    line [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    line [0, 100000000000, 109000000000, 163000000000, 217000000000, 280000000000, 397000000000, 505000000000, 550000000000, 613000000000, 703000000000, 775000000000, 847000000000, 919000000000, 991000000000, 1000000000000]
    line [0, 100000000000, 109000000000, 163000000000, 217000000000, 280000000000, 397000000000, 505000000000, 550000000000, 613000000000, 703000000000, 775000000000, 847000000000, 919000000000, 991000000000, 1000000000000]
```

*🟠 Vault balance (-1T) · 🔵 Creator balance (0) · 🟢 Alice claimable (schedule ceiling) (+1T) · 🟣 Alice balance (+1T)*

</details>

### T0: Initialize (day 0)

| observation | before | after |
| --- | --- | --- |
| Vault balance | — | 10,000,000,000,000 |
| Creator balance | — | 0 |

*observed Alice claimable (schedule ceiling): 0*

*observed Alice claimable (schedule ceiling): 100,000,000,000*

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

*2 days pass.*

### T1: FirstClaim (day 2)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 10,000,000,000,000 | 9,900,000,000,000 |
| Alice balance | — | 100,000,000,000 |

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
    p6-->>p2: ✓ 29646cu
    deactivate p6
    p2->>p4: transferChecked
    activate p4
    p4-->>p2: ✓ 105cu
    deactivate p4
    p2-->>p0: ✓ 168121cu
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
    Vestingcampaign(["Vesting campaign"]):::signer
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
    token --> campaignAtaVestingcampaigntokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,6,8,9,11,12,14,15,16,19 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,7,10,13,17,18 stroke:#b87800,stroke-width:2px
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

Alice (168271cu)
├─ computeBudget::setComputeUnitLimit ✓
└─ VestingPositions::Claim ✓ 168121cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ system::createAccount ✓
   ├─ mplCoreProgram::CreateV2 ✓ 29646cu
   │  ├─ system::createAccount ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  └─ system::transferSol ✓
   └─ token::transferChecked ✓ 105cu
```

</details>

*25056 seconds pass.*

### T2: Claim (day 2)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,900,000,000,000 | 9,891,000,000,000 |
| Alice claimable (schedule ceiling) | 100,000,000,000 | 109,000,000,000 |
| Alice balance | 100,000,000,000 | 109,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*150336 seconds pass.*

### T3: Claim (day 4)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,891,000,000,000 | 9,837,000,000,000 |
| Alice claimable (schedule ceiling) | 109,000,000,000 | 163,000,000,000 |
| Alice balance | 109,000,000,000 | 163,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*150336 seconds pass.*

### T4: Claim (day 5)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,837,000,000,000 | 9,783,000,000,000 |
| Alice claimable (schedule ceiling) | 163,000,000,000 | 217,000,000,000 |
| Alice balance | 163,000,000,000 | 217,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*175392 seconds pass.*

### T5: Claim (day 7)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,783,000,000,000 | 9,720,000,000,000 |
| Alice claimable (schedule ceiling) | 217,000,000,000 | 280,000,000,000 |
| Alice balance | 217,000,000,000 | 280,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*325728 seconds pass.*

### T6: Claim (day 11)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,720,000,000,000 | 9,603,000,000,000 |
| Alice claimable (schedule ceiling) | 280,000,000,000 | 397,000,000,000 |
| Alice balance | 280,000,000,000 | 397,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*300672 seconds pass.*

### T7: Claim (day 15)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,603,000,000,000 | 9,495,000,000,000 |
| Alice claimable (schedule ceiling) | 397,000,000,000 | 505,000,000,000 |
| Alice balance | 397,000,000,000 | 505,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*125280 seconds pass.*

### T8: Claim (day 16)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,495,000,000,000 | 9,450,000,000,000 |
| Alice claimable (schedule ceiling) | 505,000,000,000 | 550,000,000,000 |
| Alice balance | 505,000,000,000 | 550,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*175392 seconds pass.*

### T9: Claim (day 18)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,450,000,000,000 | 9,387,000,000,000 |
| Alice claimable (schedule ceiling) | 550,000,000,000 | 613,000,000,000 |
| Alice balance | 550,000,000,000 | 613,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*250560 seconds pass.*

### T10: Claim (day 21)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,387,000,000,000 | 9,297,000,000,000 |
| Alice claimable (schedule ceiling) | 613,000,000,000 | 703,000,000,000 |
| Alice balance | 613,000,000,000 | 703,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*200448 seconds pass.*

### T11: Claim (day 23)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,297,000,000,000 | 9,225,000,000,000 |
| Alice claimable (schedule ceiling) | 703,000,000,000 | 775,000,000,000 |
| Alice balance | 703,000,000,000 | 775,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*200448 seconds pass.*

### T12: Claim (day 26)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,225,000,000,000 | 9,153,000,000,000 |
| Alice claimable (schedule ceiling) | 775,000,000,000 | 847,000,000,000 |
| Alice balance | 775,000,000,000 | 847,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*200448 seconds pass.*

### T13: Claim (day 28)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,153,000,000,000 | 9,081,000,000,000 |
| Alice claimable (schedule ceiling) | 847,000,000,000 | 919,000,000,000 |
| Alice balance | 847,000,000,000 | 919,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*200448 seconds pass.*

### T14: Claim (day 30)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,081,000,000,000 | 9,009,000,000,000 |
| Alice claimable (schedule ceiling) | 919,000,000,000 | 991,000,000,000 |
| Alice balance | 919,000,000,000 | 991,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as VestingPositions
    participant p2 as mplCoreProgram
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20791cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76074cu
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
    token --> campaignAtaVestingcampaigntokenMint
    token --> userAtaAlicetokenMint
    Vestingcampaign --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf,color:#17202a;
    classDef signer fill:#d5e8d4,stroke:#82b366,color:#17202a;
    classDef state fill:#ffe6cc,stroke:#d79b00,color:#17202a;
    linkStyle 0,8,9,12 stroke:#5f913f,stroke-width:2px
    linkStyle 1,2,3,4,5,6,7,10,11 stroke:#b87800,stroke-width:2px
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

Alice (76074cu)
└─ VestingPositions::Claim ✓ 76074cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 20791cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*25056 seconds pass.*

### T15: Claim (day 31)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,009,000,000,000 | 9,000,000,000,000 |
| Alice claimable (schedule ceiling) | 991,000,000,000 | 1,000,000,000,000 |
| Alice balance | 991,000,000,000 | 1,000,000,000,000 |

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
    p2-->>p1: ✓ 23442cu
    deactivate p2
    p1->>p4: transferChecked
    activate p4
    p4-->>p1: ✓ 105cu
    deactivate p4
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 13574cu
    deactivate p2
    p1-->>p0: ✓ 97578cu
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

Alice (97578cu)
└─ VestingPositions::Claim ✓ 97578cu
   ├─ mplCoreProgram::UpdatePlugin ✓ 23442cu
   │  └─ system::transferSol ✓
   ├─ token::transferChecked ✓ 105cu
   └─ mplCoreProgram::UpdatePlugin ✓ 13574cu
```

</details>

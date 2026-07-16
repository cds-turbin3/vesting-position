# Vested tokens forfeited past the grace window

**Source:** [`tests/forfeiture.rs` L123](https://github.com/cds-turbin3/vesting-position/blob/6858b355e82f63b7269a885b34df983c66140aa4/babelfish-tests/tests/forfeiture.rs#L123)

- Given: Alice is whitelisted in a live campaign with a grace window past end
- Then: the cliff unlock reached Alice ✓
- Then: at end, Alice's vested-but-unclaimed remainder is the whole rest of her allocation ✓
- Then: the grace window let the creator reclaim Alice's fully-vested remainder ✓
- Then: Alice's balance never moved across the clawback: her remainder was forfeited, not delivered ✓

Result: ✓ success — 76464 CU · claims 4/4 ✓

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as Vesting
    participant p2 as CoRE…
    participant p3 as token
    p0->>p1: Clawback
    activate p1
    p1->>p2: Burn
    activate p2
    p2-->>p1: ✓ 9904cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 76464cu
    deactivate p1
```

## Authority

```mermaid
flowchart LR
    Vesting["Vesting"]:::program
    Creator(["Creator"]):::signer
    Collection[("Collection")]:::state
    AlicespositionNFT[("Alice's position NFT")]:::state
    CampaignATAMint[("Campaign/ATA(Mint)")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    CoRE["CoRE…"]:::program
    vGh5(["vGh5…"]):::signer
    token["token"]:::program
    Campaign(["Campaign"]):::signer
    Creator -->|signs| Vesting
    Vesting -->|writes| Collection
    Vesting -->|writes| AlicespositionNFT
    Vesting -->|writes| CampaignATAMint
    Vesting -->|writes| CreatorATAMint
    CoRE -->|writes| AlicespositionNFT
    CoRE -->|writes| Collection
    Creator -->|signs| CoRE
    vGh5 -->|signs| CoRE
    token -->|writes| CampaignATAMint
    token -->|writes| CreatorATAMint
    Campaign -->|signs| token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

## Ownership

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    AlicespositionNFT[("Alice's position NFT")]:::state
    token["token"]:::program
    CampaignATAMint[("Campaign/ATA(Mint)")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    system -->|owns| Creator
    CoRE -->|owns| Collection
    CoRE -->|owns| AlicespositionNFT
    token -->|owns| CampaignATAMint
    token -->|owns| CreatorATAMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

## Accounts

| Account | Signer | Writable | Owner |
| --- | --- | --- | --- |
| Creator | ✓ | ✓ | system |
| Campaign | · | · | Vesting |
| Collection | · | ✓ | CoRE… |
| vGh5… | · | · | system |
| Alice's position NFT | · | ✓ | CoRE… |
| Mint | · | · | token |
| Campaign/ATA(Mint) | · | ✓ | token |
| Creator/ATA(Mint) | · | ✓ | token |
| system | · | · | Nati… |
| token | · | · | BPFL… |
| splAssociatedTokenAccount | · | · | BPFL… |
| CoRE… | · | · | BPFL… |

<details>
<summary>Call tree and logs</summary>

```
Vested tokens forfeited past the grace window
Creator (76464cu)
└─ Vesting::Clawback ✓ 76464cu
   ├─ CoRE…::Burn ✓ 9904cu
   └─ token::transferChecked ✓ 105cu
```

```text
Program 7DkU9TQhcN87f2djZDd2MjjPZoXLfnZZj8HhybeZswX1 invoke [1]
Program log: Instruction: Clawback
Program CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d invoke [2]
Program log: Instruction: Burn
Program log: programs/mpl-core/src/plugins/internal/permanent/permanent_burn_delegate.rs:47:ForceApprove
Program log: programs/mpl-core/src/plugins/lifecycle.rs:733:ForceApprove
Program CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d consumed 9904 of 137236 compute units
Program CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d success
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 105 of 124967 compute units
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success
Program data: Lxc8VPVysqkbj9rxkNx6BsE7KZFxFkI1/cY0dGcqUEcemDlVNIqYVTKYElYssupGpCP43Yuvemqw+VhRCiwfNG1WXbNCQrveOoLihoc4dWQbnXmR6BD2b2ycAp5izUwQ8iaXjFeUnwo6guKGhzh1ZBudeZHoEPZvbJwCnmLNTBDyJpeMV5SfCgAoLozRAAAAAQqGZQAAAAA=
Program 7DkU9TQhcN87f2djZDd2MjjPZoXLfnZZj8HhybeZswX1 consumed 76464 of 200000 compute units
Program 7DkU9TQhcN87f2djZDd2MjjPZoXLfnZZj8HhybeZswX1 success
```

</details>

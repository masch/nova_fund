# Crowdfunding Contract - Soroban Workshop

**Technical Bootcamp Part I: Soroban & Architecture Fundamentals**

## Introducción

Este proyecto es una demostración práctica para aprender conceptos fundamentales de Soroban y el desarrollo de contratos inteligentes.  
Fue creado para el workshop de BAF en el marco del Stellar GIVE Hackathon Argentina 2025.

Se trata de un contrato básico de crowdfunding en Rust que permite a creadores lanzar campañas con metas de recaudación, aceptar contribuciones, y gestionar retiros y reembolsos.

---

## Setup

### Rust Toolchain

Descarga e instala Rust siguiendo la guía oficial:
https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup

### Target

Luego instala el target WASM según tu versión de Rustc:

```bash
# Si tienes rustc 1.85 o superior
rustup target add wasm32v1-none

# Si tienes rustc menor a 1.85
rustup target add wasm32-unknown-unknown
```

### Instalar Stellar CLI

```bash
cargo install --locked stellar-cli@23.0.0
```

---

## Extensiones para VS Code

1️⃣ Even Better TOML  
2️⃣ CodeLLDB (debugging paso a paso)  
3️⃣ Rust Analyzer (soporte para Rust)

---

## Comandos básicos para crear y desplegar el contrato

### Deploy en Testnet:

🔑 Generar Keypair para las pruebas

```bash
stellar keys generate --global alice --network testnet --fund
```

📌 Pasos para el deploy:
1️⃣ Compilar el contrato y generar el archivo .wasm

```bash
# Si tienes rustc 1.85 o superior
  cargo build --target wasm32v1-none --release
 stellar contract build
# Si tienes rustc menor a 1.85
  cargo build --target wasm32-unknown-unknown --release
```

2️⃣ Optimizar el contrato para reducir su tamaño en bytes

```bash
# Si tienes rustc 1.85 o superior
   stellar contract optimize --wasm target/wasm32v1-none/release/<contract_name>.wasm

# Si tienes rustc menor a 1.85
 stellar contract optimize --wasm target/wasm32-unknown-unknown/release/<contract_name>.wasm
```

1️⃣ Generar Admin Keypair para las pruebas

```bash
stellar keys generate --global admin --network testnet --fund
```

2️⃣ Obtener el token address de XLM para usar en el contrato

```bash
stellar contract asset id --asset native --network testnet
```

_Nota: devuelve `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`_

4️⃣ Obtener el admin public key

```bash
stellar keys address admin
```

_Nota: devuelve `GDXAECCYWYW2QKQDTGVQUTC6CQEQR3REC3PKZKXOP76PJJ6V3FRYXCO3`_

5️⃣ Deployar el contrato en la Testnet y obtener el contract ID

```bash
    stellar contract deploy `
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm `
        --source admin `
        --network testnet `
        -- `
        --admin <admin_public_key>
        --token <token_address>
```

_Nota: devuelve `CBAH4Z5CNELXMN7PVW2SAAB6QVOID34SAQAFHJF7Q7JUNACRQEJX66MB`_

---

## Funciones del Contrato

| Función           | Descripción                                                              | Firma                                                                                  |
| ----------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `__constructor`   | Inicializa el contrato con admin y token                                 | `(admin: address, token: address) -> Result<(), Error>`                                |
| `create_campaign` | Crea una campaña con goal y min_donation                                 | `(creator: address, goal: i128, min_donation: i128) -> Result<(), Error>`              |
| `get_campaign`    | Obtiene los datos de una campaña                                         | `(campaign_address: address) -> Result<Campaign, Error>`                               |
| `contribute`      | Permite a un usuario aportar a una campaña                               | `(contributor: address, campaign_address: address, amount: i128) -> Result<(), Error>` |
| `withdraw`        | Permite al creador retirar fondos si goal fue alcanzado                  | `(creator: address) -> Result<(), Error>`                                              |
| `refund`          | Permite a un contribuyente retirar su aporte si la campaña no tuvo éxito | `(contributor: address, campaign_address: address) -> Result<(), Error>`               |

---

## Estructuras Principales

```rust
#[contracttype]
struct Campaign {
  goal: i128,
  min_donation: i128,
  supporters: u32,
  total_raised: i128,
}

#[contracttype]
struct Contribution {
  amount: i128,
}

#[contracttype]
enum DataKey {
  Admin(),
  Token(),
  Campaign(address),
  Contribution(address, address),
}

#[contracterror]
enum Errors {
  ContractInitialized = 0,
  ContractNotInitialized = 1,
  MathOverflow = 2,
  MathUnderflow = 3,
  CampaignNotFound = 4,
  CampaignGoalExceeded = 5,
  ContributionBelowMinimum = 6,
  AmountMustBePositive = 7,
  CampaignGoalNotReached = 8,
  ContributionNotFound = 9,
  CampaignAlreadyExists = 10,
}
```

---

## Funciones del contrato desde el Stellar CLI

### Create Campaign

```bash
    stellar contract deploy `
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm `
        --source admin `
        --network testnet `
        -- create_campaign `
        --creator <creator_public_key>
        --goal 100000000
```

### Get Campaign

```bash
    stellar contract deploy `
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm `
        --source admin `
        --network testnet `
        -- get_campaign `
        --campaign_address <creator_public_key>
```

### Add Contribution

```bash
    stellar contract deploy `
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm `
        --source <contributor_secret_key> `
        --network testnet `
        -- contribute `
        --contributor <contributor_public_key>
        --campaign_address <creator_public_key>
        --amount 100000000
```

---

## Nota:

| XLM     | Stroops       | Explicación                             |
| ------- | ------------- | --------------------------------------- |
| 1 XLM   | 10,000,000    | 1 XLM equivale a 10 millones de stroops |
| 5 XLM   | 50,000,000    | 5 XLM en stroops                        |
| 10 XLM  | 100,000,000   | 10 XLM en stroops                       |
| 100 XLM | 1,000,000,000 | 100 XLM en stroops                      |

---

## Conclusion

Este contrato fue desarrollado exclusivamente con fines educativos dentro del contexto del bootcamp, sirviendo como una base práctica para entender los conceptos fundamentales de Soroban y el desarrollo de contratos inteligentes. No está diseñado ni recomendado para ser utilizado en entornos de producción sin antes pasar por una auditoría exhaustiva que garantice su seguridad y robustez. A lo largo del workshop, se profundizará en aspectos clave como la arquitectura del contrato, las mejores prácticas de seguridad y el manejo adecuado de estados, para que los participantes puedan construir soluciones más confiables y escalables.

# Get admin : stellar keys address admin

G@E4H}3iXaucvjL

cargo clean && cargo build --target wasm32v1-none --release && stellar contract optimize --wasm target/wasm32v1-none/release/baf_crowdfunding_contract.wasm

cargo build --target wasm32v1-none --release && stellar contract optimize --wasm target/wasm32v1-none/release/baf_crowdfunding_contract.wasm

stellar keys address admin => GCISVJIR6CT6VVYN7AJ5AXMHNPZZMZI4U3CELHYXQW77X4E42AUHC7AM
stellar keys secret admin => SAD5I7SBUKVUJW5FNUAQSJBYWQBNQBUFJPFT3GKVAIO6CXOC434PBQ3O
stellar keys secret admin --phrase => SAD5I7SBUKVUJW5FNUAQSJBYWQBNQBUFJPFT3GKVAIO6CXOC434PBQ3O audit dirt history olympic define post must orphan path salute orphan joke below cart blast extend exchange farm excite bonus steak almost deny noble

stellar keys generate alice-ong-1 --network testnet --fund
stellar keys address alice-ong-1 => GA76IHDYDMDZE3Q4PPY2YY64SCPYCTFJTWGHDH5LGHIROLV7WU6DFN7M
stellar keys secret alice-ong-1 => SCFNSTKQPDDSQIFVY6S6XVILHB6DHDRKPMZVQXPZDGCES7SDDXYOCHOJ

stellar keys generate alice-ong-2 --network testnet --fund
stellar keys address alice-ong-2 => GB2IMTB3E3NTXC6PSAS2AY3NNQ2U32AUQVGSO4B6QSNXPJTQGSL7GBNU
stellar keys secret alice-ong-2 => SBHVJENQ65UPKI56OA6UHIEU36FBGUPOGB3L3O3LWAUYCF2ACZ3IKXSI

stellar keys generate alice-contributor --network testnet --fund
stellar keys address alice-contributor => GALX2CBQFDKI32QJMTKLNZSQXR4DX7CEG5DTDGJBLCFEAXQUXM4RKXQZ
stellar keys secret alice-contributor => SCYCNRP7W6L7EJT6SJ467DUC4OP7KMKMRYHQMOCXFICVHR7WDE77YNNK
stellar keys secret alice-contributor --phrase => second shop kiss basket resource notice wait magnet pyramid engage voice aunt miracle sock card chase scrub title fish library admit language smoke sure

stellar keys generate alice-beneficiary-1 --network testnet --fund
stellar keys address alice-beneficiary-1 => GDIVVKR333DKTSFGGJYIG37VMZCK2OOURUBYYQKK7MVGDL5N2JXO2JFT
stellar keys secret alice-beneficiary-1 => SCDNMNZCHLMQ3CQ4JRJF2DOBNDYIRHSPY53CRELG6ZXH2SPECYDXWI6Q
stellar keys secret alice-beneficiary-1 --phrase => twice frozen cook worry glad choose obey disorder shed antenna zone crystal ticket letter attend arena snack step torch switch light actor brown traffic

stellar keys generate alice-beneficiary-2 --network testnet --fund
stellar keys address alice-beneficiary-2 => GDD4BFT3YSDHAAKIFXFKNSVLVGGU7NDIOYNTMR7NXJVNOZLCCJOQCUTS
stellar keys secret alice-beneficiary-2 => SDF2QVPUGVN2U4IL5NDZV6SXBEXQGQWGYGKF2F77Q22DIJS3CEO6N3K3
stellar keys secret alice-beneficiary-2 --phrase => wonder alone illness carry fever search action youth oval stomach sweet shed almost ocean other sword culture shallow veteran excite level fuel produce elevator

stellar contract alias show contract_address

    stellar contract deploy \
        --wasm target/wasm32v1-none/release/baf_crowdfunding_contract.optimized.wasm \
        --alias contract_address \
        --source admin \
        --network testnet \
        -- \
        --admin GCISVJIR6CT6VVYN7AJ5AXMHNPZZMZI4U3CELHYXQW77X4E42AUHC7AM \
        --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
        --minimum_donation 10000000

    stellar contract invoke \
        --id contract_address \
        --source admin \
        --network testnet \
        -- create_ong \
        --ong GA76IHDYDMDZE3Q4PPY2YY64SCPYCTFJTWGHDH5LGHIROLV7WU6DFN7M

    stellar contract invoke \
        --id contract_address \
        --source admin \
        --network testnet \
        -- create_ong \
        --ong GB2IMTB3E3NTXC6PSAS2AY3NNQ2U32AUQVGSO4B6QSNXPJTQGSL7GBNU

    stellar contract invoke \
        --id contract_address \
        --source alice-ong-1 \
        --network testnet \
        -- create_campaign \
        --creator GA76IHDYDMDZE3Q4PPY2YY64SCPYCTFJTWGHDH5LGHIROLV7WU6DFN7M \
        --beneficiary GDIVVKR333DKTSFGGJYIG37VMZCK2OOURUBYYQKK7MVGDL5N2JXO2JFT \
        --goal 100000000 \
        --min_donation 100000

    stellar contract invoke \
        --id contract_address \
        --source alice-ong-2 \
        --network testnet \
        -- create_campaign \
        --creator GB2IMTB3E3NTXC6PSAS2AY3NNQ2U32AUQVGSO4B6QSNXPJTQGSL7GBNU \
        --beneficiary GDD4BFT3YSDHAAKIFXFKNSVLVGGU7NDIOYNTMR7NXJVNOZLCCJOQCUTS \
        --goal 100000000 \
        --min_donation 100000

    stellar contract invoke \
        --id contract_address \
        --source admin \
        --network testnet \
        -- get_campaign \
        --campaign_id 1

    stellar contract invoke \
        --id contract_address \
        --source alice-contributor \
        --network testnet \
        -- contribute \
        --contributor GALX2CBQFDKI32QJMTKLNZSQXR4DX7CEG5DTDGJBLCFEAXQUXM4RKXQZ \
        --campaign_id GDIVVKR333DKTSFGGJYIG37VMZCK2OOURUBYYQKK7MVGDL5N2JXO2JFT \
        --amount 10000000

    stellar contract invoke \
        --id contract_address \
        --source alice-contributor \
        --network testnet \
        -- refund \
        --contributor GALX2CBQFDKI32QJMTKLNZSQXR4DX7CEG5DTDGJBLCFEAXQUXM4RKXQZ \
        --campaign_id 0


    stellar contract invoke \
        --id contract_address \
        --source alice-beneficiary \
        --network testnet \
        -- withdraw \
        --campaign_id 0

#!/bin/bash
#set -e

echo "***************************"
echo -e "\t*****Building*****..."
echo "***************************"
cargo build --target wasm32v1-none --release && stellar contract optimize --wasm target/wasm32v1-none/release/baf_crowdfunding_contract.wasm

echo "*********************"
echo -e "\tDeploying ..."
echo "*********************"
stellar contract deploy \
    --wasm target/wasm32v1-none/release/baf_crowdfunding_contract.optimized.wasm \
    --alias contract_address \
    --source admin \
    --network testnet \
    -- \
    --admin GCISVJIR6CT6VVYN7AJ5AXMHNPZZMZI4U3CELHYXQW77X4E42AUHC7AM \
    --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
    --minimum_donation 10000000

echo "**************************************"
echo -e "\tCreation of the Delta del Trige NGO ..."
echo "**************************************"
stellar contract invoke \
    --id contract_address \
    --source admin \
    --network testnet \
    -- create_ong \
    --ong GA76IHDYDMDZE3Q4PPY2YY64SCPYCTFJTWGHDH5LGHIROLV7WU6DFN7M

echo "**************************************"
echo -e "\tCreation of the Water Forests NGO ..."
echo "**************************************"
stellar contract invoke \
    --id contract_address \
    --source admin \
    --network testnet \
    -- create_ong \
    --ong GB2IMTB3E3NTXC6PSAS2AY3NNQ2U32AUQVGSO4B6QSNXPJTQGSL7GBNU

echo "*****************************************"
echo -e "\tLaunch of the High Peaks Reforestation campaign for the Water Forests NGO ..."
echo "*****************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-ong-1 \
    --network testnet \
    -- create_campaign \
    --creator GA76IHDYDMDZE3Q4PPY2YY64SCPYCTFJTWGHDH5LGHIROLV7WU6DFN7M \
    --beneficiary GDIVVKR333DKTSFGGJYIG37VMZCK2OOURUBYYQKK7MVGDL5N2JXO2JFT \
    --goal 100000000 \
    --min_donation 100000

echo "*****************************************"
echo -e "\tLaunch of the Solar Panels campaign for the Delta de Trige NGO ..."
echo "*****************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-ong-2 \
    --network testnet \
    -- create_campaign \
    --creator GB2IMTB3E3NTXC6PSAS2AY3NNQ2U32AUQVGSO4B6QSNXPJTQGSL7GBNU \
    --beneficiary GDD4BFT3YSDHAAKIFXFKNSVLVGGU7NDIOYNTMR7NXJVNOZLCCJOQCUTS \
    --goal 100000000 \
    --min_donation 100000


echo "*****************************************"
echo -e "\tLaunch another of the Solar Panels campaign for the Delta de Trige NGO ..."
echo "*****************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-ong-2 \
    --network testnet \
    -- create_campaign \
    --creator GB2IMTB3E3NTXC6PSAS2AY3NNQ2U32AUQVGSO4B6QSNXPJTQGSL7GBNU \
    --beneficiary GBA3T3WCRYPWYYG4YACJ5I3EB3JOQIFWQBML6FOK7SP6JWXIH5LJVDRS \
    --goal 100000000 \
    --min_donation 100000

echo "**********************************************"
echo -e "\tContribution to the Solar Panels campaign for the Delta de Tigre NGO  ..."
echo "**********************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-contributor \
    --network testnet \
    -- contribute \
    --contributor GALX2CBQFDKI32QJMTKLNZSQXR4DX7CEG5DTDGJBLCFEAXQUXM4RKXQZ \
    --campaign_id GDIVVKR333DKTSFGGJYIG37VMZCK2OOURUBYYQKK7MVGDL5N2JXO2JFT \
    --amount 10000000

echo "****************************************************"
echo -e "\tAnother contribution to the Solar Panels campaign for the Water Forests NGO ..."
echo "****************************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-contributor \
    --network testnet \
    -- contribute \
    --contributor GALX2CBQFDKI32QJMTKLNZSQXR4DX7CEG5DTDGJBLCFEAXQUXM4RKXQZ \
    --campaign_id GDIVVKR333DKTSFGGJYIG37VMZCK2OOURUBYYQKK7MVGDL5N2JXO2JFT \
    --amount 20000000

echo "**********************************************"
echo -e "\tContribution to the "Reforestación Altas Cumbres" campaign for the Water Forests NGO ..."
echo "**********************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-contributor \
    --network testnet \
    -- contribute \
    --contributor GALX2CBQFDKI32QJMTKLNZSQXR4DX7CEG5DTDGJBLCFEAXQUXM4RKXQZ \
    --campaign_id GDD4BFT3YSDHAAKIFXFKNSVLVGGU7NDIOYNTMR7NXJVNOZLCCJOQCUTS \
    --amount 100000000


echo "****************************************************************"
echo -e "\tWithdrawing for Delta de Tigre NGO campaign => Goal NOT reached ..."
echo "****************************************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-beneficiary \
    --network testnet \
    -- withdraw \
    --campaign_id GDIVVKR333DKTSFGGJYIG37VMZCK2OOURUBYYQKK7MVGDL5N2JXO2JFT

echo "*************************************************************"
echo -e "\tWithdrawing for Water Forests NGO campaign => Goal reached ..."
echo "*************************************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-beneficiary \
    --network testnet \
    -- withdraw \
    --campaign_id GDD4BFT3YSDHAAKIFXFKNSVLVGGU7NDIOYNTMR7NXJVNOZLCCJOQCUTS

echo "**********************************************"
echo -e "\tRefunding for Delta de Tigre NGO campaign ..."
echo "**********************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-contributor \
    --network testnet \
    -- refund \
    --contributor GALX2CBQFDKI32QJMTKLNZSQXR4DX7CEG5DTDGJBLCFEAXQUXM4RKXQZ \
    --campaign_id GDIVVKR333DKTSFGGJYIG37VMZCK2OOURUBYYQKK7MVGDL5N2JXO2JFT


echo "******************************************************"
echo -e "\tOpening contract on Steller Expert explorer"
echo "******************************************************"

CONTRACT_ID=$(stellar contract alias show contract_address)
EXPLORER_URL="https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"
xdg-open "$EXPLORER_URL"
import React, { useState, useCallback, useEffect } from "react";
import { Widget, useAccount, useNear } from "near-social-vm";
import { callMethod, viewMethod } from "../utils/method";
import { open, create } from "@nearfoundation/near-js-encryption-box";

const contractID = process.env.REACT_APP_CONTRACT_ID
const nonce = process.env.REACT_APP_NONCE


export default function ListAcceptPage(props){
    const near = useNear()
    const account = useAccount();
    const [waitingList, setWaitingList] = useState([])

    useEffect(() => {
        if(!near) return

        const fetchData = async () => {
            let data = await viewMethod(near, {
                contractId: contractID,
                method: "get_waiting_list",
                args: {
                    owner: account.accountId
                }
            })

            let response = data.map(item => {
                return viewMethod(near, {contractId: contractID, method: "get_data_by_id", args: {data_id: item[1]}})
                    .then((informationResponse) => {
                        let information = {
                            buyer: item[0],
                            ECID: item[1],
                            buyer_public_key: item[2],
                            informationData: informationResponse
                        }
                        return information
                    })
            })

            return response
        }

        fetchData().then((response => {
            Promise.all(response).then((results) => {
                setWaitingList(results)
            })
        }))
    }, [near, props])

    const confirm = useCallback(async (buyerId, ECID, is_access, pub_key)=>{

        const key = JSON.parse(localStorage.getItem("keys")).filter(key => {
            return key.accountID === account.accountId
        })[0]

        const cid = open(ECID, key.primaryKey.public, key.secondKey.private, nonce)
        const {secret} = create(cid, pub_key, key.primaryKey.private, nonce)

        const args = {
            buyer_id: buyerId,
            encrypted_cid: ECID,
            is_access: is_access,
            access_key: secret,
            contract_id: "harvardtp_ft.testnet",
            pub_key: key.primaryKey.public
        }

        await callMethod(near, {
            contractId: contractID,
            method: "confirm_transaction",
            args
        })

        return true

    }, [near, props])

    const passingProps = {
        ...props,
        waitingList,
        confirm
    }

    return <Widget src="tvh050423.testnet/widget/ListAcceptPage" props={passingProps}/>
}
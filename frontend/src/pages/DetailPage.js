import React, { useState, useCallback, useEffect } from "react";
import { useSearchParams } from "react-router-dom";
import { Widget, useAccount, useNear } from "near-social-vm";
import { callMethod, viewMethod } from "../utils/method";
import { open } from "@nearfoundation/near-js-encryption-box";
import axios from "axios";
import { convertJSON } from "../utils/readData";
import { createClient } from "@supabase/supabase-js";

const contractID = process.env.REACT_APP_CONTRACT_ID
const nonce = process.env.REACT_APP_NONCE
const URL = process.env.REACT_APP_SUPABASE_URL
const KEY = process.env.REACT_APP_SUPABASE_ANON_KEY

export default function DetailPage(props){

    const [queryParameters] = useSearchParams()
    const [inforFile, setInforFile] = useState()
    const [dataFile, setDataFile] = useState([])
    const [downloadURL, setDownloadURL] = useState("")
    const supabase = createClient(URL, KEY)
    const near = useNear()
    const account = useAccount();
    const id = queryParameters.get("id").replaceAll(" ", "+")
    const action = queryParameters.get("action")
    const isActive = queryParameters.get("isActive")
    const type = queryParameters.get("type")

    //get information of file
    const getInfor = useCallback(async () => {
        let information = {}
    
        if(!near){
          return information
        }
    
        information = await viewMethod(near, {contractId: contractID,
        method: "get_data_by_id", args: {data_id: id}})

        //get description
        const {data} = await supabase.from("description").select("description").eq("id", id).single()

        information.description = data.description

        //check that data is mine?
        const isMine = information.list_access.includes(account.accountId) || information.owner === account.accountId
        information.isMine = isMine
        
        return information
    }, [near, account])

    useEffect(() => {
        getInfor().then((response) => {
            setInforFile(response)
        })
    }, [near, props])

    //update state of product
    const updateState = useCallback(async (price) => {

        if(!near){
            return
        }

        const response = await callMethod(near, 
            {
                contractId: contractID, 
                method: "set_state",
                args: {
                    state: "Public",
                    cid: id,
                    price: Number(price),
                }
            })

    }, [near, account])

    //get data of file
    useEffect(() => {
        if(!inforFile || !inforFile.title || action === "maketPlace"){
            return
        }

        //get keys from local storage
        const key = JSON.parse(localStorage.getItem("keys")).filter(key => {
            return key.accountID === account.accountId
        })[0]

        if(type == "buy"){
            viewMethod(near, {contractId: contractID, method: "get_data_value", args: {encrypted_cid: id, user_id: account.accountId}}).then((result) => {

                const cid = open(result.key_cid, result.pub_key, key.primaryKey.private, nonce)
                const url = `https://${cid}.ipfs.w3s.link/${inforFile.title}.jsonl`

                axios.get(url, {
                    responseType: 'stream'
                })
                .then(response => {
                    const dataObject = convertJSON(response.data)
                    setDataFile(dataObject)
                })

                getLinkDownload(cid, inforFile.title)
            })
        }

        const cid = open(id, key.primaryKey.public, key.secondKey.private, nonce)
        const url = `https://${cid}.ipfs.w3s.link/${inforFile.title}.jsonl`

        axios.get(url, {
            responseType: 'stream'
        })
        .then(response => {
            const dataObject = convertJSON(response.data)
            setDataFile(dataObject)
        })

        getLinkDownload(cid, inforFile.title)
    }, [inforFile])

    //buy data func
    const purchaseData = useCallback(async (id, price = 0) => {
        if(!near){
          return
        }
    
        const key = JSON.parse(localStorage.getItem("keys")).filter(key => key.accountID === account.accountId)[0]
        const pub_key = key.primaryKey.public
    
        const args = {
          encrypted_cid: id,
          pub_key: pub_key,
          contract_id: "harvardtp_ft.testnet"
        }
    
        const response = await callMethod(near, {contractId: contractID,
          method: "purchase", args})
      }, [near, props])

    //get link download
    const getLinkDownload = (cid, title) => {
        fetch(`https://${cid}.ipfs.w3s.link/${title}.jsonl`, {
        method: 'GET',
        })
        .then(response => response.blob())
        .then(blob => {
            const url = window.URL.createObjectURL(new Blob([blob]));
            setDownloadURL(url)
        });
    }

    const passingProps = {
        ...props,
        inforFile,
        action,
        updateState,
        dataFile,
        isActive,
        purchaseData,
        downloadURL
    }

    return <Widget src="tvh050423.testnet/widget/DetailPage" props={passingProps}/>
}
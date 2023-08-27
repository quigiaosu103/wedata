import React, { useEffect, useState, useCallback } from "react";
import { Widget, useAccount, useNear } from "near-social-vm";
import { Web3Storage } from 'web3.storage';
import { callMethod, viewMethod } from "../utils/method";
import { create } from "@nearfoundation/near-js-encryption-box";
import { useNavigate } from "react-router-dom";
import { createClient } from "@supabase/supabase-js";

const contractID = process.env.REACT_APP_CONTRACT_ID
const URL = process.env.REACT_APP_SUPABASE_URL
const KEY = process.env.REACT_APP_SUPABASE_ANON_KEY

export default function StoragePage(props) {
  const client = new Web3Storage({ token: process.env.REACT_APP_WEB3STORAGE_TOKEN });
  const near = useNear()
  const account = useAccount();
  const [listFile, setListFile] = useState(null)
  const [isLoading, setIsLoading] = useState(true)
  const navigate = useNavigate();
  const supabase = createClient(URL, KEY)

  const getFiles = useCallback(async () => {
    let list = []

    if(!near){
      return list
    }

    list = await viewMethod(near, {contractId: contractID,
    method: "get_data_by_owner", args: {owner_account_id: account.accountId}})

    let boughtData = await viewMethod(near, {contractId: contractID, method: "get_accessed_data_by_user", args: {user_id: account.accountId}})
    
    return {
      storageData: list,
      purchaseData: boughtData
    }
  }, [near, account])

  const uploadFile = useCallback(async (file, description, tags) => {

    if (!near) {
      return;
    }
    
    //upload file on web3storage
    var cid
    if(!file){
      return {
        status: false,
        message: "File not found"
      }
    }
    
    if(props.signedIn){
      cid = await client.put([file], {name: file.name, })
    }else{
      return {
        status: false,
        message: "You must to login for upload"
      }
    }
    
    if(!cid){
      return {
        status: false,
        message: "An Error has occurred."
      }
    } 

    const nonce = process.env.REACT_APP_NONCE
    
    //hash
    const key = JSON.parse(localStorage.getItem("keys")).filter(key => key.accountID === account.accountId)[0]
    const public_key = key.secondKey.public
    const private_key = key.primaryKey.private
    const {secret} = create(cid, public_key, private_key, nonce)
    const hashCID = secret

    
    //convert tags
    if(tags.length === 0){
      return {
        status: false,
        message: "This data must have at least one tag"
      }
    }

    let tagsString = tags[0]
    tags.forEach((tag, index) => {
      if(index !== 0){
        tagsString += ", " + tag
      }
    })

    //call contract to save data
    const args = {
      title_given: file.name.split('.')[0],
      tags_given: tagsString,
      cid_encrypted_given: hashCID,
      size: String(file.size)
    }

    //push description to database
    const {error} = await supabase.from("description").insert({id: hashCID, description})

    const response = await callMethod(near,
      {contractId: contractID,
      method: "new_meta_data",
      args: args})

    return {
        status: true,
        message: "Successful!"
      }

  }, [near, props]);

  const reloadData = useCallback(() => {
    setIsLoading(true)
    getFiles().then((response) => {
      setListFile(response)
      setIsLoading(false)
    })
  }, [near, props])

  useEffect(() => {
    reloadData()
  }, [near, props])

  const propsPassing = {
    ...props,
    uploadFile,
    listFile,
    reloadData,
    isLoading,
    navigate,
  }

  return (
    <Widget props={propsPassing} src={"tvh050423.testnet/widget/DataStorage"} />
  );
}


import React, { useEffect, useState, useCallback } from "react";
import { Widget, useNear, useAccount } from "near-social-vm";
import { callMethod, viewMethod } from "../utils/method";
import { useNavigate } from "react-router-dom";

const contractID = process.env.REACT_APP_CONTRACT_ID

export default function DiscoverPage(props) {

  const near = useNear()
  const account = useAccount();
  const [data, setData] = useState([])
  const [isLoading, setIsLoading] = useState(true)
  const navigate = useNavigate();

  const getFiles = useCallback(async () => {
    let uploads = []

    if(!near){
      return uploads
    }

    uploads = await viewMethod(near, {contractId: contractID,
    method: "get_published_data", args: {user: account.accountId}})
    
    return uploads.reverse()
  }, [near, account])

  const reloadData = useCallback(() => {
    setIsLoading(true)
    getFiles().then((response) => {
      setData(response)
      setIsLoading(false)
    })
  }, [near, props])

  useEffect(() => {
    reloadData()
  }, [near, props])

  const purchaseData = useCallback(async (id, price) => {
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

  const propsPassing = {
    ...props,
    data,
    reloadData,
    isLoading,
    purchaseData,
    accountId: account.accountId,
    navigate
  }

  return (
    <Widget props={propsPassing} src={"tvh050423.testnet/widget/MarketPlace"} />
  );
}
import React, { useCallback, useEffect, useState } from "react";
import "bootstrap-icons/font/bootstrap-icons.css";
import "@near-wallet-selector/modal-ui/styles.css";
import "react-bootstrap-typeahead/css/Typeahead.css";
import "react-bootstrap-typeahead/css/Typeahead.bs5.css";
import "bootstrap/dist/js/bootstrap.bundle";
import "./App.scss";
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import StoragePage from "./pages/DataStoragePage"
import DiscoverPage from "./pages/DiscoverPage";
import Marketplace from "./pages/MarketPlace"
import ListAcceptPage from "./pages/ListAcceptPage";
import { setupWalletSelector } from "@near-wallet-selector/core";
import { setupNearWallet } from "@near-wallet-selector/near-wallet";
import { setupMyNearWallet } from "@near-wallet-selector/my-near-wallet";
import { setupSender } from "@near-wallet-selector/sender";
import { setupHereWallet } from "@near-wallet-selector/here-wallet";
import { setupMeteorWallet } from "@near-wallet-selector/meteor-wallet";
import { setupNeth } from "@near-wallet-selector/neth";
import { setupModal } from "@near-wallet-selector/modal-ui";
import {
  useAccount,
  useInitNear,
  useNear,
} from "near-social-vm";
import Navigation from "./components/navigation/Navigation";
import { NetworkId, Widgets } from "./data/widgets";
import { utils } from 'near-api-js';
import DetailPage from "./pages/DetailPage";
import Root from "./pages/Root";

function App(props) {
  const [connected, setConnected] = useState(false);
  const [signedIn, setSignedIn] = useState(false);
  const [signedAccountId, setSignedAccountId] = useState(null);
  const [walletModal, setWalletModal] = useState(null);

  const { initNear } = useInitNear();
  const near = useNear();
  const account = useAccount();
  const accountId = account.accountId;

  useEffect(() => {
    initNear &&
      initNear({
        networkId: NetworkId,
        selector: setupWalletSelector({
          network: NetworkId,
          modules: [
            setupNearWallet(),
            setupMyNearWallet(),
            setupSender(),
            setupHereWallet(),
            setupMeteorWallet(),
            setupNeth({
              gas: "0",
              bundle: false,
            }),
          ],
        }),
      });
  }, [initNear]);

  useEffect(() => {
    if (!near) {
      return;
    }
    near.selector.then((selector) => {
      setWalletModal(
        setupModal(selector, { contractId: near.config.contractName })
      );
    });
  }, [near]);

  const requestSignIn = useCallback(
    (e) => {
      e && e.preventDefault();
      walletModal.show();
      return false;
    },
    [walletModal]
  );

  const logOut = useCallback(async () => {
    if (!near) {
      return;
    }
    const wallet = await (await near.selector).wallet();
    wallet.signOut();
    near.accountId = null;
    setSignedIn(false);
    setSignedAccountId(null);
  }, [near]);

  const refreshAllowance = useCallback(async () => {
    alert(
      "You're out of access key allowance. Need sign in again to refresh it"
    );
    await logOut();
    requestSignIn();
  }, [logOut, requestSignIn]);

  useEffect(() => {
    if (!near) {
      return;
    }
    setSignedIn(!!accountId);
    setSignedAccountId(accountId);
    setConnected(true);
  }, [near, accountId]);

  useEffect(() => {
    if (!near || !accountId) {
      return;
    }

    const primaryKey = utils.key_pair.KeyPairEd25519.fromRandom();
    const secondKey = utils.key_pair.KeyPairEd25519.fromRandom();
    const data = JSON.parse(localStorage.getItem("keys"))

    if(!data) {
      localStorage.setItem("keys", JSON.stringify([
        {
          accountID: accountId,
          primaryKey: {
            public: primaryKey.getPublicKey().toString(),
            private: primaryKey.secretKey
          },
          secondKey: {
            public: secondKey.getPublicKey().toString(),
            private: secondKey.secretKey
          }
        }
      ]))
      return
    }

    const keys = data.filter(data => {
      return data.accountID === accountId
    })

    if(keys.length === 0){
      localStorage.setItem("keys", JSON.stringify([
        ...data,
        {
          accountID: accountId,
          primaryKey: {
            public: primaryKey.getPublicKey().toString(),
            private: primaryKey.secretKey
          },
          secondKey: {
            public: secondKey.getPublicKey().toString(),
            private: secondKey.secretKey
          }
        }
      ]))
    }

  }, [near, accountId])

  const passProps = {
    refreshAllowance: () => refreshAllowance(),
    signedAccountId,
    signedIn,
    connected,
    logOut,
    requestSignIn,
    widgets: Widgets,
  };

  const router = createBrowserRouter([
    {
      path: "/",
      element: <Root {...passProps} />,
      children: [
        {
          path: "detailPage/*",
          element: <DetailPage {...passProps} />,
        },
        {
          path: "marketplace",
          element: <Marketplace {...passProps} />,
        },
        {
          path: "dataStorage",
          element: <StoragePage {...passProps}/>,
        },
        {
          path: "listAccept",
          element: <ListAcceptPage {...passProps}/>,
        },
        {
          path: "",
          element: <DiscoverPage {...passProps} />,
        },
      ]
    },
  ]);

  return (
    <div className="App">
      <RouterProvider router={router} />
    </div>
  );
}

export default App;

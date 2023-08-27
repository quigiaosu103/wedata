import styled from "styled-components";
import React, { useCallback, useEffect, useState } from "react"
import { SignInButton } from "./SignInButton";
import { Link, useLocation } from "react-router-dom";
import { UserDropdown } from "./desktop/UserDropdown";
import logo from "../../../public/logo.svg"
import { callMethod, viewMethod } from "../../utils/method";
import { Widget, useAccount, useNear } from "near-social-vm";

const Header = styled.div`
    position: absolute;
    width: 100vw;
    height: 13vh;
    color: white;
    display: flex;
    padding: 20px 120px; 
    justify-content: space-between;
    align-items: center;
    z-index: 4
`;

const Logo = styled(Link)`
    font-size: 30px;
    background: linear-gradient(180deg, #fff 0%, rgba(255, 255, 255, 0) 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;

    :hover{
        text-decoration: none
    }
`;

const ButtonGroup = styled.div`
    display: flex,
    flex-direction: row
`;

const HeaderButton = styled(Link)`
    margin-right: 40px;
    cursor: pointer;
    text-decoration: none;
    color: white;

    :hover{
        color: white;
        text-decoration: none
    }
`;

const listBtn = [
  {
    name: "Discover",
    numberID: 0,
    href: "/"
  },
  {
    name: "Data Storage",
    numberID: 1,
    href: "/dataStorage"
  },
  {
    name: "Data Marketplace",
    numberID: 2,
    href: "/marketplace"
  },
  {
    name: "List Accept",
    numberID: 3,
    href: "/listAccept"
  },
];

const contractID = process.env.REACT_APP_CONTRACT_ID

function Navigation(props) {

    const location = useLocation()
    const near = useNear()
    const account = useAccount();
    const [isActive, setIsActive] = useState(true)
    const [amount, setAmount] = useState(0)

    useEffect(() => {

        if(!near) return
        console.log(account.accountId)

        viewMethod(near, {contractId: contractID, method: "get_balance_of", args: {account_id: account.accountId}}).then((result) => {
            if(result != -1){
                setAmount(result)
                setIsActive(true)
            }else{
                setIsActive(false)
            }
        })
    }, [near, props])

    const register = useCallback(() => {
        callMethod(near, {contractId: contractID, method: "register", args: {account_id: account.accountId}}).then(() => {})
    }, [near, props])

    return (
        <Header>
            {
                !isActive && <Widget props={{register}} src="tvh050423.testnet/widget/ModalRegister"/>
            }
            <Logo className="fw-bold" to={"/"}>
                <img src={logo} alt="..."/>
            </Logo>
            <ButtonGroup className="d-flex flex-row align-items-center">
                {listBtn.map((btn) => {
                    return (
                    <HeaderButton
                        key={btn.numberID}
                        to={btn.href}
                        id={btn.numberID}
                        className={location.pathname === btn.href && "text-decoration-underline"}
                    >
                        {btn.name}
                    </HeaderButton>
                    );
                })}
                {!props.signedIn && (
                    <SignInButton onSignIn={() => props.requestSignIn()} />
                )}
                {props.signedIn && (
                    <>
                        <UserDropdown {...props} amount={amount} />
                    </>
                )}
            </ButtonGroup>
        </Header>
    );
}

export default React.memo(Navigation)
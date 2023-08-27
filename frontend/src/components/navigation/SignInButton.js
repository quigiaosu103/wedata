import React from "react";
import styled from "styled-components";
import { Button } from "../common/buttons/Button";

const StyledButton = styled(Button)`
  background-color: #fff;
  color: black;
  margin-left: 70px
`;

export function SignInButton(props) {
  return (
    <StyledButton className="nav-sign-in-btn" onClick={props.onSignIn}>
      Sign In
    </StyledButton>
  );
}

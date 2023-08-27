import React, { useCallback } from "react";
import { Widget, useNear, useAccount } from "near-social-vm";
import styled from "styled-components";
import { NavLink } from "react-router-dom";

const StyledDropdown = styled.div`
  button,
  a {
    font-weight: var(--font-weight-medium);
  }
  .dropdown-toggle {
    display: flex;
    align-items: center;
    text-align: left;
    background-color: var(--slate-dark-5);
    border-radius: 50px;
    outline: none;
    border: 0;

    &:after {
      margin: 0 15px;
      border-top-color: var(--slate-dark-11);
    }

    img {
      border-radius: 50% !important;
    }

    .profile-info {
      margin: 5px 10px;
      line-height: normal;
      max-width: 140px;

      .profile-name,
      .profile-username {
        text-overflow: ellipsis;
        overflow: hidden;
      }

      .profile-name {
        color: var(--slate-dark-12);
      }
      .profile-username {
        color: var(--slate-dark-11);
      }
    }
  }

  ul {
    background-color: var(--slate-dark-5);
    width: 100%;

    li {
      padding: 0 6px;
    }

    button,
    a {
      color: var(--slate-dark-11);
      display: flex;
      align-items: center;
      border-radius: 8px;
      padding: 12px;

      :hover,
      :focus {
        text-decoration: none;
        background-color: var(--slate-dark-1);
        color: white;

        svg {
          path {
            stroke: white;
          }
        }
      }

      svg {
        margin-right: 7px;
        min-width: 24px;
        path {
          stroke: var(--slate-dark-9);
        }
      }
    }
  }
`;

export function UserDropdown(props) {
  const near = useNear();
  const account = useAccount();

  return (
    <>
      <StyledDropdown className="dropdown">
        <button
          className="dropdown-toggle"
          type="button"
          id="dropdownMenu2222"
          data-bs-toggle="dropdown"
          aria-expanded="false"
        >
          <Widget
            src={props.widgets.profileImage}
            props={{
              accountId: account.accountId,
              className: "d-inline-block",
              style: { width: "40px", height: "40px" },
            }}
          />
          <div className="profile-info">
            {props.widgets.profileName && (
              <div className="profile-name">
                <Widget src={props.widgets.profileName} />
              </div>
            )}
            <div className="profile-username">{account.accountId}</div>
          </div>
        </button>
        <ul
          className="dropdown-menu text-center"
          aria-labelledby="dropdownMenu2222"
          style={{ minWidth: "fit-content" }}
        >
          <li className="text-white d-flex flex-column align-items-center justify-content-center">
            <img
              style={{width: "2rem"}}
              src="https://lserxeomcpenkpxcdnan.supabase.co/storage/v1/object/public/image/370629065_688872989293745_9035513986791993486_n.png"
            />
            <p>{(props.amount).toLocaleString('fullwide', {useGrouping:false})} HTPC</p>
          </li>
          <li>
            <button
              className="dropdown-item d-flex justify-content-center"
              type="button"
              onClick={() => props.logOut()}
            >
              Sign Out
            </button>
          </li>
        </ul>
      </StyledDropdown>
    </>
  );
}

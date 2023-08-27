import { Outlet } from "react-router-dom";
import Navigation from "../components/navigation/Navigation";
import React from "react";

export default function Root(props) {
  return (
    <>
        <Navigation {...props}></Navigation>
        <Outlet />
    </>
  );
}
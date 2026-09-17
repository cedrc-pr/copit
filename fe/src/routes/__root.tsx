import { Outlet, createRootRoute } from "@tanstack/react-router";
import * as React from "react";
import { Header } from "../components/shared/Header";

export const Route = createRootRoute({
  component: RootComponent,
});

function RootComponent() {
  return (
    <React.Fragment>
      <Header />
      <Outlet />
    </React.Fragment>
  );
}

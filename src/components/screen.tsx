import { Box } from "@chakra-ui/react";
import Navbar from "./navbar.tsx";
import { PropsWithChildren } from "react";

interface Props extends PropsWithChildren {
  title: string;
}

export default function Screen({ title = "Screen name", children }: Props) {
  return (
    <Box height="100vh" width="100%" overflow="auto" p={3}>
      <Navbar text={title} />
      {children}
    </Box>
  );
}
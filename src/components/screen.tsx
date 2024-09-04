import { Box } from "@chakra-ui/react";
import { PropsWithChildren } from "react";

interface Props extends PropsWithChildren {}

export default function Screen({ children }: Props) {
  return (
    <Box height="100vh" width="100%" overflow="auto" p={3}>
      {children}
    </Box>
  );
}
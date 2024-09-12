import { Flex, Spinner } from "@chakra-ui/react";
import Screen from "./screen.tsx";
import Navbar from "./navbar.tsx";

export default function Loading() {
  return (
    <Screen>
      <Navbar />
      <Flex
        height="100%"
        flex={1}
        alignItems="center"
        justifyContent="center"
        flexDirection="column"
      >
        <Spinner />
      </Flex>
    </Screen>
  );
}
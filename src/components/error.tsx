import { Box, Heading, Text } from "@chakra-ui/react";
import Screen from "./screen.tsx";
import Navbar from "./navbar.tsx";

export default function Error(props: { error: string }) {
  return (
    <Screen>
      <Navbar />
      <Box
        height="100%"
        flex={1}
        display="flex"
        alignItems="center"
        justifyContent="center"
        flexDirection="column"
      >
        <Heading _light={{ color: "black" }} textAlign="center">
          App error
        </Heading>
        <Text _light={{ color: "gray.900" }} textAlign="center">
          {props.error}
        </Text>
      </Box>
    </Screen>
  );
}